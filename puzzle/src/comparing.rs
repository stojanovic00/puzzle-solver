use image::{DynamicImage, GenericImageView, Rgba};
use colors_transform::Color;



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


pub fn compare_right_edge_hue(img1: &DynamicImage, img2: &DynamicImage, threshold: f32) -> u32{
    let mut difference = 0.0;

    if img1.height() != img2.height(){
        return u32::MAX;
    }

    for height_idx in 0..img1.height(){
        // Right edge
        let pixel1 = img1.get_pixel(img1.width() - 1, height_idx);
        // Left edge
        let pixel2 = img2.get_pixel(0, height_idx);

       let hsv1 = colors_transform::Rgb::from(pixel1[0] as f32,pixel1[1] as f32,pixel1[2] as f32).to_hsl();
       let hue_lightness1 = hsv1.get_hue() + hsv1.get_lightness();

        let hsv2 = colors_transform::Rgb::from(pixel2[0] as f32,pixel2[1] as f32,pixel2[2] as f32).to_hsl();
        let hue_lightness2 = hsv2.get_hue() + hsv2.get_lightness();


        let hue_diff = (hue_lightness1 - hue_lightness2).abs();
        difference += hue_diff;
    }


    return difference as u32
}

pub fn compare_right_edge_delta_e(img1: &DynamicImage, img2: &DynamicImage, threshold: f32) -> u32{
    let mut difference = 0.0;

    if img1.height() != img2.height(){
        return u32::MAX;
    }

    for height_idx in 0..img1.height(){
        // Right edge
        let pixel1 = img1.get_pixel(img1.width() - 1, height_idx);
        // Left edge
        let pixel2 = img2.get_pixel(0, height_idx);

        let pixel1_lab = lab::Lab::from_rgb(&[pixel1[0], pixel1[1], pixel1[2]]);
        let pixel2_lab = lab::Lab::from_rgb(&[pixel2[0], pixel2[1], pixel2[2]]);


        let delta_l = pixel2_lab.l - pixel1_lab.l;
        let delta_a = pixel2_lab.a - pixel1_lab.a;
        let delta_b = pixel2_lab.b - pixel1_lab.b;

        let delta_e_diff = (delta_l.powi(2) + delta_a.powi(2) + delta_b.powi(2)).sqrt();
        difference += delta_e_diff;
    }


    return difference as u32
}


//LEFT
//left of img1 with right of img2
pub fn compare_left_edge_hue(img1: &DynamicImage, img2: &DynamicImage, threshold: f32) -> u32{
    let mut difference = 0.0;

    if img1.height() != img2.height(){
        return u32::MAX;
    }

    for height_idx in 0..img1.height(){
        // Left edge
        let pixel1 = img1.get_pixel(0, height_idx);
        // Right edge
        let pixel2 = img2.get_pixel(img2.width() - 1, height_idx);

        let hsv1 = colors_transform::Rgb::from(pixel1[0] as f32,pixel1[1] as f32,pixel1[2] as f32).to_hsl();
        let hue_lightness1 = hsv1.get_hue() + hsv1.get_lightness();

        let hsv2 = colors_transform::Rgb::from(pixel2[0] as f32,pixel2[1] as f32,pixel2[2] as f32).to_hsl();
        let hue_lightness2 = hsv2.get_hue() + hsv2.get_lightness();


        let hue_diff = (hue_lightness1 - hue_lightness2).abs();
        difference += hue_diff;
    }


    return difference as u32
}
