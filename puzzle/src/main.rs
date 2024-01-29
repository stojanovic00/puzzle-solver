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


     let comp_thresh = 0.0;
    //TESTING

   //  let piece7 = &pieces[27];
   //  let piece6 = &pieces[26];
   //  let piece1 = &pieces[0];
   //  let edgy_image = DynamicImage::new_rgba8(1,10);
   //
   //  let difference67 = comparing::compare_right_edge_hue(&piece6.image, &piece7.image, comp_thresh) as i32;
   //  let difference61 = comparing::compare_right_edge_hue(&piece6.image, &piece1.image, comp_thresh) as i32;
   //
   //  println!("DIFF 6 and 7 RIGHT: {}", difference67);
   //  println!("DIFF 6 and 1 RIGHT: {}", difference61);
   //
   // return;
    //TESTING

    //Pre process calculations

    let (common_width, common_height) = piece::find_most_common_dimensions(&pieces);
    let horizontal_pieces_num = solved_image.width() / common_width;
    let mut vertical_pieces_num = pieces.len() as u32 / horizontal_pieces_num;

    //PROCESSING
    let mut iter_num = 0;
    let mut merged_count;
    loop {
        (pieces, merged_count) = solve(&mut pieces, &mut pieces_ro, comp_thresh, horizontal_pieces_num, vertical_pieces_num, iter_num);
        pieces_ro = pieces.clone();
        vertical_pieces_num -= merged_count;
        if pieces.len() == 0{
            break;
        }
        iter_num += 1;
    }
}

fn solve(mut pieces: &mut Vec<Piece>, mut pieces_ro: &mut Vec<Piece>, comp_thresh: f32, horizontal_pieces_num: u32, vertical_pieces_num: u32, iter_num: u32) -> (Vec<Piece>, u32) {
    loop {
        let all_indexes: Vec<usize> = (0..pieces.len()).collect();
        let taken_right_indexes: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();
        let taken_left_indexes: HashSet<_> = pieces.iter().filter_map(|piece| piece.left_neighbor).collect();

        if taken_right_indexes.len() == all_indexes.len() - vertical_pieces_num as usize {
            break;
        }

        assign_right_neighbors(&mut pieces, &mut pieces_ro, comp_thresh, &taken_right_indexes);
        assign_left_neighbors(&mut pieces, &mut pieces_ro, comp_thresh, &taken_left_indexes);

        rm_right_neighbors_from_righ_edge_pieces(&mut pieces, vertical_pieces_num);

        println!("AFTER RIGHT EDGE");
        for piece in & *pieces{
           println!("{}", piece) ;
        }

        filter_not_best_buddies_right(&mut pieces, &mut pieces_ro, comp_thresh);
        filter_not_best_buddies_left(&mut pieces, &mut pieces_ro, comp_thresh);

        println!("AFTER BEST BUDDY");
        for piece in & *pieces{
            println!("{}", piece) ;
        }
        println!("ITER END");
    }

    //FINDING LEFT EDGE PIECES
    let unreferenced_indexes = find_unreferenced_indexes(&mut pieces);


    let (used_pieces, merged_count) = merge_pieces(&mut pieces, horizontal_pieces_num, unreferenced_indexes, iter_num);

    let mut unused_pieces: Vec<Piece> = vec![];
    let mut idx_counter = 0;
    for piece in & *pieces {
        if !used_pieces.contains(&piece.index) {
            let mut refreshed_piece = piece.clone();

            refreshed_piece.index = idx_counter;
            refreshed_piece.right_neighbor = None;
            refreshed_piece.right_neighbor_diff = u32::MAX;

            refreshed_piece.left_neighbor = None;
            refreshed_piece.left_neighbor_diff = u32::MAX;

            unused_pieces.push(refreshed_piece);
            idx_counter += 1;
        }
    }
    (unused_pieces, merged_count)
}

