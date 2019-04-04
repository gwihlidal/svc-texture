#![allow(unused_imports)]

use crate::process::schema;
use filebuffer::FileBuffer;
use std::env;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::{hash::Hasher, io};
use uuid::Uuid;

#[inline(always)]
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn compute_identity(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    use smush::{encode, Encoding, Quality};

    // create a Sha256 object
    let mut hasher = Sha256::default();

    // write input data
    hasher.input(data);

    // read hash digest and consume hasher
    let data = hasher.result().to_vec();
    let data_b58 = encode(&data, Encoding::Base58, Quality::Default).unwrap();
    String::from_utf8(data_b58).unwrap()
}

struct HashWriter<T: Hasher>(T);
impl<T: Hasher> io::Write for HashWriter<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write(buf).map(|_| ())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// This looks first for linker-inserted build ID / binary UUIDs (i.e.
/// `.note.gnu.build-id` on Linux; `LC_UUID` in Mach-O; etc), falling back to
/// hashing the whole binary.
lazy_static! {
    pub static ref BUILD_ID: String = {
        let mut hasher = twox_hash::XxHash::with_seed(0);

        // let a = |x:()|x;
        // let b = |x:u8|x;
        // hasher.write_u64(type_id(&a));
        // hasher.write_u64(type_id(&b));

        // LC_UUID https://opensource.apple.com/source/libsecurity_codesigning/libsecurity_codesigning-55037.6/lib/machorep.cpp https://stackoverflow.com/questions/10119700/how-to-get-mach-o-uuid-of-a-running-process
        // .note.gnu.build-id https://github.com/golang/go/issues/21564 https://github.com/golang/go/blob/178307c3a72a9da3d731fecf354630761d6b246c/src/cmd/go/internal/buildid/buildid.go
        let file = exe().unwrap();
        let _ = io::copy(&mut &file, &mut HashWriter(&mut hasher)).unwrap();

        let mut bytes = [0; 16];
        <byteorder::NativeEndian as byteorder::ByteOrder>::write_u64(&mut bytes, hasher.finish());
        compute_identity(&bytes)
        //Uuid::from_random_bytes(bytes)
    };
}

pub fn compute_file_identity<P: AsRef<Path>>(path: P) -> io::Result<String> {
    use sha2::{Digest, Sha256};
    use smush::{encode, Encoding, Quality};

    let fbuffer = FileBuffer::open(&path)?;

    // create a Sha256 object
    let mut hasher = Sha256::default();

    // write input data
    hasher.input(&fbuffer);

    // read hash digest and consume hasher
    let data = hasher.result().to_vec();
    let data_b58 = encode(&data, Encoding::Base58, Quality::Default).unwrap();
    Ok(String::from_utf8(data_b58).unwrap())
}

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    std::fs::metadata(path.as_ref()).is_ok()
}

pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let meta = file.metadata()?;
    let size = meta.len() as usize;
    let mut data = vec![0; size];
    file.read_exact(&mut data)?;
    Ok(data)
}

pub fn read_file_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path.as_ref())?;
    let mut text = String::new();
    if let Ok(meta) = file.metadata() {
        text.reserve(meta.len() as usize); // Safe to truncate, since it's only a suggestion
    }
    file.read_to_string(&mut text)?;
    //let text = String::from_iter(normalized(text.chars()));
    Ok(text)
}

pub fn string_from_path(path: &Path) -> Option<String> {
    let path_os_str = path.as_os_str();
    if let Some(path_str) = path_os_str.to_str() {
        Some(path_str.to_string())
    } else {
        None
    }
}

pub struct TempDir {
    pub uuid: Uuid,
    pub path: PathBuf,
}

