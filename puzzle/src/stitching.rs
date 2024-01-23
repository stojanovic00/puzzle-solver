use image::{DynamicImage, GenericImage, GenericImageView};

// Stitches img2 on the right side of img1, with starting vertical pixel position start_y
// Unpopulated pixels will have value rgba[0,0,0,0]
pub fn stitch_right(img1 : &DynamicImage, img2 :&DynamicImage, start_y: u32) -> DynamicImage{
    let width = img1.width() + img2.width();
    let height = img1.height().max(img2.height() + start_y);

    let mut result_image = DynamicImage::new_rgba8(width , height);

    // Left image
    for x in 0..img1.width(){
        for y in 0..img1.height(){
            let pixel = img1.get_pixel(x, y);
            result_image.put_pixel(x, y, pixel);
        }
    }

    // Right image
    for x in 0..img2.width(){
        for y in 0..img2.height(){
            let pixel = img2.get_pixel(x, y);
            result_image.put_pixel(x + img1.width(), y + start_y, pixel);
        }
    }

    return result_image
}

// Stitches img2 on the bottom of img1, with starting horizontal pixel position start_x
// Unpopulated pixels will have value rgba[0,0,0,0]
pub fn stitch_bottom(img1 : &DynamicImage, img2 :&DynamicImage, start_x: u32) -> DynamicImage{
    let height = img1.height() + img2.height();
    let width = img1.width().max(img2.width() + start_x);

    let mut result_image = DynamicImage::new_rgba8(width , height);

    // Top image
    for x in 0..img1.width(){
        for y in 0..img1.height(){
            let pixel = img1.get_pixel(x, y);
            result_image.put_pixel(x, y, pixel);
        }
    }

    // Bottom image
    for x in 0..img2.width(){
        for y in 0..img2.height(){
            let pixel = img2.get_pixel(x, y);
            result_image.put_pixel(x + start_x, y + img1.height(), pixel);
        }
    }

    return result_image
}
