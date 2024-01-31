//Aleksandar Stojanović E2119/2023
mod stitching;
mod comparing;
mod piece;

use std::collections::{HashMap};
use std::{env, thread};
use std::path::Path;
use std::sync::{Arc, RwLock};
use image::{DynamicImage, GenericImage, GenericImageView };
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

    //Obtain solved image name without extension
    let solved_path = Path::new(solved_path);

    // Extract the file name (if any)
    let solved_image_name = if let Some(file_name) = solved_path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            file_name_str.split(".").next().unwrap_or(file_name_str)
        } else {
            println!("Unable to convert file name to string");
            ""
        }
    } else {
        println!("No file name found in the path");
        ""
    };

    //TODO: parallelize
    let mut pieces = piece::load_images_from_folder(puzzle_folder_path);
    //Processing

    let (usual_width, usual_height) = piece::find_most_common_dimensions(&pieces);

    let horizontal_pieces_num =  solved_image.width() / usual_width;
    let  vertical_pieces_num =  solved_image.height() / usual_height;


    //TODO: parallelize
    index_pieces(&solved_image, &mut pieces, usual_width, usual_height, horizontal_pieces_num, vertical_pieces_num);

    //TODO: parallelize if possible
    resolve_unassigned(solved_image, &mut pieces, usual_width, usual_height, horizontal_pieces_num, vertical_pieces_num);

    //TODO: parallel horizontal merging
    merge_pieces(&mut pieces, horizontal_pieces_num, vertical_pieces_num, solved_image_name.to_string());
}

fn resolve_unassigned(solved_image: DynamicImage, mut pieces: &mut Vec<Piece>, usual_width: u32, usual_height: u32, horizontal_pieces_num: u32, vertical_pieces_num: u32) {
    loop {
        let mut unassigned_pieces: Vec<Piece> = pieces
            .iter()
            .filter(|piece| piece.x.is_none() && piece.y.is_none()).cloned()
            .collect();

        if unassigned_pieces.len() == 0 {
            break;
        }

        let unassigned_coords = find_unassigned_coordinates(&mut pieces, usual_width, usual_height, horizontal_pieces_num, vertical_pieces_num);

        //Assign left over pieces and coordinates
        for coordinate in &unassigned_coords {


            //resolving piece size depending on coordinates
            let og_piece_width = if coordinate.0 < (horizontal_pieces_num - 1) {
                usual_width
            } else {
                solved_image.width() - ((horizontal_pieces_num - 1) * usual_width)
            };

            let og_piece_height = if coordinate.1 < (vertical_pieces_num - 1) {
                usual_height
            } else {
                solved_image.height() - ((vertical_pieces_num - 1) * usual_height)
            };

            //Cut image from solved image
            let mut original_piece = DynamicImage::new_rgba8(og_piece_width, og_piece_height);
            for x in 0..usual_width {
                for y in 0..usual_height {
                    let pixel = solved_image.get_pixel(coordinate.0 + x, coordinate.1 + y);
                    original_piece.put_pixel(x, y, pixel);
                }
            }

            let mut min_diff = u32::MAX;
            let mut min_index = 0;
            for piece in &unassigned_pieces {
                let diff = comparing::compare_pieces_rgb(&piece.image, &original_piece);
                if diff < min_diff {
                    min_diff = diff;
                    min_index = piece.index;
                }
            }


            let winner_piece = &mut pieces[min_index as usize];
            winner_piece.x = Some(coordinate.0);
            winner_piece.y = Some(coordinate.1);

            unassigned_pieces.retain(|piece| piece.index != winner_piece.index);
        }
    }
}

