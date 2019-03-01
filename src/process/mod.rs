pub mod bcn;
pub mod manifest;
pub mod mips;

pub mod generated;
pub use generated::service::texture::schema;
pub use schema::{Texture, TextureData, TextureDesc};

pub use self::bcn::*;
pub use self::manifest::*;
pub use self::mips::*;

pub fn alpha_format(color_type: image::ColorType) -> bool {
    match color_type {
        image::ColorType::Gray(ref _bit_depth) => false,
        image::ColorType::GrayA(ref _bit_depth) => true,
        image::ColorType::RGB(ref _bit_depth) => false,
        image::ColorType::RGBA(ref _bit_depth) => true,
        image::ColorType::BGRA(ref _bit_depth) => true,
        image::ColorType::BGR(ref _bit_depth) => false,
        image::ColorType::Palette(ref _bit_depth) => unimplemented!(),
    }
}

pub fn calculate_mip_count(width: u32, height: u32) -> u32 {
    1 + (std::cmp::max(width, height) as f32).log2().floor() as u32
}
