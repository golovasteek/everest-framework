// This duplicates schema entries from everestrs-build, but this one only uses what is needed for
// code gen, while the other one only uses what is required for making everest-rs run.
//
// This code here specfically is meant to be deleted as soon at the python code gen is ready to
// accept the Rust generated code.

pub mod interface;
pub mod manifest;

pub use interface::Interface;
pub use manifest::Manifest;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataTypes {
    pub description: String,
    pub types: BTreeMap<String, interface::Variable>,
}
