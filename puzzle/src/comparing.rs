use image::{DynamicImage, GenericImageView};

// Compares right edge of img1 and left edge of img2 and returns difference
pub fn compare_right_edge(img1: &DynamicImage, img2: &DynamicImage) -> u32{
    let mut difference = 0;

    if img1.height() != img2.height(){
        return u32::MAX;
    }

    for height_idx in 0..img1.height(){
        let mut pixel_diff = 0;
        // Right edge
        let pixel1 = img1.get_pixel(img1.width() - 1, height_idx);
        // Left edge
        let pixel2 = img2.get_pixel(0, height_idx);

        for channel_idx in 0..4{
            let  channel_diff = pixel1[channel_idx] as i32 - pixel2[channel_idx] as i32;
            pixel_diff += channel_diff.abs();
        }
        difference += pixel_diff;
    }


    return difference as u32
}
