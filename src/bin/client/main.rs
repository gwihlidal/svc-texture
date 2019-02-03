#![allow(dead_code)]

extern crate base58;
extern crate chashmap;
extern crate scoped_threadpool;
extern crate serde;
extern crate svc_texture;
extern crate tower_http;
extern crate tower_util;
extern crate yansi;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate ddsfile;
extern crate fern;
extern crate flatbuffers;
extern crate image;

use scoped_threadpool::Pool;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use svc_texture::client::transport;
use svc_texture::compile::*;
//use svc_texture::encoding::{decode_data, encode_data, Encoding};
use ddsfile::{AlphaMode, Caps2, D3D10ResourceDimension, Dds, DxgiFormat};
use image::FilterType;
use image::GenericImageView;
use image::ImageBuffer;
use image::Pixel;
use intel_tex::bc7;
use std::sync::{Arc, RwLock};
use svc_texture::error::Result;
use svc_texture::utilities::{
    compute_file_identity, compute_identity, /*self,*/ path_exists, read_file,
};

mod generated;
//use crate::generated::service::texture::schema;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[derive(StructOpt, Debug)]
#[structopt(name = "Texture Build")]
struct Options {
    /// Activate debug mode
    #[structopt(short = "x", long = "debug")]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Input manifest
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,

    /// Output manifest
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Cache directory
    #[structopt(short = "c", long = "cache", parse(from_os_str))]
    cache: Option<PathBuf>,

    /// Embed data in output manifest
    #[structopt(short = "e", long = "embed")]
    embed: bool,

    /// Download artifacts to local cache
    #[structopt(short = "d", long = "download")]
    download: bool,

    /// Service remote endpoint (defaults to 127.0.0.1:63999)
    #[structopt(short = "t", long = "endpoint")]
    endpoint: Option<String>,

    /// Windowing size
    #[structopt(short = "w", long = "window_size")]
    window_size: Option<u32>,

    /// Connection windowing size
    #[structopt(short = "n", long = "connection_window_size")]
    connection_window_size: Option<u32>,

    /// Parallel compilation
    #[structopt(short = "p", long = "parallel")]
    parallel: bool,
}

fn cache_hit(cache_path: &Path, identity: &str) -> bool {
    let data_path = cache_path.join(&identity);
    path_exists(&data_path)
}

fn cache_miss(cache_path: &Path, identity: &str) -> bool {
    !cache_hit(cache_path, identity)
}

fn cache_if_missing(cache_path: &Path, identity: &str, data: &[u8]) -> Result<()> {
    if cache_miss(cache_path, identity) {
        let data_path = cache_path.join(&identity);
        let data_file = File::create(data_path)?;
        let mut data_writer = BufWriter::new(data_file);
        data_writer.write_all(data)?;
    }
    Ok(())
}

fn fetch_from_cache(cache_path: &Path, identity: &str) -> Result<Vec<u8>> {
    let data_path = cache_path.join(&identity);
    Ok(read_file(&data_path)?)
}

fn main() {
    if let Err(err) = process() {
        let err = failure::Error::from(err);
        let mut count_context = 0;
        let mut _indent = " ".to_string();
        let causation = "".to_string();
        let separator = "---------------------------------------------------------".to_string();
        let mut message = "=========================================================\n".to_string();
        message.push_str(&format!(
            "Texture Build encountered an {}",
            yansi::Paint::red("error")
        ));
        message.push_str("\n");
        message.push_str(&separator);
        message.push_str("\n");
        message.push_str(&format!("{}", yansi::Paint::yellow(err.to_string())));
        message.push_str("\n");
        message.push_str(&separator);
        for cause in err.iter_causes() {
            message.push_str("\n");
            message.push_str(&_indent);
            _indent.push_str(&" ".to_string());
            message.push_str("▶ ");
            message.push_str(&causation);
            message.push_str(": ");
            message.push_str(&cause.to_string());
            count_context += 1;
        }
        if count_context != 0 {
            message.push_str("\n");
            //message.push_str(&separator);
        }

        error!("{}", message);
        std::process::exit(1);
    }
}

#[derive(Clone, Default, Debug)]
struct TextureArtifact {
    name: String,
    identity: String,
    encoding: String,
}

#[derive(Clone, Default, Debug)]
struct TextureRecord {
    entry: TextureEntry,
    input_identity: String,
    output_identity: Option<String>,
}