impl TempDir {
    pub fn new(temp_path: &Path) -> Self {
        let dir_uuid = Uuid::new_v4();
        let dir_path = temp_path.join(dir_uuid.to_string());
        Self {
            uuid: dir_uuid,
            path: dir_path,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn create(&self) -> io::Result<()> {
        std::fs::create_dir_all(&self.path)
    }

    pub fn as_str(&self) -> String {
        string_from_path(&self.path).unwrap_or_else(|| "PATH_ERROR".to_string())
    }

    pub fn exists(&self) -> bool {
        path_exists(&self.path)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.exists() {
            match std::fs::remove_dir_all(&self.path) {
                Ok(_) => {
                    // Temp dir was deleted!
                }
                Err(err) => {
                    panic!(
                        "Error occurred trying to delete temp dir! path: {:?} - {:?}",
                        self.path, err
                    );
                }
            }
        }
    }
}

pub struct TempFile {
    pub uuid: Uuid,
    pub path: PathBuf,
}

impl TempFile {
    pub fn new(temp_path: &Path) -> Self {
        let file_uuid = Uuid::new_v4();
        let file_path = temp_path.join(file_uuid.to_string());
        Self {
            uuid: file_uuid,
            path: file_path,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn as_str(&self) -> String {
        string_from_path(&self.path).unwrap_or_else(|| "PATH_ERROR".to_string())
    }

    pub fn exists(&self) -> bool {
        path_exists(&self.path)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if self.exists() {
            match std::fs::remove_file(&self.path) {
                Ok(_) => {
                    // Temp file was deleted!
                }
                Err(err) => {
                    panic!(
                        "Error occurred trying to delete temp file! path: {:?} - {:?}",
                        self.path, err
                    );
                }
            }
        }
    }
}

pub fn exe() -> io::Result<std::fs::File> {
    exe_path().and_then(std::fs::File::open)
}

/// Returns the path of the currently running executable. On Linux this is `/proc/self/exe`.
// https://stackoverflow.com/questions/1023306/finding-current-executables-path-without-proc-self-exe
pub fn exe_path() -> io::Result<std::path::PathBuf> {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        Ok(std::path::PathBuf::from("/proc/self/exe"))
    }
    #[cfg(any(target_os = "dragonfly"))]
    {
        Ok(std::path::PathBuf::from("/proc/curproc/file"))
    }
    #[cfg(any(target_os = "netbsd"))]
    {
        Ok(std::path::PathBuf::from("/proc/curproc/exe"))
    }
    #[cfg(any(target_os = "solaris"))]
    {
        Ok(std::path::PathBuf::from(format!(
            "/proc/{}/path/a.out",
            nix::unistd::getpid()
        ))) // or /proc/{}/object/a.out ?
    }
    #[cfg(not(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "solaris"
    )))]
    {
        std::env::current_exe()
    }
}

// TODO: Move to a better place, possibly a new crate
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextureFormatInfo {
    pub block_width: u32,
    pub block_height: u32,
    pub block_bits: u32,
    pub red_bits: u32,
    pub green_bits: u32,
    pub blue_bits: u32,
    pub alpha_bits: u32,
    pub depth_bits: u32,
    pub stencil_bits: u32,
    pub padding_bits: u32,
    pub exponent_bits: u32,
}

