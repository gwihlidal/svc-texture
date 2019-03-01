use crate::error::{Error, ErrorKind, Result};
use crate::process::generated::service::texture::schema;
use crate::utilities::{path_exists, read_file_string};
use failure::ResultExt;
use std::path::Path;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextureEntry {
    pub name: String,
    pub file: String,
    pub format: String,
    pub mips: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureManifest {
    pub entries: Vec<TextureEntry>,
}

impl TextureManifest {
    pub fn validate(&self, base_dir: &Path) -> Result<()> {
        for entry in &self.entries {
            let entry_file = base_dir.join(&entry.file);
            if !path_exists(&entry_file) {
                return Err(Error::config(format!(
                    "file {:?} does not exist",
                    entry.file
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

pub fn parse_output_format(format: &str) -> schema::TextureFormat {
    match format {
        "bc1" => schema::TextureFormat::BC1_UNORM,
        "bc3" => schema::TextureFormat::BC3_UNORM,
        "bc6h" => schema::TextureFormat::BC6S_FLOAT,
        "bc7" => schema::TextureFormat::BC7_UNORM,
        _ => unimplemented!(),
    }
}
