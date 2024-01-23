mod stitching;
use image::{GenericImage, GenericImageView};



fn main() {
    let piece1 = image::open("../examples/slika1/part_b.jpg").expect("Failed to open");
    let piece2 = image::open("../examples/slika1/part_a.jpg").expect("Failed to open");

    let mut result_image = stitching::stitch_right(&piece1, &piece2, 9);
    result_image = stitching::stitch_bottom(&result_image, &piece2, 0);
    result_image = stitching::stitch_bottom(&result_image, &piece2, piece1.width());

    result_image.save("final.jpg").expect("Failed to save");
}
