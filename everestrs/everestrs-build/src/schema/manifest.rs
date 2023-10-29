use serde::Deserialize;
use std::collections::BTreeMap;
use crate::schema::interface::Variable;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub description: String,
    pub provides: BTreeMap<String, ProvidesEntry>,
    #[serde(default)]
    pub requires: BTreeMap<String, RequiresEntry>,
    pub metadata: Metadata,
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
