mod stitching;
mod comparing;
mod piece;

use std::collections::HashSet;
use image::{DynamicImage, GenericImage, GenericImageView, ImageResult};



fn main() {
    let solved_path =  "solved/slika1.jpg";

    let solved_image = image::open(solved_path).unwrap_or_else(|_| {
        eprintln!("Failed to open image: {:?}", solved_path);
        DynamicImage::new_rgba8(1, 1)
    });



    // let mut pieces = piece::load_images_from_folder("jedan_red");
    // let mut pieces = piece::load_images_from_folder("slika1");
    let mut pieces = piece::load_images_from_folder("slika1");
    let mut pieces_ro = pieces.clone();

    let (common_width, common_height)  = piece::find_most_common_dimensions(&pieces);

    let horizontal_pieces_num = solved_image.width() / common_width;

    //Initially choose right neighbors
    for piece in &mut pieces{
        let mut min_diff = u32::MAX;
        //TODO check safer solution
        let mut min_index = 0;

        for comparing_piece in &pieces_ro{
            if piece.index == comparing_piece.index{
                continue;
            }

            let difference = comparing::compare_right_edge(&piece.image, &comparing_piece.image);
            if difference < min_diff{
                min_index = comparing_piece.index;
                min_diff = difference;
            }
        }

        piece.right_neighbor = Some(min_index);

    }

    for piece in &pieces{
        println!("{}", piece);
    }

    // REMOVING DOUBLE REFS

    pieces_ro = pieces.clone();



    for piece_idx in 0..pieces.len() as u32{
        // .0 is idx of disputed piece, .1 is idx of piece that contains him as right neighbor
        let mut contain_as_neighbor : Vec<(u32,u32)> = vec![];

        for piece in &pieces_ro{
            if let Some(right_neighbor) = piece.right_neighbor {
                if right_neighbor == piece_idx {
                    contain_as_neighbor.push((piece_idx, piece.index));
                }
            }
        }

        if(contain_as_neighbor.len() > 1){
            let mut min_diff = u32::MAX;
            //TODO check safer solution
            let mut min_idx = 0;

            for contestant in &contain_as_neighbor{
                let disputed_piece = &pieces_ro[contestant.0 as usize];
                let contestant_piece = &pieces_ro[contestant.1 as usize];

                let difference = comparing::compare_right_edge(&contestant_piece.image, &disputed_piece.image);

                if difference < min_diff{
                    min_diff = difference;
                    min_idx = contestant.1;
                }
            }

            //Contestant with min diff gets disputed piece as right neighbors and others get None
            let mut winner = &mut pieces[min_idx as usize];
            winner.right_neighbor = Some(piece_idx);
            pieces_ro = pieces.clone();

            for contestant in &contain_as_neighbor{
                if contestant.1 == min_idx{
                    continue;
                }
                let mut loser = &mut pieces[contestant.1 as usize];
                loser.right_neighbor = None;
                pieces_ro = pieces.clone();
            }
        }

    }

    //UPDATING RO
    pieces_ro = pieces.clone();

    println!("AFTER CONFLICT RESOLVING");
    for piece in &pieces{
        println!("{}", piece);
    }

    //Finding right edge pieces
    for piece in &pieces_ro{
        // Slide across solved edge one pixel at at a time and compare
        let mut solved_edge = DynamicImage::new_rgba8(1, piece.image.height());

        for y_idx in 0..solved_image.height() - piece.image.height(){
            let mut solved_edge_idx = 0;
            for y in y_idx..y_idx + piece.image.height() - 1{
                let mut pixel = solved_image.get_pixel(solved_image.width() - 1, y);
                solved_edge.put_pixel(0, solved_edge_idx, pixel);
                solved_edge_idx += 1;
            }

            let mut difference = comparing::compare_right_edge(&piece.image, &solved_edge);

            //Matching
            //TODO: SCALE THRESHOLD
            //Threshold: each channel allowed deviation is 25
            let threshold = piece.image.height() * 75;
            if difference < threshold{
                pieces[piece.index as usize].right_neighbor = None;
                break;
            }
        }
    }

    //UPDATING RO
    pieces_ro = pieces.clone();

    println!("AFTER RIGHT EDGE PIECES FINDING");
    for piece in &pieces{
        println!("{}", piece);
    }



    //Finding pieces that are on the left edge

    // Collect all unique indexes present in right_neighbor fields
    let indexes_in_right_neighbor: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();

    // Create a set of all indexes
    let all_indexes: HashSet<_> = pieces.iter().map(|piece| piece.index).collect();

    // Find the difference to get indexes that are never in right_neighbor
    let unreferenced_indexes: HashSet<_> = all_indexes.difference(&indexes_in_right_neighbor).collect();

    // Now 'unreferenced_indexes' contains all indexes never present in right_neighbor
    for index in &unreferenced_indexes {
        println!("Unreferenced Index: {}", index);
    }

    //Merge all images that are in same horizontal line
    let mut file_counter = 0;
    for unref_idx in unreferenced_indexes{
       let start_piece = &pieces[*unref_idx as usize];
       let mut neighbor_piece = &pieces[start_piece.right_neighbor.unwrap() as usize];

        let mut result_image = stitching::stitch_right(&start_piece.image, &neighbor_piece.image, 0);
        for i in 0..horizontal_pieces_num -2{
            neighbor_piece = &pieces[neighbor_piece.right_neighbor.unwrap() as usize];
            result_image = stitching::stitch_right(&result_image, &neighbor_piece.image, 0);
        }
        let file_path = format!("horizontals/horizontal_{}.jpg", file_counter);
        result_image.save(file_path).expect("WRITING IMAGE FAILED");
        file_counter += 1;
    }

}