pub fn get_texture_format_info(format: schema::TextureFormat) -> TextureFormatInfo {
    use schema::TextureFormat;

    let mut info: TextureFormatInfo = Default::default();

    match format {
        TextureFormat::R32G32B32A32_FLOAT
        | TextureFormat::R32G32B32A32_UINT
        | TextureFormat::R32G32B32A32_SINT => {
            info.red_bits = 32;
            info.green_bits = 32;
            info.blue_bits = 32;
            info.alpha_bits = 32;
        }
        TextureFormat::R32G32B32_FLOAT
        | TextureFormat::R32G32B32_UINT
        | TextureFormat::R32G32B32_SINT => {
            info.red_bits = 32;
            info.green_bits = 32;
            info.blue_bits = 32;
        }
        TextureFormat::R16G16B16A16_FLOAT
        | TextureFormat::R16G16B16A16_UNORM
        | TextureFormat::R16G16B16A16_UINT
        | TextureFormat::R16G16B16A16_SNORM
        | TextureFormat::R16G16B16A16_SINT => {
            info.red_bits = 16;
            info.green_bits = 16;
            info.blue_bits = 16;
            info.alpha_bits = 16;
        }
        TextureFormat::R32G32_FLOAT | TextureFormat::R32G32_UINT | TextureFormat::R32G32_SINT => {
            info.red_bits = 32;
            info.green_bits = 32;
        }
        TextureFormat::D32_FLOAT_S8_UINT => {
            info.depth_bits = 32;
            info.stencil_bits = 8;
            info.padding_bits = 24;
        }
        TextureFormat::R10G10B10A2_UNORM | TextureFormat::R10G10B10A2_UINT => {
            info.red_bits = 10;
            info.green_bits = 10;
            info.blue_bits = 10;
            info.alpha_bits = 2;
        }
        TextureFormat::R11G11B10_FLOAT => {
            info.red_bits = 11;
            info.green_bits = 11;
            info.blue_bits = 10;
        }
        TextureFormat::R8G8B8_UNORM | TextureFormat::R8G8B8_SRGB => {
            info.red_bits = 8;
            info.green_bits = 8;
            info.blue_bits = 8;
        }
        TextureFormat::R8G8B8A8_UNORM
        | TextureFormat::R8G8B8A8_UINT
        | TextureFormat::R8G8B8A8_SNORM
        | TextureFormat::R8G8B8A8_SINT => {
            info.red_bits = 8;
            info.green_bits = 8;
            info.blue_bits = 8;
            info.alpha_bits = 8;
        }
        TextureFormat::R16G16_FLOAT
        | TextureFormat::R16G16_UNORM
        | TextureFormat::R16G16_UINT
        | TextureFormat::R16G16_SNORM
        | TextureFormat::R16G16_SINT => {
            info.red_bits = 16;
            info.green_bits = 16;
        }
        TextureFormat::D32_FLOAT => {
            info.depth_bits = 32;
        }
        TextureFormat::R32_FLOAT | TextureFormat::R32_UINT | TextureFormat::R32_SINT => {
            info.red_bits = 32;
        }
        TextureFormat::D24_UNORM_S8_UINT => {
            info.depth_bits = 24;
            info.stencil_bits = 8;
        }
        TextureFormat::R8G8_UNORM
        | TextureFormat::R8G8_UINT
        | TextureFormat::R8G8_SNORM
        | TextureFormat::R8G8_SINT => {
            info.red_bits = 8;
            info.green_bits = 8;
        }
        TextureFormat::D16_UNORM => {
            info.depth_bits = 16;
        }
        TextureFormat::R16_FLOAT
        | TextureFormat::R16_UNORM
        | TextureFormat::R16_UINT
        | TextureFormat::R16_SNORM
        | TextureFormat::R16_SINT => {
            info.red_bits = 16;
        }
        TextureFormat::R8_UNORM
        | TextureFormat::R8_UINT
        | TextureFormat::R8_SNORM
        | TextureFormat::R8_SINT => {
            info.red_bits = 8;
        }
        TextureFormat::R9G9B9E5_FLOAT => {
            info.red_bits = 9;
            info.green_bits = 9;
            info.blue_bits = 9;
            info.exponent_bits = 5;
        }
        TextureFormat::R5G6B5_UNORM => {
            info.red_bits = 5;
            info.green_bits = 6;
            info.blue_bits = 5;
        }
        TextureFormat::R5G5B5A1_UNORM => {
            info.red_bits = 5;
            info.green_bits = 6;
            info.blue_bits = 5;
            info.alpha_bits = 1;
        }
        TextureFormat::B8G8R8A8_UNORM | TextureFormat::B8G8R8A8_SRGB => {
            info.red_bits = 8;
            info.green_bits = 8;
            info.blue_bits = 8;
            info.alpha_bits = 8;
        }
        TextureFormat::BC1_UNORM
        | TextureFormat::BC1A_UNORM
        | TextureFormat::BC2_UNORM
        | TextureFormat::BC3_UNORM
        | TextureFormat::BC4_UNORM
        | TextureFormat::BC4_SNORM
        | TextureFormat::BC5_UNORM
        | TextureFormat::BC5_SNORM
        | TextureFormat::BC1_SRGB
        | TextureFormat::BC1A_SRGB
        | TextureFormat::BC2_SRGB
        | TextureFormat::BC3_SRGB
        | TextureFormat::BC6U_FLOAT
        | TextureFormat::BC6S_FLOAT
        | TextureFormat::BC7_UNORM
        | TextureFormat::BC7_SRGB => {
            // Ignore compressed formats
        }
        _ => {
            println!("format {:?} is not implemented!", format);
            unimplemented!();
        }
    }

    // Calculate block size based on total bit count
    info.block_bits += info.red_bits;
    info.block_bits += info.green_bits;
    info.block_bits += info.blue_bits;
    info.block_bits += info.alpha_bits;
    info.block_bits += info.depth_bits;
    info.block_bits += info.stencil_bits;
    info.block_bits += info.padding_bits;
    info.block_bits += info.exponent_bits;

    // A block size of zero is a compressed format
    match info.block_bits {
        0 => match format {
            TextureFormat::BC1_UNORM
            | TextureFormat::BC1_SRGB
            | TextureFormat::BC1A_UNORM
            | TextureFormat::BC1A_SRGB
            | TextureFormat::BC4_UNORM => {
                info.block_bits = 64;
                info.block_width = 4;
                info.block_height = 4;
            }
            TextureFormat::BC2_UNORM
            | TextureFormat::BC2_SRGB
            | TextureFormat::BC3_UNORM
            | TextureFormat::BC3_SRGB
            | TextureFormat::BC5_UNORM
            | TextureFormat::BC6U_FLOAT
            | TextureFormat::BC6S_FLOAT
            | TextureFormat::BC7_UNORM
            | TextureFormat::BC7_SRGB => {
                info.block_bits = 128;
                info.block_width = 4;
                info.block_height = 4;
            }
            _ => {
                println!("format {:?} is not implemented!", format);
                unimplemented!();
            }
        },
        _ => {
            info.block_width = 1;
            info.block_height = 1;
        }
    }

    info
}

pub struct TextureLayoutInfo {
    pub pitch: u32,
    pub slice_pitch: u32,
}

#[inline(always)]
pub fn get_texture_layout_info(
    format: schema::TextureFormat,
    width: u32,
    height: u32,
) -> TextureLayoutInfo {
    use std::cmp::max;
    let format_info = get_texture_format_info(format);
    let width_by_block = max(1, width / format_info.block_width);
    let height_by_block = max(1, height / format_info.block_height);
    TextureLayoutInfo {
        pitch: (width_by_block * format_info.block_bits) / 8,
        slice_pitch: (width_by_block * height_by_block * format_info.block_bits) / 8,
    }
}
