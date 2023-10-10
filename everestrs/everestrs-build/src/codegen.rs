use crate::schema::{
    interface::{Argument, Type, Variable},
    Interface, Manifest,
};
use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use minijinja::{Environment, UndefinedBehavior};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const SERVICE_JINJA: &str = include_str!("../jinja/service.jinja2");
const CLIENT_JINJA: &str = include_str!("../jinja/client.jinja2");
const MODULE_JINJA: &str = include_str!("../jinja/module.jinja2");

fn as_typename(arg: &Argument) -> &'static str {
    use Argument::*;
    use Type::*;
    match arg {
        Single(Null) => "()",
        Single(Boolean) => "bool",
        Single(String(_)) => "String",
        Single(Number(_)) => "f64",
        Single(Integer(_)) => "i32",
        Single(Array(_) | Object(_)) | Multiple(_) => "::serde_json::Value",
    }
}

#[derive(Debug, Clone, Serialize)]
struct ArgumentContext {
    name: String,
    description: Option<String>,
    data_type: &'static str,
}

impl ArgumentContext {
    pub fn from_schema(name: String, var: Variable) -> Self {
        ArgumentContext {
            name,
            description: var.description,
            data_type: as_typename(&var.arg),
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
    pub fn from_schema(name: String, cmd: crate::schema::interface::Command) -> Self {
        CommandContext {
            name,
            description: cmd.description,
            result: cmd
                .result
                .map(|arg| ArgumentContext::from_schema("return_value".to_string(), arg)),
            arguments: cmd
                .arguments
                .into_iter()
                .map(|(name, arg)| ArgumentContext::from_schema(name, arg))
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
    pub fn from_yaml(everest_core: &Path, name: &str) -> Result<Self> {
        let p = everest_core.join(format!("interfaces/{}.yaml", name));
        let blob = fs::read_to_string(&p).with_context(|| format!("Reading {p:?}"))?;
        let interface_yaml: Interface = serde_yaml::from_str(&blob)?;

        Ok(InterfaceContext {
            name: name.to_string(),
            description: interface_yaml.description,
            vars: interface_yaml
                .vars
                .into_iter()
                .map(|(name, var)| ArgumentContext::from_schema(name, var))
                .collect(),
            cmds: interface_yaml
                .cmds
                .into_iter()
                .map(|(name, cmd)| CommandContext::from_schema(name, cmd))
                .collect(),
        })
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
) -> Result<(Vec<InterfaceContext>, Vec<SlotContext>)> {
    let mut implementations = Vec::new();
    let mut unique_interfaces = Vec::new();
    let mut seen_interfaces = HashSet::new();
    for (implementation_id, interface) in entries {
        let interface_context = InterfaceContext::from_yaml(&everest_core, &interface)?;

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

    let (provided_interfaces, provides) = handle_implementations(
        &everest_core,
        manifest
            .provides
            .into_iter()
            .map(|(name, imp)| (name, imp.interface)),
    )?;
    let (required_interfaces, requires) = handle_implementations(
        &everest_core,
        manifest
            .requires
            .into_iter()
            .map(|(name, imp)| (name, imp.interface)),
    )?;

    let context = RenderContext {
        provided_interfaces,
        required_interfaces,
        provides,
        requires,
    };
    let tmpl = env.get_template("module").unwrap();
    let out = tmpl.render(context).unwrap();
    println!("{}", out);
    Ok(out)
}
