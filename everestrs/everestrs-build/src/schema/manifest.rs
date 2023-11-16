use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub description: String,
    pub metadata: Metadata,
    pub provides: BTreeMap<String, ProvidesEntry>,
    #[serde(default)]
    pub requires: BTreeMap<String, RequiresEntry>,

    // This is just here, so that we do not crash for deny_unknown_fields,
    // this is never used in Rust code.
    #[allow(dead_code)]
    enable_external_mqtt: bool,

    #[serde(default)]
    pub config: BTreeMap<String, ConfigEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvidesEntry {
    pub interface: String,
    pub description: String,
    #[serde(default)]
    pub config: BTreeMap<String, ConfigEntry>,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", deny_unknown_fields)]
pub enum ConfigEntry {
    Boolean {
        #[allow(dead_code)]
        description: Option<String>,
        default: Option<bool>,
    },
    String {
        #[allow(dead_code)]
        description: Option<String>,
        default: Option<String>,
        min_length: Option<i32>,
        max_length: Option<i32>,
    },
    Integer {
        #[allow(dead_code)]
        description: Option<String>,
        default: Option<i32>,
        minimum: Option<i32>,
        maximum: Option<i32>,
    },
    Number {
        #[allow(dead_code)]
        description: Option<String>,
        minimum: Option<f32>,
        default: Option<f32>,
        maximum: Option<f32>,
    },
}