// REDOSLED: C A Y Z -> 1 0 2 3


// ***** LAST ROW MANUAL *****
// let piece1 = image::open("jedan_red/part_c.jpg").expect("Failed to open");
// let piece2 = image::open("jedan_red/part_a.jpg").expect("Failed to open");
// let piece3 = image::open("jedan_red/part_y.jpg").expect("Failed to open");
// let piece4 = image::open("jedan_red/part_z.jpg").expect("Failed to open");
//
// let mut final_result = stitching::stitch_right(&piece1, &piece2, 0);
//  final_result = stitching::stitch_right(&final_result, &piece3, 0);
//  final_result = stitching::stitch_right(&final_result, &piece4, 0);

// final_result.save("final.jpg").expect("FAILED");

// **** LAST ROW RIGHT EDGE COMPARISON
// let diff12 = comparing::compare_right_edge(&piece1, &piece2);
// let diff13 = comparing::compare_right_edge(&piece1, &piece3);
// let diff14 = comparing::compare_right_edge(&piece1, &piece4);
// println!("Diff 1 and 2: {}", diff12);
// println!("Diff 1 and 3: {}", diff13);
// println!("Diff 1 and 3: {}", diff14);
// println!();
//
//
// let diff21 = comparing::compare_right_edge(&piece2, &piece1);
// let diff23 = comparing::compare_right_edge(&piece2, &piece3);
// let diff24 = comparing::compare_right_edge(&piece2, &piece4);
// println!("Diff 2 and 1: {}", diff21);
// println!("Diff 2 and 3: {}", diff23);
// println!("Diff 2 and 4: {}", diff24);
// println!();
//
//
// let diff31 = comparing::compare_right_edge(&piece3, &piece1);
// let diff32 = comparing::compare_right_edge(&piece3, &piece2);
// let diff34 = comparing::compare_right_edge(&piece3, &piece4);
// println!("Diff 3 and 1: {}", diff31);
// println!("Diff 3 and 2: {}", diff32);
// println!("Diff 3 and 4: {}", diff34);
// println!();



//*****RESOLVE CONFLICTS WITH PIECES WITH SAME NEIGHBOR******
//This also decides what is an edge piece
//TODO: READONLY VECTOR NEEDS TO BE UPDATED

// for piece_idx in 0..horizontal_pieces_num{
//  .0 is idx of disputed piece, .1 is idx of piece that contains him as right neighbor
// let mut contain_as_neighbor : Vec<(u32,u32)> = vec![];
//
// for piece in &pieces_ro{
// if let Some(right_neighbor) = piece.right_neighbor {
// if right_neighbor == piece_idx {
// contain_as_neighbor.push((piece_idx, piece.index));
// }
// }
// }
//
// if(contain_as_neighbor.len() > 1){
// let mut min_diff = u32::MAX;
// //TODO check safer solution
// let mut min_idx = 0;
//
// for contestant in &contain_as_neighbor{
// let disputed_piece = &pieces_ro[contestant.0 as usize];
// let contestant_piece = &pieces_ro[contestant.1 as usize];
//
// let difference = comparing::compare_right_edge(&contestant_piece.image, &disputed_piece.image);
//
// if difference < min_diff{
// min_diff = difference;
// min_idx = contestant.1;
// }
// }
//
// //Contestant with min diff gets disputed piece as right neighbors and others get None
// let mut winner = &mut pieces[min_idx as usize];
// winner.right_neighbor = Some(piece_idx);
// pieces_ro = pieces.clone();
//
// for contestant in &contain_as_neighbor{
// if contestant.1 == min_idx{
// continue;
// }
// let mut loser = &mut pieces[contestant.1 as usize];
// loser.right_neighbor = None;
// pieces_ro = pieces.clone();
// }
// }
//
// }
//
// println!("AFTER CONFLICT RESOLVING");
// for piece in &pieces{
// println!("{}", piece);
// }
