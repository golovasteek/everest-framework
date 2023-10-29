use crate::schema::{
    interface::{Argument, ObjectOptions, StringOptions, Type, Variable},
    DataTypes, Interface, Manifest,
};
use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use minijinja::{Environment, UndefinedBehavior};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

const SERVICE_JINJA: &str = include_str!("../jinja/service.jinja2");
const CLIENT_JINJA: &str = include_str!("../jinja/client.jinja2");
const MODULE_JINJA: &str = include_str!("../jinja/module.jinja2");
const TYPE_MODULE: &str = include_str!("../jinja/type_module.jinja2");

// We just pull out of ObjectOptions what we really need for codegen.
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct TypeRef {
    /// The same as the file name under everest-core/types.
    module_path: Vec<String>,
    type_name: String,
}

impl TypeRef {
    pub fn from_object(args: &ObjectOptions) -> Self {
        assert!(args.object_reference.is_some());
        assert!(
            args.properties.is_empty(),
            "Found an object with $ref, but also with properties. Cannot handle that case."
        );
        Self::from_reference(&args.object_reference.as_ref().unwrap())
    }

    pub fn from_string(args: &StringOptions) -> Self {
        assert!(args.object_reference.is_some());
        Self::from_reference(&args.object_reference.as_ref().unwrap())
    }

    fn from_reference(r: &str) -> Self {
        let mut it = r.trim_start_matches('/').split("#/");
        // NOCOM(#sirver): error handling
        let module_name = it.next().unwrap().to_string();
        let module_path = module_name.split('/').map(|s| s.to_string()).collect();
        let type_name = it.next().unwrap().to_string();
        Self {
            module_path,
            type_name,
        }
    }

    pub fn module_name(&self) -> String {
        format!("::crate::generated::types::{}", self.module_path.join("::"),)
    }

    pub fn absolute_type_path(&self) -> String {
        format!("{}::{}", self.module_name(), self.type_name)
    }
}

fn as_typename(arg: &Argument, type_refs: &mut BTreeSet<TypeRef>) -> String {
    use Argument::*;
    use Type::*;
    match arg {
        Single(Null) => "()".to_string(),
        Single(Boolean) => "bool".to_string(),
        Single(String(args)) => {
            if args.object_reference.is_none() {
                "String".to_string()
            } else {
                let t = TypeRef::from_string(args);
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
                let t = TypeRef::from_object(args);
                let name = t.absolute_type_path();
                type_refs.insert(t);
                name
            }
        }
        Single(Array(args)) => match args.items {
            None => "Vec<::serde_json::Value>".to_string(),
            Some(ref v) => {
                let item_type = as_typename(&v.arg, type_refs);
                format!("Vec<{item_type}>")
            }
        },
        Multiple(_) => "::serde_json::Value".to_string(),
    }
}

#[derive(Debug, Clone, Serialize)]
struct ArgumentContext {
    name: String,
    description: Option<String>,
    data_type: String,
}

