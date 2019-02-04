use crate::process::*;
use ddsfile::{AlphaMode, Caps2, D3D10ResourceDimension, Dds, DxgiFormat};
use image::FilterType;
use image::GenericImageView;
use image::ImageBuffer;
use image::Pixel;
use intel_tex::*;

pub enum Bc6hQuality {
    VeryFast,
    Basic,
    Slow,
    VerySlow,
}

pub enum Bc7Quality {
    UltraFast,
    VeryFast,
    Fast,
    Basic,
    Slow,
}

pub fn get_bc6h_settings(quality: Bc6hQuality) -> bc6h::EncodeSettings {
    match quality {
        Bc6hQuality::VeryFast => bc6h::very_fast_settings(),
        Bc6hQuality::Basic => bc6h::basic_settings(),
        Bc6hQuality::Slow => bc6h::slow_settings(),
        Bc6hQuality::VerySlow => bc6h::very_slow_settings(),
    }
}

pub fn get_bc7_settings(quality: Bc7Quality, alpha: bool) -> bc7::EncodeSettings {
    if alpha {
        match quality {
            Bc7Quality::UltraFast => bc7::alpha_ultra_fast_settings(),
            Bc7Quality::VeryFast => bc7::alpha_very_fast_settings(),
            Bc7Quality::Fast => bc7::alpha_fast_settings(),
            Bc7Quality::Basic => bc7::alpha_basic_settings(),
            Bc7Quality::Slow => bc7::alpha_slow_settings(),
        }
    } else {
        match quality {
            Bc7Quality::UltraFast => bc7::opaque_ultra_fast_settings(),
            Bc7Quality::VeryFast => bc7::opaque_very_fast_settings(),
            Bc7Quality::Fast => bc7::opaque_fast_settings(),
            Bc7Quality::Basic => bc7::opaque_basic_settings(),
            Bc7Quality::Slow => bc7::opaque_slow_settings(),
        }
    }
}

