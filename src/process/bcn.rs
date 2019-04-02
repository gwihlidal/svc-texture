use crate::process::*;
use ddsfile::{AlphaMode, Caps2, D3D10ResourceDimension, D3DFormat, Dds, DxgiFormat};
use image::FilterType;
use image::GenericImageView;
use image::ImageBuffer;
use image::Pixel;
use intel_tex::*;

pub fn is_bcn_format(format: schema::TextureFormat) -> bool {
    match format {
        schema::TextureFormat::BC1_UNORM
        | schema::TextureFormat::BC1_SRGB
        | schema::TextureFormat::BC1A_UNORM
        | schema::TextureFormat::BC1A_SRGB
        | schema::TextureFormat::BC2_UNORM
        | schema::TextureFormat::BC2_SRGB
        | schema::TextureFormat::BC3_UNORM
        | schema::TextureFormat::BC3_SRGB
        | schema::TextureFormat::BC4_UNORM
        | schema::TextureFormat::BC4_SNORM
        | schema::TextureFormat::BC5_UNORM
        | schema::TextureFormat::BC5_SNORM
        | schema::TextureFormat::BC6U_FLOAT
        | schema::TextureFormat::BC6S_FLOAT
        | schema::TextureFormat::BC7_UNORM
        | schema::TextureFormat::BC7_SRGB => true,
        _ => false,
    }
}

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