#[derive(Clone, Debug)]
pub enum OutputFormat {
    Bc1,
    Bc2,
    Bc3,
    Bc4,
    Bc5,
    Bc6h,
    Bc7,
}

fn guess_format(color_type: image::ColorType) -> (OutputFormat, bool /* alpha */) {
    // http://www.reedbeta.com/blog/understanding-bcn-texture-compression-formats/
    match color_type {
        image::ColorType::Gray(ref _bit_depth) => (OutputFormat::Bc4, false),
        image::ColorType::GrayA(ref _bit_depth) => (OutputFormat::Bc4, true),
        image::ColorType::RGB(ref _bit_depth) => {
            //OutputFormat::Bc1
            (OutputFormat::Bc7, false)
        }
        image::ColorType::RGBA(ref _bit_depth) => {
            //OutputFormat::Bc3
            (OutputFormat::Bc7, true)
        }
        image::ColorType::BGRA(ref _bit_depth) => {
            //OutputFormat::Bc3
            (OutputFormat::Bc7, true)
        }
        image::ColorType::BGR(ref _bit_depth) => (OutputFormat::Bc1, false),
        image::ColorType::Palette(ref _bit_depth) => unimplemented!(),
    }
}

fn calculate_mip_count(width: u32, height: u32) -> u32 {
    1 + (std::cmp::max(width, height) as f32).log2().floor() as u32
}