fn find_unassigned_coordinates(pieces: &mut Vec<Piece>, usual_width: u32, usual_height: u32, horizontal_pieces_num: u32, vertical_pieces_num: u32) -> Vec<(u32,u32)> {
//Check even sub-matrix
    let mut unassigned_coordinates: Vec<(u32, u32)> = vec![];

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

            if !pieces.iter().any(|piece| piece.x == Some(x_cursor / usual_width) && piece.y == Some(y_cursor / usual_height)) {
                unassigned_coordinates.push((x_cursor / usual_width, y_cursor / usual_height));
            }


            y_cursor += usual_height;
        }
        x_cursor += usual_width;
    }


    //Far right column
    let mut y_cursor = 0;
    loop {
        if y_cursor == (vertical_pieces_num - 1) * usual_height {
            break;
        }

        if !pieces.iter().any(|piece| piece.x == Some(horizontal_pieces_num - 1) && piece.y == Some(y_cursor / usual_height)) {
            unassigned_coordinates.push((horizontal_pieces_num - 1, y_cursor / usual_height));
        }


        y_cursor += usual_height;
    }

    //Bottom column
    let mut x_cursor = 0;
    loop {
        if x_cursor == (horizontal_pieces_num - 1) * usual_width {
            break;
        }

        if !pieces.iter().any(|piece| piece.x == Some(x_cursor / usual_width) && piece.y == Some(vertical_pieces_num - 1 )) {
            unassigned_coordinates.push((x_cursor / usual_width, vertical_pieces_num - 1 ));
        }

        x_cursor += usual_width;
    }

    unassigned_coordinates
}

fn merge_pieces(pieces: &mut Vec<Piece>, horizontal_pieces_num: u32, vertical_pieces_num: u32, image_name: String) {
// Create a HashMap to group pieces by their y values
    let mut y_groups: HashMap<Option<u32>, Vec<&Piece>> = HashMap::new();

    // Iterate over the pieces and insert them into the HashMap based on their y values
    for piece in & *pieces {
        y_groups.entry(piece.y).or_insert(vec![]).push(piece);
    }

    // Sort each vector by x value
    for (_, piece_group) in y_groups.iter_mut() {
        piece_group.sort_by_key(|piece| piece.x.unwrap_or_default());
    }

    // Create horizontal images
    let mut horizontal_pieces: Vec<Piece> = vec![];
    for (y_value, piece_group) in &y_groups {
        if piece_group.len() > 1{
            let mut horizontal_image = stitching::stitch_right(&piece_group[0].image, &piece_group[1].image, 0);
            for idx in 2..horizontal_pieces_num - 1 {
                horizontal_image = stitching::stitch_right(&horizontal_image, &piece_group[idx as usize].image, 0);
            }

            let horizontal_piece = Piece {
                index: y_value.unwrap(),
                image: horizontal_image,
                x: Some(0),
                y: Some(y_value.unwrap()),
                file_name: "temp".to_string()
            };
            horizontal_pieces.push(horizontal_piece);
        } else{
            horizontal_pieces.push(piece_group[0].clone())
        }
    }

    horizontal_pieces.sort_by_key(|piece| piece.y.unwrap_or_default());

    let image_path = format!("solved/{}_solved.jpg", image_name);
    //Create final image
    if horizontal_pieces.len() > 1{
        let mut solved_image= stitching::stitch_bottom(&horizontal_pieces[0].image, &horizontal_pieces[1].image, 0);
        for idx in 2..vertical_pieces_num - 1 {
            solved_image = stitching::stitch_bottom(&solved_image, &horizontal_pieces[idx as usize].image, 0);
        }
        solved_image.save(image_path).expect("Failed to save final image.");
    } else{
        horizontal_pieces[0].image.save(image_path).expect("Failed to save final image.");
    }
}


