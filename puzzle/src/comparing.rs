use image::{DynamicImage, GenericImageView, Rgb, Rgba};

// Compares right edge of img1 and left edge of img2 and returns difference
pub fn compare_right_edge_abs(img1: &DynamicImage, img2: &DynamicImage, threshold: i32) -> u32{
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
            let mut  channel_diff = pixel1[channel_idx] as i32 - pixel2[channel_idx] as i32;
            channel_diff = channel_diff.abs();
            if channel_diff > threshold{
                pixel_diff += channel_diff;
            }
        }
        difference += pixel_diff;
    }


    return difference as u32
}


//JOS JE GORE AAAAAAAAAAAAAAAA
pub fn compare_right_edge_sq(img1: &DynamicImage, img2: &DynamicImage) -> u32{
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
            pixel_diff += channel_diff.pow(2);
        }
        difference += pixel_diff;
    }


    return difference as u32
}

//MALO BOLJEEEEEEE
pub fn compare_right_edge_euclidean(img1: &DynamicImage, img2: &DynamicImage) -> u32{
    let mut difference = 0;

    if img1.height() != img2.height(){
        return u32::MAX;
    }

    for height_idx in 0..img1.height(){
        // Right edge
        let pixel1 = img1.get_pixel(img1.width() - 1, height_idx);
        // Left edge
        let pixel2 = img2.get_pixel(0, height_idx);

        let pixel_diff = euclidean_distance_rgba(pixel1, pixel2) as u32;

        difference += pixel_diff;
    }


    return difference as u32
}


fn euclidean_distance_rgba(p1: Rgba<u8>, p2: Rgba<u8>) -> f64 {
    let r_diff = f64::from(p2[0] as i32 - p1[0] as i32);
    let g_diff = f64::from(p2[1] as i32 - p1[1] as i32);
    let b_diff = f64::from(p2[2] as i32 - p1[2] as i32);
    let a_diff = f64::from(p2[3] as i32 - p1[3] as i32);

    (r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2) + a_diff.powi(2)).sqrt()
}
