use crate::schema::{
    interface::{Argument, ObjectOptions, StringOptions, Type, Variable},
    manifest::{ConfigEntry, ConfigEnum},
    DataTypes, Interface, Manifest,
};
use anyhow::{anyhow, bail, Context, Result};
use convert_case::{Case, Casing};
use minijinja::{Environment, UndefinedBehavior};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

// We include the JINJA templates into the binary. This has the disadvantage
// that every change to the templates requires a recompilation, but the
// advantage that the codegen library/binary is truly standalone and needs
// nothing shipped with it to work.
const CLIENT_JINJA: &str = include_str!("../jinja/client.jinja2");
const CONFIG_JINJA: &str = include_str!("../jinja/config.jinja2");
const MODULE_JINJA: &str = include_str!("../jinja/module.jinja2");
const SERVICE_JINJA: &str = include_str!("../jinja/service.jinja2");
const TYPES_JINJA: &str = include_str!("../jinja/types.jinja2");

fn parse_yaml<T: DeserializeOwned>(everest_core: &Path, subdir: &str, name: &str) -> Result<T> {
    let p = everest_core.join(format!("{subdir}/{name}.yaml"));
    let blob = fs::read_to_string(&p).with_context(|| format!("Reading {p:?}"))?;
    serde_yaml::from_str(&blob).with_context(|| format!("Parsing {p:?}"))
}

fn lazy_load<'a, T: DeserializeOwned>(
    storage: &'a mut HashMap<String, T>,
    everest_core: &Vec<PathBuf>,
    prefix: &str,
    postfix: &str,
) -> Result<&'a T> {
    if storage.contains_key(postfix) {
        return Ok(storage.get(postfix).unwrap());
    }

    let mut matches = everest_core
        .iter()
        .filter_map(|core| match parse_yaml(core, prefix, postfix) {
            Err(_) => None,
            Ok(res) => Some(res),
        })
        .collect::<Vec<_>>();

    assert!(
        matches.len() == 1,
        "The name {postfix} must be defined exactly once"
    );

    storage.insert(postfix.to_string(), matches.pop().unwrap());
    Ok(storage.get(postfix).unwrap())
}

/// A lazy loader for YAML files. If the same file is requested twice, it will
/// not be re-parsed again.
#[derive(Default, Debug)]
struct YamlRepo {
    // This might be also a HashMap of "namespaces" and paths.
    everest_core: Vec<PathBuf>,
    interfaces: HashMap<String, Interface>,
    data_types: HashMap<String, DataTypes>,
}

impl YamlRepo {
    pub fn new(everest_core: Vec<PathBuf>) -> Self {
        Self {
            everest_core,
            ..Default::default()
        }
    }

    pub fn get_interface<'a>(&'a mut self, name: &str) -> Result<&'a Interface> {
        lazy_load(&mut self.interfaces, &self.everest_core, "interfaces", name)
    }

    pub fn get_data_types<'a>(&'a mut self, name: &str) -> Result<&'a DataTypes> {
        lazy_load(&mut self.data_types, &self.everest_core, "types", name)
    }
}

// We just pull out of ObjectOptions what we really need for codegen.
#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct TypeRef {
    /// The same as the file name under everest-core/types.
    module_path: Vec<String>,
    type_name: String,
}

impl TypeRef {
    fn from_object(args: &ObjectOptions) -> Result<Self> {
        assert!(args.object_reference.is_some());
        assert!(
            args.properties.is_empty(),
            "Found an object with $ref, but also with properties. Cannot handle that case."
        );
        Self::from_reference(args.object_reference.as_ref().unwrap())
    }

    fn from_string(args: &StringOptions) -> Result<Self> {
        assert!(args.object_reference.is_some());
        Self::from_reference(args.object_reference.as_ref().unwrap())
    }

