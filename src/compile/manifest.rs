use crate::error::{Error, ErrorKind, Result};
use crate::utilities::{path_exists, read_file_string};
use failure::ResultExt;
use std::path::Path;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextureEntry {
    pub name: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureManifest {
    pub entries: Vec<TextureEntry>,
}

impl TextureManifest {
    pub fn validate(&self, base_dir: &Path) -> Result<()> {
        for entry in &self.entries {
            let texture_file = base_dir.join(&entry.file);
            if !path_exists(&texture_file) {
                return Err(Error::config(format!(
                    "file {:?} does not exist",
                    texture_file
                )));
            }
        }
        Ok(())
    }
}

pub fn load_manifest(base_dir: &Path, path: &Path) -> Result<TextureManifest> {
    let manifest_toml = read_file_string(&path).with_context(|_| ErrorKind::path(path))?;
    parse_manifest(base_dir, &manifest_toml)
}

pub fn parse_manifest(base_dir: &Path, manifest_toml: &str) -> Result<TextureManifest> {
    let manifest: TextureManifest = toml::from_str(&manifest_toml)?;
    manifest.validate(&base_dir)?;
    Ok(manifest)
}