fn process() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let process_opt = Options::from_args();

    let verbosity = if process_opt.debug { 1 } else { 0 };
    setup_logging(verbosity).expect("failed to initialize logging.");

    info!(
        "Texture Build v{} starting up!",
        VERSION.unwrap_or("UNKNOWN")
    );
    debug!("{:?}", process_opt);

    let cache_path = match process_opt.cache {
        Some(ref cache_path) => cache_path,
        None => Path::new("./.cache"),
    };

    std::fs::create_dir_all(cache_path)?;

    let config = transport::Config {
        address: if let Some(ref endpoint) = process_opt.endpoint {
            endpoint.to_owned()
        } else {
            "127.0.0.1:63998".to_string()
        },
        window_size: process_opt.window_size,
        connection_window_size: process_opt.connection_window_size,
    };

    let mut thread_pool = Pool::new(8);

    // Load texture manifest from toml path
    let manifest = load_manifest(&process_opt.input.as_path())?;

    let mut active_identities: Vec<String> = Vec::with_capacity(manifest.entries.len());

    let records: Arc<RwLock<Vec<TextureRecord>>> = Arc::new(RwLock::new(Vec::new()));

    // Populate records from entries
    if manifest.entries.len() > 0 {
        let mut records = records.write().unwrap();
        for entry in &manifest.entries {
            let input_path = Path::new(&entry.file);
            let input_identity = compute_file_identity(&input_path).unwrap();
            active_identities.push(input_identity.clone());
            println!("name: {}, identity: {}", entry.name, input_identity);
            records.push(TextureRecord {
                entry: entry.clone(),
                input_identity,
                output_identity: None,
            });
        }
    }

    // Remove multiple references to the same file (for efficiency).
    active_identities.sort_by(|a, b| a.cmp(&b));
    active_identities.dedup_by(|a, b| a.eq(&b));

    // Query what identities are missing from the remote endpoint.
    /*
    let missing_identities = transport::query_missing_identities(&config, &active_identities)?;

    // Upload missing identities to the remote endpoint.
    if process_opt.parallel {
        thread_pool.scoped(|scoped| {
            for missing_identity in &missing_identities {
                let config = config.clone();
                scoped.execute(move || {
                    info!("Uploading missing identity: {}", missing_identity);
                    let identity_data = fetch_from_cache(cache_path, &missing_identity).unwrap(); //?;
                    let uploaded_identity =
                        transport::upload_identity(&config, &missing_identity, &identity_data)
                            .unwrap(); //?;
                    assert_eq!(missing_identity, &uploaded_identity);
                });
            }
        });
    } else {
        for missing_identity in &missing_identities {
            info!("Uploading missing identity: {}", missing_identity);
            let identity_data = fetch_from_cache(cache_path, &missing_identity)?;
            let uploaded_identity =
                transport::upload_identity(&config, &missing_identity, &identity_data)?;
            assert_eq!(missing_identity, &uploaded_identity);
        }
    }
    */

    let min_width = 4;
    let min_height = 4;

    {
        let mut records = records.write().unwrap();
        for record in &mut *records {
            let input_image = image::open(&Path::new(&record.entry.file)).unwrap();
            let (width, height) = input_image.dimensions();
            let color_type = input_image.color();
            let (output_format, has_alpha) = guess_format(color_type);
            println!("ColorType is {:?}", color_type);
            println!("OutputFormat: {:?}", output_format);
            println!("Mip[0] Width is {}", width);
            println!("Mip[0] Height is {}", height);
            match color_type {
                image::ColorType::Gray(ref bit_depth) => {
                    println!("Pixel is gray scale, bit depth is {}", bit_depth);
                    // BC4
                }
                image::ColorType::GrayA(ref bit_depth) => {
                    println!(
                        "Pixel is gray scale with an alpha channel, bit depth is {}",
                        bit_depth
                    );
                }
                image::ColorType::RGB(ref bit_depth) => {
                    println!(
                        "Pixel contains R, G and B channels, bit depth is {}",
                        bit_depth
                    );
                    // BC1
                }
                image::ColorType::RGBA(ref bit_depth) => {
                    println!(
                        "Pixel is RGB with an alpha channel, bit depth is {}",
                        bit_depth
                    );
                    // BC3 or BC2 (old)
                }
                image::ColorType::BGRA(ref bit_depth) => {
                    println!(
                        "Pixel is BGR with an alpha channel, bit depth is {}",
                        bit_depth
                    );
                    // BC3 or BC2 (old)
                }
                image::ColorType::BGR(ref bit_depth) => {
                    println!(
                        "Pixel contains B, G and R channels, bit depth is {}",
                        bit_depth
                    );
                    // BC1
                }
                image::ColorType::Palette(ref bit_depth) => {
                    println!(
                        "Pixel is an index into a color palette, bit depth is {}",
                        bit_depth
                    );
                }
            }

            let generate_mips = true;
            let mip_count = if generate_mips {
                calculate_mip_count(width, height)
            } else {
                1
            };

            let mut images: Vec<image::DynamicImage> = Vec::with_capacity(mip_count as usize);
            images.push(input_image);

            for i in 1..mip_count {
                // Get mip map dimensions
                let dst_width = width >> i;
                let dst_height = height >> i;
                if dst_width < min_width || dst_height < min_height {
                    break;
                }

                let src_image = &images[i as usize - 1];

                let dst_image = src_image.resize(dst_width, dst_height, FilterType::Lanczos3);

                let block_count = intel_tex::divide_up_by_multiple(dst_width * dst_height, 16);
                println!("Block count: {}", block_count);

                let (width, height) = dst_image.dimensions();
                println!("Mip[{}] Width is {}", i, width);
                println!("Mip[{}] Height is {}", i, height);

                images.push(dst_image);
            }

            println!("Mip count: {}", images.len());

            let mip_count = images.len();
            let array_layers = 1;
            let caps2 = Caps2::empty();
            let is_cubemap = false;
            let resource_dimension = D3D10ResourceDimension::Texture2D;
            let alpha_mode = if has_alpha {
                AlphaMode::Straight
            } else {
                AlphaMode::Opaque
            };
            let depth = 1;

            let mut dds = Dds::new_dxgi(
                height,
                width,
                Some(depth),
                DxgiFormat::BC7_UNorm,
                Some(mip_count as u32),
                Some(array_layers),
                Some(caps2),
                is_cubemap,
                resource_dimension,
                alpha_mode,
            )
            .unwrap();

            let layer_data = dds.get_mut_data(0 /* layer */).unwrap();

            let mut start_offset = 0;
            for i in 0..mip_count {
                let rgba_image = images[i].to_rgba();
                let (width, height) = rgba_image.dimensions();

                let mip_size = intel_tex::bc7::calc_output_size(width, height);
                let mut mip_data = &mut layer_data[start_offset..(start_offset + mip_size)];

                let surface = intel_tex::RgbaSurface {
                    width,
                    height,
                    stride: width * 4,
                    data: &rgba_image,
                };

                println!("Compressing mip[{}] to BC7...", i);
                bc7::compress_blocks_into(
                    &bc7::opaque_ultra_fast_settings(),
                    &surface,
                    &mut mip_data,
                );

                start_offset += mip_size;
            }

            let mut dds_memory = std::io::Cursor::new(Vec::<u8>::new());
            dds.write(&mut dds_memory)
                .expect("Failed to write dds memory");

            let output_identity = compute_identity(dds_memory.get_ref());

            println!("  Done!");

            println!("Saving dds file");
            cache_if_missing(cache_path, &output_identity, &dds_memory.get_ref())?;

            record.output_identity = Some(output_identity);
        }
    }

    /*
        // DO STUFF

        if process_opt.download || (process_opt.output.is_some() && process_opt.embed) {
            let records = records.read().unwrap();
            for record in &*records {
                let identity_path = cache_path.join(&record.artifact.identity);
                if cache_miss(cache_path, &record.artifact.identity) {
                    let remote_data = transport::download_identity(&config, &record.artifact.identity)?;
                    cache_if_missing(cache_path, &record.artifact.identity, &remote_data)?;
                    debug!(
                        "  '{}' [Cache Miss]: {:?}",
                        record.artifact.name, identity_path
                    );
                } else {
                    debug!(
                        "  '{}' [Cache Hit]: {:?}",
                        record.artifact.name, identity_path
                    );
                }
            }
        }

    */

    {
        let records = records.read().unwrap();
        println!("Records: {:?}", records);
    }

    if let Some(ref output_path) = process_opt.output {
        let records = records.read().unwrap();
        println!("Records: {:?}", records);
        /*let mut manifest_builder = flatbuffers::FlatBufferBuilder::new();
        let manifest_textures: Vec<_> = records
            .iter()
            .map(|texture| {
                let artifact = &texture.artifact;
                let name = Some(manifest_builder.create_string(&texture.name));
                let entry = Some(manifest_builder.create_string(&texture.entry));
                let name = Some(manifest_builder.create_string(&artifact.name));
                let identity = Some(manifest_builder.create_string(&artifact.identity));
                let encoding = Some(manifest_builder.create_string(&artifact.encoding));
                let data = if process_opt.embed {
                    let data = fetch_from_cache(cache_path, &artifact.identity)
                        .expect("failed to fetch from cache");
                    Some(manifest_builder.create_vector(&data))
                } else {
                    None
                };
                schema::Artifact::create(
                    &mut manifest_builder,
                    &schema::ArtifactArgs {
                        name,
                        identity,
                        encoding,
                        data,
                    },
                )
            })
            .collect();

        let manifest_textures = Some(manifest_builder.create_vector(&manifest_textures));
        let manifest = schema::Manifest::create(
            &mut manifest_builder,
            &schema::ManifestArgs {
                textures: manifest_textures,
            },
        );

        manifest_builder.finish(manifest, None);
        let manifest_data = manifest_builder.finished_data();
        let manifest_file = File::create(output_path)?;
        let mut manifest_writer = BufWriter::new(manifest_file);
        manifest_writer.write_all(&manifest_data)?;*/
    }

    Ok(())
}