pub fn guess_format(color_type: image::ColorType) -> (OutputFormat, bool /* alpha */) {
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

pub fn compress_bc1_2d(images: &Vec<image::DynamicImage>) -> Vec<u8> {
    if images.len() == 0 {
        return Vec::new();
    }

    let top_level = &images[0];
    let (width, height) = top_level.dimensions();

    let mip_count = images.len();
    let array_layers = 1;
    let caps2 = Caps2::empty();
    let is_cubemap = false;
    let resource_dimension = D3D10ResourceDimension::Texture2D;
    let depth = 1;

    let mut dds = Dds::new_dxgi(
        height,
        width,
        Some(depth),
        DxgiFormat::BC1_UNorm,
        Some(mip_count as u32),
        Some(array_layers),
        Some(caps2),
        is_cubemap,
        resource_dimension,
        AlphaMode::Opaque,
    )
    .unwrap();

    let layer_data = dds.get_mut_data(0 /* layer */).unwrap();

    let mut start_offset = 0;
    for i in 0..mip_count {
        let rgba_image = images[i].to_rgba();
        let (width, height) = rgba_image.dimensions();

        let mip_size = intel_tex::bc1::calc_output_size(width, height);
        let mut mip_data = &mut layer_data[start_offset..(start_offset + mip_size)];

        let surface = intel_tex::RgbaSurface {
            width,
            height,
            stride: width * 4,
            data: &rgba_image,
        };

        bc1::compress_blocks_into(&surface, &mut mip_data);

        start_offset += mip_size;
    }

    serialize_dds_bytes(&dds)
}

pub fn compress_bc3_2d(images: &Vec<image::DynamicImage>) -> Vec<u8> {
    if images.len() == 0 {
        return Vec::new();
    }

    let top_level = &images[0];
    let (width, height) = top_level.dimensions();

    let mip_count = images.len();
    let array_layers = 1;
    let caps2 = Caps2::empty();
    let is_cubemap = false;
    let resource_dimension = D3D10ResourceDimension::Texture2D;
    let depth = 1;

    let mut dds = Dds::new_dxgi(
        height,
        width,
        Some(depth),
        DxgiFormat::BC3_UNorm,
        Some(mip_count as u32),
        Some(array_layers),
        Some(caps2),
        is_cubemap,
        resource_dimension,
        AlphaMode::Opaque,
    )
    .unwrap();

    let layer_data = dds.get_mut_data(0 /* layer */).unwrap();

    let mut start_offset = 0;
    for i in 0..mip_count {
        let rgba_image = images[i].to_rgba();
        let (width, height) = rgba_image.dimensions();

        let mip_size = intel_tex::bc3::calc_output_size(width, height);
        let mut mip_data = &mut layer_data[start_offset..(start_offset + mip_size)];

        let surface = intel_tex::RgbaSurface {
            width,
            height,
            stride: width * 4,
            data: &rgba_image,
        };

        bc3::compress_blocks_into(&surface, &mut mip_data);

        start_offset += mip_size;
    }

    serialize_dds_bytes(&dds)
}

pub fn compress_bc6h_2d(images: &Vec<image::DynamicImage>, quality: Bc6hQuality) -> Vec<u8> {
    if images.len() == 0 {
        return Vec::new();
    }

    let top_level = &images[0];

    let bc6h_settings = get_bc6h_settings(quality);

    let (width, height) = top_level.dimensions();

    let mip_count = images.len();
    let array_layers = 1;
    let caps2 = Caps2::empty();
    let is_cubemap = false;
    let resource_dimension = D3D10ResourceDimension::Texture2D;
    let depth = 1;

    //BC6H_Typeless,
    //BC6H_UF16,
    //BC6H_SF16,

    let mut dds = Dds::new_dxgi(
        height,
        width,
        Some(depth),
        DxgiFormat::BC6H_SF16,
        Some(mip_count as u32),
        Some(array_layers),
        Some(caps2),
        is_cubemap,
        resource_dimension,
        AlphaMode::Opaque,
    )
    .unwrap();

    let layer_data = dds.get_mut_data(0 /* layer */).unwrap();

    let mut start_offset = 0;
    for i in 0..mip_count {
        let rgba_image = images[i].to_rgba();
        let (width, height) = rgba_image.dimensions();

        let mip_size = intel_tex::bc6h::calc_output_size(width, height);
        let mut mip_data = &mut layer_data[start_offset..(start_offset + mip_size)];

        let surface = intel_tex::RgbaSurface {
            width,
            height,
            stride: width * 4,
            data: &rgba_image,
        };

        bc6h::compress_blocks_into(&bc6h_settings, &surface, &mut mip_data);

        start_offset += mip_size;
    }

    serialize_dds_bytes(&dds)
}

pub fn compress_bc7_2d(images: &Vec<image::DynamicImage>, quality: Bc7Quality) -> Vec<u8> {
    if images.len() == 0 {
        return Vec::new();
    }

    let top_level = &images[0];

    let color_type = top_level.color();
    let has_alpha = alpha_format(color_type);
    let bc7_settings = get_bc7_settings(quality, has_alpha);

    let (width, height) = top_level.dimensions();

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

        bc7::compress_blocks_into(&bc7_settings, &surface, &mut mip_data);

        start_offset += mip_size;
    }

    serialize_dds_bytes(&dds)
}

pub fn serialize_dds_bytes(dds: &Dds) -> Vec<u8> {
    let mut dds_memory = std::io::Cursor::new(Vec::<u8>::new());
    dds.write(&mut dds_memory)
        .expect("Failed to write dds memory");

    dds_memory.into_inner()
}

pub fn read_dds_result<R: std::io::Read>(r: &mut R) -> (schema::TextureDescArgs, Vec<u8>) {
    let dds = Dds::read(r).expect("failed to read dds");
    extract_dds_result(&dds)
}

pub fn extract_dds_result(dds: &Dds) -> (schema::TextureDescArgs, Vec<u8>) {
    //dds.get_format()
    //dds.get_dxgi_format()
    //dds.get_d3d_format()
    //dds.data
    //dds.get_num_mipmap_levels()
    //dds.get_num_array_layers()
    //dds.get_offset_and_size(array_layer: u32)
    //dds.header10
    //dds.get_data(array_layer: u32)
    //dds.get_bits_per_pixel()
    //dds.get_array_stride()
    println!("DDS: {:?}", dds);
    let desc = schema::TextureDescArgs {
        type_: schema::TextureType::Tex2d,
        format: schema::TextureFormat::BC7_UNORM,
        width: dds.get_width(),
        height: dds.get_height(),
        depth: dds.get_depth(),
        levels: 1,
        elements: 1,
    };
    (desc, Vec::new())
}
