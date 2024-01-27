use std::env;
use image::{DynamicImage, GenericImage, GenericImageView};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <puzzle_folder_path> <solved_image_path> <y-grid-size> <x-grid-size>", args[0]);
        std::process::exit(1);
    }

    let puzzle_folder_path = &args[1];
    let solved_path = &args[2];
    let y_size = &args[3];
    let x_size = &args[4];

    let y_size = y_size.parse::<u32>().expect("Invalid y size");
    let x_size = x_size.parse::<u32>().expect("Invalid x size");

    let solved_image = image::open(solved_path).unwrap_or_else(|_| {
        eprintln!("Failed to open image: {:?}", solved_path);
        std::process::exit(1);
    });

    println!("Dimensions: {}x{}", y_size, x_size);

    let piece_width = solved_image.width() / x_size;
    let last_col_width =  solved_image.width() % x_size + piece_width;

    println!("Pieces width: {}, last one: {}", piece_width, last_col_width);

    let piece_height = solved_image.height() / y_size;
    let last_row_height =  solved_image.height() % y_size + piece_height;

    println!("Pieces height: {}, last one: {}", piece_height, last_row_height);
    let mut file_counter = 1;
    let mut y_cursor = 0;
    loop{
        if y_cursor == solved_image.height(){
            break;
        }

        let piece_height = if y_cursor == solved_image.height() - last_row_height{
            last_row_height
        } else {
            piece_height
        };

        let mut x_cursor = 0;
        loop{
            if x_cursor == solved_image.width(){
                break;
            }

            let piece_width = if x_cursor == solved_image.width() - last_col_width{
                last_col_width
            } else {
                piece_width
            };

            let mut piece = DynamicImage::new_rgba8(piece_width, piece_height);
            for x in 0..piece_width{
                for y in 0..piece_height {
                    let pixel = solved_image.get_pixel(x_cursor + x, y_cursor + y);
                    piece.put_pixel(x,y,pixel);
                }
            }

            let file_path = format!("{}/piece_{}.jpg",puzzle_folder_path,file_counter);
            piece.save(file_path).expect("Failed to save image piece");
            file_counter += 1;

            x_cursor += piece_width;
        }
        y_cursor += piece_height;
    }



}
