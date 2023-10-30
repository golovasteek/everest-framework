use crate::schema::interface::Variable;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub description: String,
    pub provides: BTreeMap<String, ProvidesEntry>,
    #[serde(default)]
    pub requires: BTreeMap<String, RequiresEntry>,
    pub metadata: Metadata,

    // This is just here, so that we do not crash for deny_unknown_fields, this is never used in
    // Rust code.
    #[allow(dead_code)]
    enable_external_mqtt: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct YamlData {
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvidesEntry {
    pub interface: String,
    pub description: String,
    #[serde(default)]
    pub config: BTreeMap<String, Variable>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiresEntry {
    pub interface: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub license: String,
    pub authors: Vec<String>,
}
