//TODO: remove when finished implementation
#![allow(dead_code)]
#![allow(unused_variables)]

mod stitching;
mod comparing;
mod piece;

use std::collections::HashSet;
use std::env;
use image::{DynamicImage};
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
    //UPDATING RO
    let mut pieces_ro = pieces.clone();

    //TESTING

    let piece7 = &pieces[9];
    let piece8 = &pieces[10];
    let piece1 = &pieces[0];
    let edgy_image = DynamicImage::new_rgba8(1,10);

    let comp_thresh = 0.0;
    let difference78 = comparing::compare_right_edge_hue(&piece7.image, &piece8.image, comp_thresh) as i32;
    let difference81 = comparing::compare_right_edge_hue(&piece8.image, &piece1.image, comp_thresh) as i32;

    println!("DIFF 7 and 8 RIGHT: {}", difference78);
    println!("DIFF 8 and 1 RIGHT: {}", difference81);

    let difference87_left = comparing::compare_left_edge_hue(&piece8.image, &piece7.image, comp_thresh) as i32;
    let difference18_left = comparing::compare_left_edge_hue(&piece1.image, &piece8.image, comp_thresh) as i32;

    println!("DIFF 7 and 8 LEFT: {}", difference87_left);
    println!("DIFF 8 and 1 LEFT: {}", difference18_left);
   return;
    //TESTING

    //Pre process calculations

    let (common_width, common_height) = piece::find_most_common_dimensions(&pieces);
    let horizontal_pieces_num = solved_image.width() / common_width;
    let vertical_pieces_num = pieces.len() as u32 / horizontal_pieces_num;

    //Processing
    loop {
        let all_indexes: Vec<usize> = (0..pieces.len()).collect();
        let taken_indexes: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();

        if taken_indexes.len() == all_indexes.len() {
            break;
        }

        assign_right_neighbors(&mut pieces, &mut pieces_ro, comp_thresh, taken_indexes);

        resolve_neighboring_conflicts(&mut pieces, &mut pieces_ro, comp_thresh);
    }


    //FINDING RIGHT EDGE PIECES
    let mut max_diff_idxs = vec![];
    let mut sorted = pieces.clone();

    sorted.sort_by(|a, b| b.neighbor_diff.cmp(&a.neighbor_diff));
    for i in 0..vertical_pieces_num{
        max_diff_idxs.push(sorted[i as usize].index);
    }

    for piece in &mut pieces{
        if max_diff_idxs.contains(&piece.index){
            piece.right_neighbor = None;
        }
    }

    //UPDATING RO
    // pieces_ro = pieces.clone(); not necessary

    for piece in &pieces{
        println!("{}", piece);
    }

    //FINDING LEFT EDGE PIECES

    // Collect all unique indexes present in right_neighbor fields
    let indexes_in_right_neighbor: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();

    // Create a set of all indexes
    let all_indexes: HashSet<_> = pieces.iter().map(|piece| piece.index).collect();

    // Find the difference to get indexes that are never in right_neighbor
    let unreferenced_indexes: HashSet<_> = all_indexes.difference(&indexes_in_right_neighbor).collect();



    //MERGING PIECES
    // Collect all unique indexes present in right_neighbor fields
    let indexes_in_right_neighbor: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();

    // Create a set of all indexes
    let all_indexes: HashSet<_> = pieces.iter().map(|piece| piece.index).collect();

    // Find the difference to get indexes that are never in right_neighbor
    let unreferenced_indexes: HashSet<_> = all_indexes.difference(&indexes_in_right_neighbor).collect();


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





fn resolve_neighboring_conflicts(pieces: &mut Vec<Piece>, pieces_ro: &mut Vec<Piece>, comp_thresh: f32) {
// REMOVING DOUBLE REFS

    for piece_idx in 0..pieces.len() as u32 {
        // .0 is idx of disputed piece, .1 is idx of piece that contains him as right neighbor
        let mut contain_as_neighbor: Vec<(u32, u32)> = vec![];

        for piece in &mut *pieces_ro {
            if let Some(right_neighbor) = piece.right_neighbor {
                if right_neighbor == piece_idx {
                    contain_as_neighbor.push((piece_idx, piece.index));
                }
            }
        }

        if contain_as_neighbor.len() > 1 {
            let mut min_diff = u32::MAX;
            //TODO check safer solution
            let mut min_idx = 0;

            for contestant in &contain_as_neighbor {
                let disputed_piece = &pieces_ro[contestant.0 as usize];
                let contestant_piece = &pieces_ro[contestant.1 as usize];

                let difference = comparing::compare_right_edge_hue(&contestant_piece.image, &disputed_piece.image, comp_thresh);

                if difference < min_diff {
                    min_diff = difference;
                    min_idx = contestant.1;
                }
            }

            //Contestant with min diff gets disputed piece as right neighbors and others get None
            let winner = &mut pieces[min_idx as usize];
            winner.right_neighbor = Some(piece_idx);
            //UPDATING RO
            *pieces_ro = pieces.clone();

            for contestant in &contain_as_neighbor {
                if contestant.1 == min_idx {
                    continue;
                }
                let loser = &mut pieces[contestant.1 as usize];
                loser.right_neighbor = None;
                //UPDATING RO
                *pieces_ro = pieces.clone();
            }
        }
    }

    //UPDATING RO
    *pieces_ro = pieces.clone();
}

fn assign_right_neighbors(pieces: &mut Vec<Piece>, pieces_ro: &mut Vec<Piece>, comp_thresh: f32, taken_indexes: HashSet<u32>) {
    for piece in &mut *pieces {
        if piece.right_neighbor != None {
            continue;
        }


        let mut min_diff = u32::MAX;
        //TODO check safer solution
        let mut min_index = 0;

        for comparing_piece in &mut *pieces_ro {
            if piece.index == comparing_piece.index || taken_indexes.contains(&comparing_piece.index) {
                continue;
            }

            let difference = comparing::compare_right_edge_hue(&piece.image, &comparing_piece.image, comp_thresh);
            if difference < min_diff {
                min_index = comparing_piece.index;
                min_diff = difference;
            }
        }
        piece.right_neighbor = Some(min_index);
        piece.neighbor_diff = min_diff;
    }
    //UPDATING RO
    *pieces_ro = pieces.clone();
}
