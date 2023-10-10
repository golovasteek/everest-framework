mod codegen;
mod schema;

use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Builder {
    everest_core: PathBuf,
    // TODO(hrapp): This is almost always the same anyways.
    manifest_path: PathBuf,
    out_dir: Option<PathBuf>,
}

impl Builder {
    pub fn new(
        manifest_path: impl Into<PathBuf>,
        everest_core: impl Into<PathBuf>,
    ) -> Self {
        Self {
            everest_core: everest_core.into(),
            manifest_path: manifest_path.into(),
            ..Builder::default()
        }
    }

    pub fn out_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(path.into());
        self
    }

    pub fn generate(self) -> Result<()> {
        let path = self
            .out_dir
            .unwrap_or_else(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()))
            .join("generated.rs");

        let out = codegen::emit(self.manifest_path, self.everest_core)?;

        let mut f = std::fs::File::create(path).context("Could not generate the output file.")?;
        f.write_all(out.as_bytes())?;
        Ok(())
    }
}
