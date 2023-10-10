use crate::schema::{
    interface::{Type, Variable, Argument},
    Interface, Manifest,
};
use anyhow::{Context, Result};
use minijinja::{context, Environment};
use std::fs;
use std::path::{Path, PathBuf};

// NOCOM(#sirver): better understand the %- and -%
// NOCOM(#sirver): this does not work properly, I need to construct prior.
fn typename(arg: String) -> String {
    println!("#sirver arg: {:#?}", arg);
    match &arg as &str {
        "string" => "String",
        "boolean" => "bool",
        "" => "serde_json::Value",
        _ => "",
    }.to_string()
}

fn emit_interface_service_trait(interface: &Interface) -> Result<()> {
    let mut env = Environment::new();
    let blob = fs::read_to_string("everestrs-build/jinja/provide_traits.jinja2")?;
    env.add_filter("typename", typename);
    env.add_template("provide_traits", &blob)?;
    let tmpl = env.get_template("provide_traits").unwrap();

    println!("{}", tmpl.render(interface).unwrap());

    Ok(())
}

pub fn emit(module_name: String, manifest_path: PathBuf, everest_core: PathBuf) -> Result<String> {
    let blob = fs::read_to_string(&manifest_path).context("reading manifest file")?;
    let manifest: Manifest = serde_yaml::from_str(&blob)?;

    for (implementation_id, implementation) in &manifest.provides {
        let p = everest_core.join(format!("interfaces/{}.yaml", implementation.interface));
        let blob = fs::read_to_string(&p).with_context(|| format!("Reading {p:?}"))?;
        let mut interface_yaml: Interface = serde_yaml::from_str(&blob)?;

        interface_yaml.name = Some(implementation.interface.clone());
        emit_interface_service_trait(&interface_yaml)?;
    }

    todo!();
}