    fn from_reference(r: &str) -> Result<Self> {
        let parts: Vec<_> = r.trim_start_matches('/').split("#/").collect();
        if parts.len() != 2 {
            bail!("Unexpected type reference: {}", r);
        }
        let module_name = parts[0].to_string();
        let module_path = module_name.split('/').map(|s| s.to_string()).collect();
        let type_name = parts[1].to_string();
        Ok(Self {
            module_path,
            type_name,
        })
    }

    pub fn module_name(&self) -> String {
        format!("crate::generated::types::{}", self.module_path.join("::"),)
    }

    pub fn absolute_type_path(&self) -> String {
        format!("{}::{}", self.module_name(), self.type_name)
    }
}

impl std::fmt::Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TypeRef /{}#/{}",
            self.module_path.join("/"),
            self.type_name
        )
    }
}

fn as_typename(arg: &Argument, type_refs: &mut BTreeSet<TypeRef>) -> Result<String> {
    use Argument::*;
    use Type::*;
    Ok(match arg {
        Single(Null) => "()".to_string(),
        Single(Boolean) => "bool".to_string(),
        Single(String(args)) => {
            if args.object_reference.is_none() {
                "String".to_string()
            } else {
                let t = TypeRef::from_string(args)?;
                let name = t.absolute_type_path();
                type_refs.insert(t);
                name
            }
        }
        Single(Number(_)) => "f64".to_string(),
        Single(Integer(_)) => "i64".to_string(),
        Single(Object(args)) => {
            if args.object_reference.is_none() {
                "::serde_json::Value".to_string()
            } else {
                let t = TypeRef::from_object(args)?;
                let name = t.absolute_type_path();
                type_refs.insert(t);
                name
            }
        }
        Single(Array(args)) => match args.items {
            None => "Vec<::serde_json::Value>".to_string(),
            Some(ref v) => {
                let item_type = as_typename(&v.arg, type_refs)?;
                format!("Vec<{item_type}>")
            }
        },
        Multiple(_) => "::serde_json::Value".to_string(),
    })
}

#[derive(Debug, Clone, Serialize)]
struct ArgumentContext {
    name: String,
    description: Option<String>,
    data_type: String,
}

