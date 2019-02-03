use image::FilterType;
use image::GenericImageView;
use image::ImageBuffer;
use image::Pixel;

use crate::process::calculate_mip_count;

pub fn generate_mips(
    top_level: image::DynamicImage,
    filter: image::FilterType,
    min_size: Option<(u32, u32)>,
) -> Vec<image::DynamicImage> {
    let (width, height) = top_level.dimensions();
    let mip_count = calculate_mip_count(width, height);

    let mut images: Vec<_> = Vec::with_capacity(mip_count as usize);
    images.push(top_level);

    for i in 1..mip_count {
        // Get mip map dimensions
        let dst_width = width >> i;
        let dst_height = height >> i;
        if let Some((min_width, min_height)) = min_size {
            if dst_width < min_width || dst_height < min_height {
                break;
            }
        }

        let src_image = &images[i as usize - 1];
        let dst_image = src_image.resize(dst_width, dst_height, filter);
        images.push(dst_image);
    }

    images
}
