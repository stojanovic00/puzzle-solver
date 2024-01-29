//TODO: remove when finished implementation
#![allow(dead_code)]
#![allow(unused_variables)]

mod stitching;
mod comparing;
mod piece;

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::env;
use image::{DynamicImage, GenericImage, GenericImageView};
use crate::piece::Piece;


fn main() {
    //Loading data
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <puzzle_folder_path> <solved_image_path>", args[0]);
        std::process::exit(1);
    }

    let puzzle_folder_path = &args[1];
    let solved_path = &args[2];

    let solved_image = image::open(solved_path).unwrap_or_else(|_| {
        eprintln!("Failed to open image: {:?}", solved_path);
        DynamicImage::new_rgba8(1, 1)
    });

    let mut pieces = piece::load_images_from_folder(puzzle_folder_path);

    let (usual_width, usual_height) = piece::find_most_common_dimensions(&pieces);

    let horizontal_pieces_num =  solved_image.width() / usual_width;
    let vertical_pieces_num =  solved_image.height() / usual_height;

    println!("Dims: {}x{}", usual_width, usual_height);
    println!("Horizontal pieces: {} Vertical pieces {}", horizontal_pieces_num, vertical_pieces_num);

    index_uneven_pieces(solved_image, &mut pieces, usual_width, usual_height, horizontal_pieces_num, vertical_pieces_num);

    resolve_duplicates(&mut pieces, horizontal_pieces_num, vertical_pieces_num);

    //MERGING

    // Create a HashMap to group pieces by their y values
    let mut y_groups: HashMap<Option<u32>, Vec<&Piece>> = HashMap::new();

    // Iterate over the pieces and insert them into the HashMap based on their y values
    for piece in &pieces {
        y_groups.entry(piece.y).or_insert(vec![]).push(piece);
    }

    // Sort each vector by x value
    for (_, piece_group) in y_groups.iter_mut() {
        piece_group.sort_by_key(|piece| piece.x.unwrap_or_default());
    }

    for (key, piece_group) in y_groups.iter_mut() {
        println!("{}: {}",key.unwrap(), piece_group.len());
    }

    // Create horizontal images
    let mut horizontal_pieces: Vec<Piece> = vec![];
    for (y_value, piece_group) in &y_groups {
        let mut horizontal_image = stitching::stitch_right(&piece_group[0].image, &piece_group[1].image,0);
        for idx in 2..horizontal_pieces_num - 1 {
           horizontal_image = stitching::stitch_right(&horizontal_image, &piece_group[idx as usize].image,0);
        }

        let horizontal_piece = Piece{
            index: y_value.unwrap(),
            image: horizontal_image,
            x: Some(0),
            y: Some(y_value.unwrap()),
            file_name: "temp".to_string(),
            diff: 0
        };
        horizontal_pieces.push(horizontal_piece);
    }

    horizontal_pieces.sort_by_key(|piece| piece.y.unwrap_or_default());

    //Create final image
    let mut solved_image = stitching::stitch_bottom(&horizontal_pieces[0].image, &horizontal_pieces[1].image,0);
    for idx in 2..vertical_pieces_num - 1 {
        solved_image = stitching::stitch_bottom(&solved_image, &horizontal_pieces[idx as usize].image,0);
    }

    solved_image.save("FINAL.jpg").expect("Failed to save final image.");
}

fn resolve_duplicates(pieces: &mut Vec<Piece>, horizontal_pieces_num: u32, vertical_pieces_num: u32) {
    for x in 0..horizontal_pieces_num {
        for y in 0..vertical_pieces_num {
            let mut has_coordinates: Vec<u32> = vec![];

            for piece in & *pieces {
                if piece.x.unwrap() == x && piece.y.unwrap() == y {
                    has_coordinates.push(piece.index);
                }
            }

            if has_coordinates.len() > 1 {
                //diff and idx
                let mut min_info = (u32::MAX, 0);
                for idx in &has_coordinates {
                    let piece = &pieces[*idx as usize];
                    if piece.diff < min_info.0 {
                        min_info.0 = piece.diff;
                        min_info.1 = piece.index;
                    }
                }
                for idx in &has_coordinates {
                    let piece = &mut pieces[*idx as usize];
                    if piece.index != min_info.1 {
                        piece.x = Some(1476);
                        piece.y = Some(1476);
                    }
                }
            }
        }
    }
}