pub fn guess_format(color_type: image::ColorType) -> (schema::TextureFormat, bool /* alpha */) {
    // http://www.reedbeta.com/blog/understanding-bcn-texture-compression-formats/
    match color_type {
        image::ColorType::Gray(ref _bit_depth) => (schema::TextureFormat::BC4_UNORM, false),
        image::ColorType::GrayA(ref _bit_depth) => (schema::TextureFormat::BC4_UNORM, true),
        image::ColorType::RGB(ref _bit_depth) => {
            //OutputFormat::Bc1
            (schema::TextureFormat::BC7_UNORM, false)
        }
        image::ColorType::RGBA(ref _bit_depth) => {
            //OutputFormat::Bc3
            (schema::TextureFormat::BC7_UNORM, true)
        }
        image::ColorType::BGRA(ref _bit_depth) => {
            //OutputFormat::Bc3
            (schema::TextureFormat::BC7_UNORM, true)
        }
        image::ColorType::BGR(ref _bit_depth) => (schema::TextureFormat::BC1_UNORM, false),
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

pub fn get_dds_format(dds: &Dds) -> schema::TextureFormat {
    use schema::TextureFormat;
    if let Some(dxgi) = dds.get_dxgi_format() {
        match dxgi {
            DxgiFormat::Unknown => TextureFormat::UNKNOWN,
            DxgiFormat::R32G32B32A32_Typeless => unimplemented!(),
            DxgiFormat::R32G32B32A32_Float => TextureFormat::R32G32B32A32_FLOAT,
            DxgiFormat::R32G32B32A32_UInt => unimplemented!(),
            DxgiFormat::R32G32B32A32_SInt => unimplemented!(),
            DxgiFormat::R32G32B32_Typeless => unimplemented!(),
            DxgiFormat::R32G32B32_Float => unimplemented!(),
            DxgiFormat::R32G32B32_UInt => unimplemented!(),
            DxgiFormat::R32G32B32_SInt => unimplemented!(),
            DxgiFormat::R16G16B16A16_Typeless => unimplemented!(),
            DxgiFormat::R16G16B16A16_Float => unimplemented!(),
            DxgiFormat::R16G16B16A16_UNorm => unimplemented!(),
            DxgiFormat::R16G16B16A16_UInt => unimplemented!(),
            DxgiFormat::R16G16B16A16_SNorm => unimplemented!(),
            DxgiFormat::R16G16B16A16_SInt => unimplemented!(),
            DxgiFormat::R32G32_Typeless => unimplemented!(),
            DxgiFormat::R32G32_Float => unimplemented!(),
            DxgiFormat::R32G32_UInt => unimplemented!(),
            DxgiFormat::R32G32_SInt => unimplemented!(),
            DxgiFormat::R32G8X24_Typeless => unimplemented!(),
            DxgiFormat::D32_Float_S8X24_UInt => unimplemented!(),
            DxgiFormat::R32_Float_X8X24_Typeless => unimplemented!(),
            DxgiFormat::X32_Typeless_G8X24_UInt => unimplemented!(),
            DxgiFormat::R10G10B10A2_Typeless => unimplemented!(),
            DxgiFormat::R10G10B10A2_UNorm => unimplemented!(),
            DxgiFormat::R10G10B10A2_UInt => unimplemented!(),
            DxgiFormat::R11G11B10_Float => unimplemented!(),
            DxgiFormat::R8G8B8A8_Typeless => unimplemented!(),
            DxgiFormat::R8G8B8A8_UNorm => TextureFormat::R8G8B8A8_UNORM,
            DxgiFormat::R8G8B8A8_UNorm_sRGB => TextureFormat::R8G8B8A8_SRGB,
            DxgiFormat::R8G8B8A8_UInt => TextureFormat::R8G8B8A8_UINT,
            DxgiFormat::R8G8B8A8_SNorm => TextureFormat::R8G8B8A8_SNORM,
            DxgiFormat::R8G8B8A8_SInt => TextureFormat::R8G8B8A8_SINT,
            DxgiFormat::R16G16_Typeless => unimplemented!(),
            DxgiFormat::R16G16_Float => unimplemented!(),
            DxgiFormat::R16G16_UNorm => unimplemented!(),
            DxgiFormat::R16G16_UInt => unimplemented!(),
            DxgiFormat::R16G16_SNorm => unimplemented!(),
            DxgiFormat::R16G16_SInt => unimplemented!(),
            DxgiFormat::R32_Typeless => unimplemented!(),
            DxgiFormat::D32_Float => unimplemented!(),
            DxgiFormat::R32_Float => unimplemented!(),
            DxgiFormat::R32_UInt => unimplemented!(),
            DxgiFormat::R32_SInt => unimplemented!(),
            DxgiFormat::R24G8_Typeless => unimplemented!(),
            DxgiFormat::D24_UNorm_S8_UInt => unimplemented!(),
            DxgiFormat::R24_UNorm_X8_Typeless => unimplemented!(),
            DxgiFormat::X24_Typeless_G8_UInt => unimplemented!(),
            DxgiFormat::R8G8_Typeless => unimplemented!(),
            DxgiFormat::R8G8_UNorm => unimplemented!(),
            DxgiFormat::R8G8_UInt => unimplemented!(),
            DxgiFormat::R8G8_SNorm => unimplemented!(),
            DxgiFormat::R8G8_SInt => unimplemented!(),
            DxgiFormat::R16_Typeless => unimplemented!(),
            DxgiFormat::R16_Float => unimplemented!(),
            DxgiFormat::D16_UNorm => unimplemented!(),
            DxgiFormat::R16_UNorm => unimplemented!(),
            DxgiFormat::R16_UInt => unimplemented!(),
            DxgiFormat::R16_SNorm => unimplemented!(),
            DxgiFormat::R16_SInt => unimplemented!(),
            DxgiFormat::R8_Typeless => unimplemented!(),
            DxgiFormat::R8_UNorm => unimplemented!(),
            DxgiFormat::R8_UInt => unimplemented!(),
            DxgiFormat::R8_SNorm => unimplemented!(),
            DxgiFormat::R8_SInt => unimplemented!(),
            DxgiFormat::A8_UNorm => unimplemented!(),
            DxgiFormat::R1_UNorm => unimplemented!(),
            DxgiFormat::R9G9B9E5_SharedExp => unimplemented!(),
            DxgiFormat::R8G8_B8G8_UNorm => unimplemented!(),
            DxgiFormat::G8R8_G8B8_UNorm => unimplemented!(),
            DxgiFormat::BC1_Typeless => unimplemented!(),
            DxgiFormat::BC1_UNorm => TextureFormat::BC1_UNORM,
            DxgiFormat::BC1_UNorm_sRGB => TextureFormat::BC1_SRGB,
            DxgiFormat::BC2_Typeless => unimplemented!(),
            DxgiFormat::BC2_UNorm => TextureFormat::BC2_UNORM,
            DxgiFormat::BC2_UNorm_sRGB => TextureFormat::BC2_SRGB,
            DxgiFormat::BC3_Typeless => unimplemented!(),
            DxgiFormat::BC3_UNorm => TextureFormat::BC3_UNORM,
            DxgiFormat::BC3_UNorm_sRGB => TextureFormat::BC3_SRGB,
            DxgiFormat::BC4_Typeless => unimplemented!(),
            DxgiFormat::BC4_UNorm => TextureFormat::BC4_UNORM,
            DxgiFormat::BC4_SNorm => TextureFormat::BC4_SNORM,
            DxgiFormat::BC5_Typeless => unimplemented!(),
            DxgiFormat::BC5_UNorm => TextureFormat::BC5_UNORM,
            DxgiFormat::BC5_SNorm => TextureFormat::BC5_SNORM,
            DxgiFormat::B5G6R5_UNorm => unimplemented!(),
            DxgiFormat::B5G5R5A1_UNorm => unimplemented!(),
            DxgiFormat::B8G8R8A8_UNorm => unimplemented!(),
            DxgiFormat::B8G8R8X8_UNorm => unimplemented!(),
            DxgiFormat::R10G10B10_XR_Bias_A2_UNorm => unimplemented!(),
            DxgiFormat::B8G8R8A8_Typeless => unimplemented!(),
            DxgiFormat::B8G8R8A8_UNorm_sRGB => unimplemented!(),
            DxgiFormat::B8G8R8X8_Typeless => unimplemented!(),
            DxgiFormat::B8G8R8X8_UNorm_sRGB => unimplemented!(),
            DxgiFormat::BC6H_Typeless => unimplemented!(),
            DxgiFormat::BC6H_UF16 => TextureFormat::BC6U_FLOAT,
            DxgiFormat::BC6H_SF16 => TextureFormat::BC6S_FLOAT,
            DxgiFormat::BC7_Typeless => unimplemented!(),
            DxgiFormat::BC7_UNorm => TextureFormat::BC7_UNORM,
            DxgiFormat::BC7_UNorm_sRGB => TextureFormat::BC7_SRGB,
            DxgiFormat::AYUV => unimplemented!(),
            DxgiFormat::Y410 => unimplemented!(),
            DxgiFormat::Y416 => unimplemented!(),
            DxgiFormat::NV12 => unimplemented!(),
            DxgiFormat::P010 => unimplemented!(),
            DxgiFormat::P016 => unimplemented!(),
            DxgiFormat::Format_420_Opaque => unimplemented!(),
            DxgiFormat::YUY2 => unimplemented!(),
            DxgiFormat::Y210 => unimplemented!(),
            DxgiFormat::Y216 => unimplemented!(),
            DxgiFormat::NV11 => unimplemented!(),
            DxgiFormat::AI44 => unimplemented!(),
            DxgiFormat::IA44 => unimplemented!(),
            DxgiFormat::P8 => unimplemented!(),
            DxgiFormat::A8P8 => unimplemented!(),
            DxgiFormat::B4G4R4A4_UNorm => unimplemented!(),
            DxgiFormat::P208 => unimplemented!(),
            DxgiFormat::V208 => unimplemented!(),
            DxgiFormat::V408 => unimplemented!(),
            DxgiFormat::Force_UInt => unreachable!(),
        }
    } else if let Some(d3d) = dds.get_d3d_format() {
        match d3d {
            D3DFormat::A8B8G8R8 => unimplemented!(),
            D3DFormat::G16R16 => unimplemented!(),
            D3DFormat::A2B10G10R10 => unimplemented!(),
            D3DFormat::A1R5G5B5 => unimplemented!(),
            D3DFormat::R5G6B5 => unimplemented!(),
            D3DFormat::A8 => unimplemented!(),
            D3DFormat::A8R8G8B8 => unimplemented!(),
            D3DFormat::X8R8G8B8 => unimplemented!(),
            D3DFormat::X8B8G8R8 => unimplemented!(),
            D3DFormat::A2R10G10B10 => unimplemented!(),
            D3DFormat::R8G8B8 => unimplemented!(),
            D3DFormat::X1R5G5B5 => unimplemented!(),
            D3DFormat::A4R4G4B4 => unimplemented!(),
            D3DFormat::X4R4G4B4 => unimplemented!(),
            D3DFormat::A8R3G3B2 => unimplemented!(),
            D3DFormat::A8L8 => unimplemented!(),
            D3DFormat::L16 => unimplemented!(),
            D3DFormat::L8 => unimplemented!(),
            D3DFormat::A4L4 => unimplemented!(),
            D3DFormat::DXT1 => unimplemented!(),
            D3DFormat::DXT3 => unimplemented!(),
            D3DFormat::DXT5 => unimplemented!(),
            D3DFormat::R8G8_B8G8 => unimplemented!(),
            D3DFormat::G8R8_G8B8 => unimplemented!(),
            D3DFormat::A16B16G16R16 => unimplemented!(),
            D3DFormat::Q16W16V16U16 => unimplemented!(),
            D3DFormat::R16F => unimplemented!(),
            D3DFormat::G16R16F => unimplemented!(),
            D3DFormat::A16B16G16R16F => unimplemented!(),
            D3DFormat::R32F => unimplemented!(),
            D3DFormat::G32R32F => TextureFormat::R32G32_FLOAT, // TODO: Component swizzle?
            D3DFormat::A32B32G32R32F => TextureFormat::R32G32B32A32_FLOAT,
            D3DFormat::DXT2 => unimplemented!(),
            D3DFormat::DXT4 => unimplemented!(),
            D3DFormat::UYVY => unimplemented!(),
            D3DFormat::YUY2 => unimplemented!(),
            D3DFormat::CXV8U8 => unimplemented!(),
        }
    } else {
        TextureFormat::UNKNOWN
    }
}

pub fn extract_dds_result(dds: &Dds) -> (schema::TextureDescArgs, Vec<u8>) {
    let format = get_dds_format(dds);

    // This gets the number of bytes required to store one row of data
    //let pitch = data_format.get_pitch(dds.get_width()).expect("failed to determine pitch");
    let _pitch = dds.get_pitch().expect("failed to parse dds pitch");

    // This gets the height of each row of data. Normally it is 1, but for block
    // compressed textures, each row is 4 pixels high.
    let _pitch_height = dds.get_pitch_height();

    // This gets the number of bits required to store a single pixel, and is
    // only defined for uncompressed formats.
    //let bits_per_pixel = dds.get_bits_per_pixel();

    // This gets a block compression format's block size, and is only defined
    // for compressed formats.
    //let block_size = dds.get_block_size();

    // This gets the minimum mipmap size in bytes. Even if they go all the way
    // down to 1x1, there is a minimum number of bytes based on bits per pixel
    // or blocksize.
    let _min_mip_size = dds.get_min_mipmap_size_in_bytes();

    //println!("dds: {:?}", dds);

    // Is DX10 extension required for this format?
    //let extension = data_format.requires_extension();

    // TODO: The dds crate needs some work...

    let texture_type = /*if let Some(ref header10) = dds.header10 {
        println!("Have header10");
        let is_array = false; // TODO
        let is_cube = header10.get_
        //.misc_flag.contains(ddsfile::Header10::MiscFlag::TEXTURECODE);
                             /*
                             DDS: Dds:
                             Format: A32B32G32R32F
                             Header:
                                 flags: CAPS | HEIGHT | WIDTH | PITCH | PIXELFORMAT | MIPMAPCOUNT
                                 height: 512, width: 512, depth: None
                                 pitch: Some(32)  linear_size: None
                                 mipmap_count: Some(10)
                                 caps: MIPMAP | TEXTURE, caps2 CUBEMAP | CUBEMAP_POSITIVEX | CUBEMAP_NEGATIVEX | CUBEMAP_POSITIVEY | CUBEMAP_NEGATIVEY | CUBEMAP_POSITIVEZ | CUBEMAP_NEGATIVEZ
                                 Pixel Format:
                                 flags: FOURCC
                                 fourcc: Some(FourCC(116))
                                 bits_per_pixel: None
                                 RGBA bitmasks: None, None, None, None
                             (data elided)
                             */

        if is_array {
            if is_cube {
                schema::TextureType::CubeArray
            } else {
                match header10.resource_dimension {
                    D3D10ResourceDimension::Unknown => unimplemented!(),
                    D3D10ResourceDimension::Buffer => unimplemented!(),
                    D3D10ResourceDimension::Texture1D => schema::TextureType::Tex1dArray,
                    D3D10ResourceDimension::Texture2D => schema::TextureType::Tex2dArray,
                    D3D10ResourceDimension::Texture3D => unimplemented!(),
                }
            }
        } else {
            if is_cube {
                schema::TextureType::Cube
            } else {
                match header10.resource_dimension {
                    D3D10ResourceDimension::Unknown => unimplemented!(),
                    D3D10ResourceDimension::Buffer => unimplemented!(),
                    D3D10ResourceDimension::Texture1D => schema::TextureType::Tex1dArray, // TODO
                    D3D10ResourceDimension::Texture2D => schema::TextureType::Tex2d,
                    D3D10ResourceDimension::Texture3D => schema::TextureType::Tex3d,
                }
            }
        }
    } else {
    */
        if dds.header.caps2.contains(ddsfile::Caps2::VOLUME) {
            schema::TextureType::Tex3d
        } else {
            let is_array = dds.get_num_array_layers() > 1;
            if dds.header.caps2.contains(ddsfile::Caps2::CUBEMAP) {
                schema::TextureType::Cube
            } else if dds.get_height() > 1 {
                if is_array {
                     schema::TextureType::Tex2dArray
                } else {
                    schema::TextureType::Tex2d
                }
            } else {
                if is_array {
                    schema::TextureType::Tex1dArray
                } else {
                    schema::TextureType::Tex1d
                }
            }
        };
    //};

    //dds.get_offset_and_size(array_layer: u32)
    //dds.get_data(array_layer: u32)
    //dds.get_array_stride()
    //println!("DDS: {:?}", dds);
    let desc = schema::TextureDescArgs {
        type_: texture_type,
        format,
        width: dds.get_width(),
        height: dds.get_height(),
        depth: dds.get_depth(),
        levels: dds.get_num_mipmap_levels(),
        elements: dds.get_num_array_layers(),
    };
    (desc, dds.data.clone())
}