impl ArgumentContext {
    pub fn from_schema(
        name: String,
        var: &Variable,
        type_refs: &mut BTreeSet<TypeRef>,
    ) -> Result<Self> {
        Ok(ArgumentContext {
            name,
            description: var.description.clone(),
            data_type: as_typename(&var.arg, type_refs)?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
struct CommandContext {
    name: String,
    description: String,
    result: Option<ArgumentContext>,
    arguments: Vec<ArgumentContext>,
}

impl CommandContext {
    pub fn from_schema(
        name: String,
        cmd: &crate::schema::interface::Command,
        type_refs: &mut BTreeSet<TypeRef>,
    ) -> Result<Self> {
        let mut arguments = Vec::new();
        for (name, arg) in &cmd.arguments {
            arguments.push(ArgumentContext::from_schema(name.clone(), arg, type_refs)?);
        }
        Ok(CommandContext {
            name,
            description: cmd.description.clone(),
            result: match &cmd.result {
                None => None,
                Some(arg) => Some(ArgumentContext::from_schema(
                    "return_value".to_string(),
                    arg,
                    type_refs,
                )?),
            },
            arguments,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
struct InterfaceContext {
    name: String,
    description: String,
    cmds: Vec<CommandContext>,
    vars: Vec<ArgumentContext>,
}

impl InterfaceContext {
    pub fn from_yaml(
        yaml_repo: &mut YamlRepo,
        name: &str,
        type_refs: &mut BTreeSet<TypeRef>,
    ) -> Result<Self> {
        let interface_yaml = yaml_repo.get_interface(name)?;
        let mut vars = Vec::new();
        for (name, var) in &interface_yaml.vars {
            vars.push(ArgumentContext::from_schema(name.clone(), var, type_refs)?);
        }
        let mut cmds = Vec::new();
        for (name, cmd) in &interface_yaml.cmds {
            cmds.push(CommandContext::from_schema(name.clone(), cmd, type_refs)?);
        }
        Ok(InterfaceContext {
            name: name.to_string(),
            description: interface_yaml.description.clone(),
            vars,
            cmds,
        })
    }
}

#[derive(Debug, Clone, Serialize, Default)]
struct TypeModuleContext {
    children: BTreeMap<String, TypeModuleContext>,
    objects: Vec<ObjectTypeContext>,
    enums: Vec<EnumTypeContext>,
}

#[derive(Debug, Clone, Serialize)]
struct ObjectTypeContext {
    name: String,
    properties: Vec<ArgumentContext>,
}

#[derive(Debug, Clone, Serialize)]
struct EnumTypeContext {
    name: String,
    items: Vec<String>,
}

#[derive(Debug, Clone)]
enum TypeContext {
    Object(ObjectTypeContext),
    Enum(EnumTypeContext),
}

fn type_context_from_ref(
    r: &TypeRef,
    yaml_repo: &mut YamlRepo,
    type_refs: &mut BTreeSet<TypeRef>,
) -> Result<TypeContext> {
    use Argument::*;
    use Type::*;

    let data_types_yaml = yaml_repo.get_data_types(&r.module_path.join("/"))?;

    let type_descr = data_types_yaml
        .types
        .get(&r.type_name)
        .ok_or_else(|| anyhow!("Unable to find data type {:?}. Is it defined?", r))?;
    match &type_descr.arg {
        Single(Object(args)) => {
            let mut properties = Vec::new();
            for (name, var) in &args.properties {
                let data_type = {
                    let d = as_typename(&var.arg, type_refs)?;
                    if !args.required.contains(name) {
                        format!("Option<{}>", d)
                    } else {
                        d
                    }
                };
                properties.push(ArgumentContext {
                    name: name.clone(),
                    description: var.description.clone(),
                    data_type,
                });
            }
            Ok(TypeContext::Object(ObjectTypeContext {
                name: r.type_name.clone(),
                properties,
            }))
        }
        Single(String(args)) => {
            assert!(
                args.enum_items.is_some(),
                "Expected a named string type to be an enum, but {} was not.",
                r.type_name
            );

            Ok(TypeContext::Enum(EnumTypeContext {
                name: r.type_name.clone(),
                items: args.enum_items.clone().unwrap(),
            }))
        }
        other => unreachable!("Does not support $ref for {other:?}"),
    }
}

#[derive(Debug, Clone, Serialize)]
struct SlotContext {
    implementation_id: String,
    interface: String,
}

#[derive(Debug, Clone, Serialize)]
struct ConfigContext {
    name: String,
    config: Vec<ArgumentContext>,
}

#[derive(Debug, Clone, Serialize)]
struct RenderContext {
    /// The interfaces the user will need to fill in.
    provided_interfaces: Vec<InterfaceContext>,
    /// The interfaces we are requiring.
    required_interfaces: Vec<InterfaceContext>,
    provides: Vec<SlotContext>,
    requires: Vec<SlotContext>,
    types: TypeModuleContext,
    module_config: Vec<ArgumentContext>,
    provided_config: Vec<ConfigContext>,
}

fn title_case(arg: String) -> String {
    arg.to_case(Case::Pascal)
}

fn snake_case(arg: String) -> String {
    arg.to_case(Case::Snake)
}

fn handle_implementations(
    yaml_repo: &mut YamlRepo,
    entries: impl Iterator<Item = (String, String)>,
    type_refs: &mut BTreeSet<TypeRef>,
) -> Result<(Vec<InterfaceContext>, Vec<SlotContext>)> {
    let mut implementations = Vec::new();
    let mut unique_interfaces = Vec::new();
    let mut seen_interfaces = HashSet::new();
    for (implementation_id, interface) in entries {
        let interface_context = InterfaceContext::from_yaml(yaml_repo, &interface, type_refs)?;

        if !seen_interfaces.contains(&interface) {
            unique_interfaces.push(interface_context);
            seen_interfaces.insert(interface.clone());
        }

        implementations.push(SlotContext {
            implementation_id,
            interface,
        });
    }
    Ok((unique_interfaces, implementations))
}

/// Converts the config data read from yaml and generates the context for Jinja.
///
/// The config data contains the config name (key) and the config data (value).
/// We use the value to derive the type and the (optional) description.
fn emit_config(config: BTreeMap<String, ConfigEntry>) -> Vec<ArgumentContext> {
    config
        .into_iter()
        .map(|(k, v)| match v.value {
            ConfigEnum::Boolean { default: _ } => ArgumentContext {
                name: k,
                description: v.description,
                data_type: "bool".to_string(),
            },
            ConfigEnum::Integer {
                default: _,
                minimum: _,
                maximum: _,
            } => ArgumentContext {
                name: k,
                description: v.description,
                data_type: "i64".to_string(),
            },
            ConfigEnum::Number {
                default: _,
                minimum: _,
                maximum: _,
            } => ArgumentContext {
                name: k,
                description: v.description,
                data_type: "f64".to_string(),
            },
            ConfigEnum::String {
                default: _,
                max_length: _,
                min_length: _,
            } => ArgumentContext {
                name: k,
                description: v.description,
                data_type: "String".to_string(),
            },
        })
        .collect::<Vec<_>>()
}

pub fn emit(manifest_path: PathBuf, everest_core: Vec<PathBuf>) -> Result<String> {
    let mut yaml_repo = YamlRepo::new(everest_core);
    let blob = fs::read_to_string(&manifest_path).context("While reading manifest file")?;
    let manifest: Manifest = serde_yaml::from_str(&blob).context("While parsing manifest")?;

    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    env.add_filter("title", title_case);
    env.add_filter("snake", snake_case);
    env.add_template("client", CLIENT_JINJA)?;
    env.add_template("config", CONFIG_JINJA)?;
    env.add_template("module", MODULE_JINJA)?;
    env.add_template("service", SERVICE_JINJA)?;
    env.add_template("types", TYPES_JINJA)?;

    let provided_config = manifest
        .provides
        .iter()
        .filter(|(_, data)| !data.config.is_empty())
        .map(|(name, data)| ConfigContext {
            name: name.clone(),
            config: emit_config(data.config.clone()),
        })
        .collect::<Vec<_>>();

    let mut type_refs = BTreeSet::new();
    let (provided_interfaces, provides) = handle_implementations(
        &mut yaml_repo,
        manifest
            .provides
            .into_iter()
            .map(|(name, imp)| (name, imp.interface)),
        &mut type_refs,
    )?;
    let (required_interfaces, requires) = handle_implementations(
        &mut yaml_repo,
        manifest
            .requires
            .into_iter()
            .map(|(name, imp)| (name, imp.interface)),
        &mut type_refs,
    )?;

    let mut type_module_root = TypeModuleContext::default();

    let mut done: BTreeSet<TypeRef> = BTreeSet::new();
    while done.len() != type_refs.len() {
        let mut new = BTreeSet::new();
        for t in &type_refs {
            if done.contains(t) {
                continue;
            }
            let mut module = &mut type_module_root;
            for p in &t.module_path {
                module = module.children.entry(p.clone()).or_default();
            }
            match type_context_from_ref(t, &mut yaml_repo, &mut new)? {
                TypeContext::Object(item) => module.objects.push(item),
                TypeContext::Enum(item) => module.enums.push(item),
            }
            done.insert(t.clone());
        }
        type_refs.extend(new.into_iter());
    }

    let module_config = emit_config(manifest.config);

    let context = RenderContext {
        provided_interfaces,
        required_interfaces,
        provides,
        requires,
        types: type_module_root,
        module_config,
        provided_config,
    };
    let tmpl = env.get_template("module").unwrap();
    Ok(tmpl.render(context).unwrap())
}