fn merge_pieces(pieces: &mut Vec<Piece>, horizontal_pieces_num: u32, unreferenced_indexes: HashSet<u32>, iter_num: u32) -> (Vec<u32>,u32) {
    let mut used_pieces: Vec<u32> = vec![];
    //Merge all images that are in same horizontal line
    let mut file_counter = 0;
    for unref_idx in unreferenced_indexes {
        let mut locally_used_pieces: Vec<u32> = vec![];
        let start_piece = &pieces[unref_idx as usize];
        locally_used_pieces.push(unref_idx);


        if start_piece.right_neighbor.is_none() {
            continue;
        }
        let neighbor_piece_idx = start_piece.right_neighbor.unwrap();
        let mut neighbor_piece = &pieces[neighbor_piece_idx as usize];
        locally_used_pieces.push(neighbor_piece_idx);

        let mut result_image = stitching::stitch_right(&start_piece.image, &neighbor_piece.image, 0);
        let mut failed_merging = false;
        for i in 0..horizontal_pieces_num - 2 {
            if neighbor_piece.right_neighbor.is_none() {
                failed_merging = true;
                break;
            }
            let neighbor_piece_idx = neighbor_piece.right_neighbor.unwrap();
            neighbor_piece = &pieces[neighbor_piece_idx as usize];
            result_image = stitching::stitch_right(&result_image, &neighbor_piece.image, 0);
            locally_used_pieces.push(neighbor_piece_idx)
        }
        if failed_merging {
            continue;
        }
        let file_path = format!("horizontals/horizontal_{}_{}.jpg", iter_num, file_counter);
        result_image.save(file_path).expect("WRITING IMAGE FAILED");
        file_counter += 1;
        //Register what is used
        for idx in locally_used_pieces {
            used_pieces.push(idx);
        }
    }

    (used_pieces, file_counter)
}

fn rm_right_neighbors_from_righ_edge_pieces(pieces: &mut Vec<Piece>, vertical_pieces_num: u32) {
    let mut max_diff_idxs = vec![];
    let mut sorted = pieces.clone();

    sorted.sort_by(|a, b| b.right_neighbor_diff.cmp(&a.right_neighbor_diff));
    for i in 0..vertical_pieces_num {
        max_diff_idxs.push(sorted[i as usize].index);
    }

    for piece in &mut *pieces {
        if max_diff_idxs.contains(&piece.index) {
            piece.right_neighbor = None;
        }
    }
}

fn find_unreferenced_indexes(pieces: &mut Vec<Piece>) -> HashSet<u32> {
    let indexes_in_right_neighbor: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();

    let all_indexes: HashSet<_> = pieces.iter().map(|piece| piece.index).collect();

    // Find the difference to get indexes that are never in right_neighbor
    let unreferenced_indexes: HashSet<_> = all_indexes.difference(&indexes_in_right_neighbor).cloned().collect();

    unreferenced_indexes
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

fn filter_not_best_buddies_right(pieces: &mut Vec<Piece>, pieces_ro: &mut Vec<Piece>, comp_thresh: f32) {
    for piece in &mut *pieces{

       if let Some(right_neighbor_idx) = piece.right_neighbor{
            let right_neighbor = pieces_ro.get(right_neighbor_idx as usize).unwrap();

           if let Some(right_neighbors_left_neighbor) = right_neighbor.left_neighbor{
               if right_neighbors_left_neighbor != piece.index{
                   piece.right_neighbor = None;
               }
           }
           else {
               piece.right_neighbor = None;
           }

       }
    }
    // UPDATING RO
    *pieces_ro = pieces.clone();
}


fn filter_not_best_buddies_left(pieces: &mut Vec<Piece>, pieces_ro: &mut Vec<Piece>, comp_thresh: f32) {
    for piece in &mut *pieces{

        if let Some(left_neighbor_idx) = piece.left_neighbor{
            let left_neighbor = pieces_ro.get(left_neighbor_idx as usize).unwrap();

            if let Some(left_neighbors_right_neighbor) = left_neighbor.right_neighbor{
                if left_neighbors_right_neighbor != piece.index{
                    piece.left_neighbor = None;
                }
            } else {
                piece.left_neighbor = None;
            }

        }
    }
    // UPDATING RO
    *pieces_ro = pieces.clone();
}


fn assign_right_neighbors(pieces: &mut Vec<Piece>, pieces_ro: &mut Vec<Piece>, comp_thresh: f32, taken_indexes: &HashSet<u32>) {
    for piece in &mut *pieces {
        if piece.right_neighbor != None{
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
        piece.right_neighbor_diff = min_diff;
    }
    //UPDATING RO
    *pieces_ro = pieces.clone();
}
fn assign_left_neighbors(pieces: &mut Vec<Piece>, pieces_ro: &mut Vec<Piece>, comp_thresh: f32, taken_indexes: &HashSet<u32>) {
    for piece in &mut *pieces {
        if piece.left_neighbor != None{
            continue;
        }


        let mut min_diff = u32::MAX;
        //TODO check safer solution
        let mut min_index = 0;

        for comparing_piece in &mut *pieces_ro {
            if piece.index == comparing_piece.index || taken_indexes.contains(&comparing_piece.index) {
                continue;
            }

            let difference = comparing::compare_left_edge_hue(&piece.image, &comparing_piece.image, comp_thresh);
            if difference < min_diff {
                min_index = comparing_piece.index;
                min_diff = difference;
            }
        }
        piece.left_neighbor = Some(min_index);
        piece.left_neighbor_diff = min_diff;
    }
    //UPDATING RO
    *pieces_ro = pieces.clone();
}
