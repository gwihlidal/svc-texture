use crate::error::{Error, ErrorKind, Result};
use crate::utilities::{path_exists, read_file_string};
use failure::ResultExt;
use std::path::Path;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextureEntry {
    pub name: String,
    pub file: String,
    pub format: String,
    pub mips: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureManifest {
    pub entries: Vec<TextureEntry>,
}

impl TextureManifest {
    pub fn validate(&self) -> Result<()> {
        for entry in &self.entries {
            if !path_exists(&entry.file) {
                return Err(Error::config(format!(
                    "file {:?} does not exist",
                    entry.file
                )));
            }
        }
        Ok(())
    }
}

pub fn load_manifest(path: &Path) -> Result<TextureManifest> {
    let manifest_toml = read_file_string(&path).with_context(|_| ErrorKind::path(path))?;
    parse_manifest(&manifest_toml)
}

pub fn parse_manifest(manifest_toml: &str) -> Result<TextureManifest> {
    let manifest: TextureManifest = toml::from_str(&manifest_toml)?;
    manifest.validate()?;
    Ok(manifest)
}