impl ArgumentContext {
    pub fn from_schema(name: String, var: Variable, type_refs: &mut BTreeSet<TypeRef>) -> Self {
        ArgumentContext {
            name,
            description: var.description,
            data_type: as_typename(&var.arg, type_refs),
        }
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
        cmd: crate::schema::interface::Command,
        type_refs: &mut BTreeSet<TypeRef>,
    ) -> Self {
        CommandContext {
            name,
            description: cmd.description,
            result: cmd.result.map(|arg| {
                ArgumentContext::from_schema("return_value".to_string(), arg, type_refs)
            }),
            arguments: cmd
                .arguments
                .into_iter()
                .map(|(name, arg)| ArgumentContext::from_schema(name, arg, type_refs))
                .collect(),
        }
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
        everest_core: &Path,
        name: &str,
        type_refs: &mut BTreeSet<TypeRef>,
    ) -> Result<Self> {
        let p = everest_core.join(format!("interfaces/{}.yaml", name));
        let blob = fs::read_to_string(&p).with_context(|| format!("Reading {p:?}"))?;
        let interface_yaml: Interface = serde_yaml::from_str(&blob)?;

        Ok(InterfaceContext {
            name: name.to_string(),
            description: interface_yaml.description,
            vars: interface_yaml
                .vars
                .into_iter()
                .map(|(name, var)| ArgumentContext::from_schema(name, var, type_refs))
                .collect(),
            cmds: interface_yaml
                .cmds
                .into_iter()
                .map(|(name, cmd)| CommandContext::from_schema(name, cmd, type_refs))
                .collect(),
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
    everest_core: &Path,
    type_refs: &mut BTreeSet<TypeRef>,
) -> Result<TypeContext> {
    use Argument::*;
    use Type::*;

    let p = everest_core.join(format!("types/{}.yaml", r.module_path.join("/")));
    let blob = fs::read_to_string(&p).with_context(|| format!("Reading {p:?}"))?;
    let data_types_yaml: DataTypes = serde_yaml::from_str(&blob)?;

    // NOCOM(#sirver): error checking
    let type_descr = data_types_yaml.types.get(&r.type_name).unwrap();
    match &type_descr.arg {
        Single(Object(args)) => {
            let mut properties = Vec::new();
            for (name, var) in &args.properties {
                let data_type = {
                    let d = as_typename(&var.arg, type_refs);
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
struct RenderContext {
    /// The interfaces the user will need to fill in.
    provided_interfaces: Vec<InterfaceContext>,
    /// The interfaces we are requiring.
    required_interfaces: Vec<InterfaceContext>,
    provides: Vec<SlotContext>,
    requires: Vec<SlotContext>,
    type_module: TypeModuleContext,
}

// NOCOM(#sirver): better understand the %- and -%
fn title_case(arg: String) -> String {
    arg.to_case(Case::Pascal)
}

fn snake_case(arg: String) -> String {
    arg.to_case(Case::Snake)
}

fn handle_implementations(
    everest_core: &Path,
    entries: impl Iterator<Item = (String, String)>,
    type_refs: &mut BTreeSet<TypeRef>,
) -> Result<(Vec<InterfaceContext>, Vec<SlotContext>)> {
    let mut implementations = Vec::new();
    let mut unique_interfaces = Vec::new();
    let mut seen_interfaces = HashSet::new();
    for (implementation_id, interface) in entries {
        let interface_context = InterfaceContext::from_yaml(&everest_core, &interface, type_refs)?;

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

pub fn emit(manifest_path: PathBuf, everest_core: PathBuf) -> Result<String> {
    let blob = fs::read_to_string(&manifest_path).context("reading manifest file")?;
    let manifest: Manifest = serde_yaml::from_str(&blob)?;

    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    env.add_filter("title", title_case);
    env.add_filter("snake", snake_case);
    env.add_template("module", MODULE_JINJA)?;
    env.add_template("service", SERVICE_JINJA)?;
    env.add_template("client", CLIENT_JINJA)?;
    env.add_template("type_module", TYPE_MODULE)?;

    let mut type_refs = BTreeSet::new();
    let (provided_interfaces, provides) = handle_implementations(
        &everest_core,
        manifest
            .provides
            .into_iter()
            .map(|(name, imp)| (name, imp.interface)),
        &mut type_refs,
    )?;
    let (required_interfaces, requires) = handle_implementations(
        &everest_core,
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
                // NOCOM(#sirver): this could use references
                module = module.children.entry(p.clone()).or_default();
            }
            // NOCOM(#sirver): this reparses the same yamls over and over again.
            // instead, I should parse the whole subtree and build a hashmap from ref to
            // description
            match type_context_from_ref(t, &everest_core, &mut new)? {
                TypeContext::Object(item) => module.objects.push(item),
                TypeContext::Enum(item) => module.enums.push(item),
            }
            done.insert(t.clone());
        }
        println!("#sirver new: {:#?}", new);
        type_refs.extend(new.into_iter());
    }

    println!("#sirver type_module_root: {:#?}", type_module_root);
    let context = RenderContext {
        provided_interfaces,
        required_interfaces,
        provides,
        requires,
        type_module: type_module_root,
    };
    let tmpl = env.get_template("module").unwrap();
    Ok(tmpl.render(context).unwrap())
}