fn setup_logging(verbosity: u64) -> Result<()> {
    std::fs::create_dir_all(Path::new("./logs"))?;

    let mut base_config = fern::Dispatch::new();
    base_config = match verbosity {
        0 => {
            // Let's say we depend on something which whose "info" level messages are too
            // verbose to include in end-user output. If we don't need them,
            // let's not include them.
            base_config
                .level(log::LevelFilter::Info)
                .level_for("overly-verbose-target", log::LevelFilter::Warn)
                .level_for("tokio_core", log::LevelFilter::Warn)
                .level_for("tokio_reactor", log::LevelFilter::Warn)
                .level_for("httpbis", log::LevelFilter::Warn)
        }
        1 => base_config
            .level(log::LevelFilter::Debug)
            .level_for("overly-verbose-target", log::LevelFilter::Info)
            .level_for("tokio_core", log::LevelFilter::Warn)
            .level_for("tokio_reactor", log::LevelFilter::Warn)
            .level_for("h2", log::LevelFilter::Warn)
            .level_for("httpbis", log::LevelFilter::Warn),
        2 => base_config.level(log::LevelFilter::Debug),
        _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                //.append(true)
                .truncate(true)
                .open("logs/compile.log")?,
        );

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            // special format for debug messages coming from our own crate.
            if record.level() > log::LevelFilter::Info && record.target() == "texture_build" {
                out.finish(format_args!(
                    "---\nDEBUG: {}: {}\n---",
                    chrono::Local::now().format("%H:%M:%S"),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "[{}][{}][{}] {}",
                    chrono::Local::now().format("%H:%M"),
                    record.target(),
                    record.level(),
                    message
                ))
            }
        })
        .chain(::std::io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();

    Ok(())
}