fn index_pieces(solved_image: &DynamicImage, pieces: &mut Vec<Piece>, usual_width: u32, usual_height: u32, horizontal_pieces_num: u32, vertical_pieces_num: u32) {

    //There are 3 loops for even matrix, far right column and bottom row
    //Each loop will be parallelized separately

    let core_num = num_cpus::get() as u32;
    println!("Cores: {}", core_num);

    //PREPARE THREAD DATA
    let longer_dim = horizontal_pieces_num.max(vertical_pieces_num) - 1;

    let thread_num = longer_dim.min(core_num);
    println!("Threads needed: {}", thread_num);

    //Each thread gets even amount of jobs and remainder is spread evenly across
    let even_jobs_num = longer_dim / thread_num;
    let mut jobs: Vec<u32> = vec![even_jobs_num; thread_num as usize];
    let leftover_jobs_num = longer_dim % thread_num;
    for idx in 0..leftover_jobs_num {
        jobs[idx as usize] += 1;
    }

    //TODO: PROMENI OVO KAD SVE ZAVRSIS ALOOOUUUUU AAAAAAAAAAAAAAAA
    let pieces_temp = pieces.clone();
    let solved_image_temp = solved_image.clone();
    let pieces_guarded = Arc::new(RwLock::new(pieces_temp));
    let solved_image_guarded = Arc::new(RwLock::new(solved_image_temp));



    //EVEN PIECES MATRIX
    let mut thread_handles = vec![];
    for i in 0..thread_num{
        let thread_horizontal_pieces_num;
        let thread_vertical_pieces_num;
        let mut offset = 0;
        let partition_horizontal;

        if horizontal_pieces_num > vertical_pieces_num{
            thread_horizontal_pieces_num = jobs[i as usize];
            //minus 1 because this is just even size pieces matrix
            thread_vertical_pieces_num = vertical_pieces_num - 1;
            partition_horizontal = true;
        } else{
            thread_horizontal_pieces_num = horizontal_pieces_num - 1;
            thread_vertical_pieces_num = jobs[i as usize];
            partition_horizontal = false;
        }

        for k in 0..i{
            offset += jobs[k as usize];
        }


        let pieces = Arc::clone(&pieces_guarded);
        let solved_image = Arc::clone(&solved_image_guarded);

        //SPAWNING THREADS
        let handle = thread::spawn(move || {
            //THREAD
            // println!("Horizontal: {}, Vertical: {}, offset: {}, partition horizontal: {}", thread_horizontal_pieces_num, thread_vertical_pieces_num, offset, partition_horizontal);
            //TODO OVO SE MENJA SRB
            let mut x_cursor = if partition_horizontal{
                offset * usual_width
            } else{
                0
            };

            loop{
                //TODO OVO SE MENJA SRB
                if partition_horizontal && x_cursor == (offset + thread_horizontal_pieces_num)* usual_width {
                    break;
                } else if x_cursor == thread_horizontal_pieces_num * usual_width{
                    break;
                }


                //TODO OVO SE MENJA SRB
                let mut y_cursor = if !partition_horizontal{
                    offset * usual_height
                } else{
                    0
                };

                //TODO OVO SE MENJA SRB
                loop {
                    if !partition_horizontal && y_cursor == (offset + thread_vertical_pieces_num) * usual_height{
                        break;
                    } else if y_cursor == thread_vertical_pieces_num * usual_height{
                        break;
                    }

                    //Cut image from solved image
                    let mut original_piece = DynamicImage::new_rgba8(usual_width, usual_height);
                    for x in 0..usual_width {
                        for y in 0..usual_height {
                            //TODO PROVERI OVO AAAAAAAAAA
                            let pixel = solved_image.read().unwrap().get_pixel(x_cursor + x, y_cursor + y);
                            original_piece.put_pixel(x, y, pixel);
                        }
                    }

                    //Find best piece for it
                    let mut min_diff = u32::MAX;
                    let mut min_idx = 0;
                    //TODO PROVERI OVO AAAAAAAAAA
                    for piece in & *pieces.read().unwrap(){
                        let diff = comparing::compare_pieces_rgb(&original_piece, &piece.image);
                        if diff < min_diff{
                            min_diff = diff;
                            min_idx = piece.index;
                        }
                    }


                    //Assign coordinates of OG piece to best piece candidate
                    //TODO PROVERI OVO AAAAAAAAAA
                    let winner_piece = &mut pieces.write().unwrap()[min_idx as usize];
                    winner_piece.x = Some(x_cursor/usual_width);
                    winner_piece.y = Some(y_cursor/usual_height);

                    y_cursor += usual_height;
                }
                x_cursor += usual_width;
            }


           //THREAD
        });

        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }


    //REVOKE PIECES TO MAIN THREAD
    //TODO SREDI OVO DA SE NE KLONIRA 1247 puta
    let mut pieces = pieces_guarded.read().unwrap().clone();

    println!("SRB");




    //Get marginal width and size
    let end_width = solved_image.width() - (horizontal_pieces_num - 1) * usual_width;
    let end_height = solved_image.height() - (vertical_pieces_num- 1) * usual_height;


    //Far right column
    let mut y_cursor = 0;
    loop {
        if y_cursor == (vertical_pieces_num - 1) * usual_height {
            break;
        }

        //Cut image from solved image
        let mut original_piece = DynamicImage::new_rgba8(end_width, usual_height);
        for x in 0..end_width {
            for y in 0..usual_height {
                let pixel = solved_image.get_pixel((horizontal_pieces_num - 1) * usual_width + x, y_cursor + y);
                original_piece.put_pixel(x, y, pixel);
            }
        }

        //Find best piece for it
        let mut min_diff = u32::MAX;
        let mut min_idx = 0;
        for piece in &mut *pieces{
            let diff = comparing::compare_pieces_rgb(&original_piece, &piece.image);
            if diff < min_diff{
                min_diff = diff;
                min_idx = piece.index;
            }
        }

        //Assign coordinates of OG piece to best piece candidate
        let winner_piece = &mut pieces[min_idx as usize];
        winner_piece.x = Some(horizontal_pieces_num - 1);
        winner_piece.y = Some(y_cursor/usual_height);

        y_cursor += usual_height;
    }


    //Bottom column
    let mut x_cursor = 0;
    loop {
        if x_cursor == (horizontal_pieces_num - 1) * usual_width {
            break;
        }

        //Cut image from solved image
        let mut original_piece = DynamicImage::new_rgba8(usual_width, end_height);
        for x in 0..usual_width {
            for y in 0..end_height {
                let pixel = solved_image.get_pixel(x_cursor + x, (vertical_pieces_num - 1) * usual_height + y);
                original_piece.put_pixel(x, y, pixel);
            }
        }

        //Find best piece for it
        let mut min_diff = u32::MAX;
        let mut min_idx = 0;
        for piece in &mut *pieces{
            let diff = comparing::compare_pieces_rgb(&original_piece, &piece.image);
            if diff < min_diff{
                min_diff = diff;
                min_idx = piece.index;
            }
        }

        //Assign coordinates of OG piece to best piece candidate
        let winner_piece = &mut pieces[min_idx as usize];
        winner_piece.x = Some(x_cursor/usual_width);
        winner_piece.y = Some(vertical_pieces_num - 1);

        x_cursor += usual_width;
    }

    //Piece in bottom right corner

    //Cut image from solved image
    let mut original_piece = DynamicImage::new_rgba8(end_width, end_height);
    for x in 0..end_width {
        for y in 0..end_height {
            let pixel = solved_image.get_pixel((horizontal_pieces_num - 1) * usual_width + x, (vertical_pieces_num - 1) * usual_height + y);
            original_piece.put_pixel(x, y, pixel);
        }
    }

    //Find best piece for it
    let mut min_diff = u32::MAX;
    let mut min_idx = 0;
    for piece in &mut *pieces{
        let diff = comparing::compare_pieces_rgb(&original_piece, &piece.image);
        if diff < min_diff{
            min_diff = diff;
            min_idx = piece.index;
        }
    }

    //Assign coordinates of OG piece to best piece candidate
    let winner_piece = &mut pieces[min_idx as usize];
    winner_piece.x = Some(horizontal_pieces_num - 1);
    winner_piece.y = Some(vertical_pieces_num - 1);
}