fn index_uneven_pieces(solved_image: DynamicImage, pieces: &mut Vec<Piece>, usual_width: u32, usual_height: u32, horizontal_pieces_num: u32, vertical_pieces_num: u32) {
    for piece in &mut *pieces {

        // min diff, x , y
        let mut min_info = (u32::MAX, 0, 0);

        if piece.image.width() != usual_width && piece.image.height() != usual_height {
            piece.x = Some(horizontal_pieces_num - 1);
            piece.y = Some(vertical_pieces_num - 1);
            continue;
        }

        //FAR RIGHT COLUMN
        if piece.image.width() != usual_width {
            let width = solved_image.width() - (horizontal_pieces_num - 1) * usual_width;

            let mut y_cursor = 0;
            loop {
                if y_cursor == (vertical_pieces_num - 1) * usual_height {
                    break;
                }

                //Cut image from solved image
                let mut original_piece = DynamicImage::new_rgba8(width, usual_height);
                for src_x in 0..width {
                    for src_y in 0..usual_height {
                        let pixel = solved_image.get_pixel((horizontal_pieces_num - 1) * usual_width + src_x, y_cursor + src_y);
                        original_piece.put_pixel(src_x, src_y, pixel);
                    }
                }

                //Compare
                let diff = comparing::compare_pieces_hsv(&piece.image, &original_piece);
                if diff < min_info.0 {
                    min_info.0 = diff;
                    min_info.1 = horizontal_pieces_num - 1;
                    min_info.2 = y_cursor / usual_height;
                }

                y_cursor += usual_height;
            }

            //Assign minimal
            piece.x = Some(min_info.1);
            piece.y = Some(min_info.2);
            piece.diff = min_info.0;

            continue;
        }

        //BOTTOM ROW
        if piece.image.height() != usual_height {
            let height = solved_image.height() - (vertical_pieces_num - 1) * usual_height;

            let mut x_cursor = 0;
            loop {
                if x_cursor == (horizontal_pieces_num - 1) * usual_width {
                    break;
                }

                //Cut image from solved image
                let mut original_piece = DynamicImage::new_rgba8(usual_width, height);
                for src_x in 0..usual_width {
                    for src_y in 0..height {
                        let pixel = solved_image.get_pixel(x_cursor + src_x, (vertical_pieces_num - 1) * usual_height + src_y);
                        original_piece.put_pixel(src_x, src_y, pixel);
                    }
                }

                //Compare
                let diff = comparing::compare_pieces_hsv(&piece.image, &original_piece);
                if diff < min_info.0 {
                    min_info.0 = diff;
                    min_info.1 = x_cursor / usual_width;
                    min_info.2 = vertical_pieces_num - 1;
                }

                x_cursor += usual_width;
            }

            //Assign minimal
            piece.x = Some(min_info.1);
            piece.y = Some(min_info.2);


            continue;
        }


        let mut x_cursor = 0;
        loop {
            if x_cursor == (horizontal_pieces_num - 1) * usual_width {
                break;
            }

            let mut y_cursor = 0;
            loop {
                if y_cursor == (vertical_pieces_num - 1) * usual_height {
                    break;
                }

                //Cut image from solved image
                let mut original_piece = DynamicImage::new_rgba8(usual_width, usual_height);
                for src_x in 0..usual_width {
                    for src_y in 0..usual_height {
                        let pixel = solved_image.get_pixel(x_cursor + src_x, y_cursor + src_y);
                        original_piece.put_pixel(src_x, src_y, pixel);
                    }
                }

                //Compare
                let diff = comparing::compare_pieces_hsv(&piece.image, &original_piece);
                if diff < min_info.0 {
                    min_info.0 = diff;
                    min_info.1 = x_cursor / usual_width;
                    min_info.2 = y_cursor / usual_height;
                }

                y_cursor += usual_height;
            }
            x_cursor += usual_width;
        }

        //Assign minimal
        piece.x = Some(min_info.1);
        piece.y = Some(min_info.2);
    }
}






