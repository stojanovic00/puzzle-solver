mod stitching;
mod comparing;
mod piece;

use std::collections::HashSet;
use image::{DynamicImage, GenericImage, GenericImageView, ImageResult};



fn main() {
    let solved_path = "solved/slika1.jpg";
    let solved_image = image::open(solved_path).unwrap_or_else(|_| {
        eprintln!("Failed to open image: {:?}", solved_path);
        DynamicImage::new_rgba8(1, 1)
    });


    let mut pieces = piece::load_images_from_folder("slika1");
    let mut pieces_ro = pieces.clone();

    //TESTING

   //  let piece3 = &pieces[5];
   //  let piece1 = &pieces[1];
   //  let piece9 = &pieces[11];
   //  let edgy_image = DynamicImage::new_rgba8(1,10);
   //
   //      let difference31 = comparing::compare_right_edge_abs(&piece3.image, &piece1.image, 110) as i32;
   //      let difference19 = comparing::compare_right_edge_abs(&piece1.image, &piece9.image, 110) as i32;
   //
   //      println!("DIFF 3 and 1: {}", difference31);
   //      println!("DIFF 1 and 9: {}", difference19);
   //      println!("DIFF19 - DIFF31: {}", difference19 - difference31);
   //
   //
   // return;
    //TESTING

    let (common_width, common_height) = piece::find_most_common_dimensions(&pieces);
    let horizontal_pieces_num = solved_image.width() / common_width;
    let vertical_pieces_num = pieces.len() as u32 / horizontal_pieces_num;


    loop {
        let all_indexes: Vec<usize> = (0..pieces.len()).collect();
        let taken_indexes: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();


        if taken_indexes.len() == all_indexes.len() {
            break;
        }

        //Initially choose right neighbors
        for piece in &mut pieces {
            if piece.right_neighbor != None {
                continue;
            }


            let mut min_diff = u32::MAX;
            //TODO check safer solution
            let mut min_index = 0;

            for comparing_piece in &pieces_ro {
                if piece.index == comparing_piece.index || taken_indexes.contains(&comparing_piece.index) {
                    continue;
                }

                let difference = comparing::compare_right_edge_euclidean(&piece.image, &comparing_piece.image);
                if difference < min_diff {
                    min_index = comparing_piece.index;
                    min_diff = difference;
                }
            }

            piece.right_neighbor = Some(min_index);
        }
        pieces_ro = pieces.clone();

        // REMOVING DOUBLE REFS

        for piece_idx in 0..pieces.len() as u32 {
            // .0 is idx of disputed piece, .1 is idx of piece that contains him as right neighbor
            let mut contain_as_neighbor: Vec<(u32, u32)> = vec![];

            for piece in &pieces_ro {
                if let Some(right_neighbor) = piece.right_neighbor {
                    if right_neighbor == piece_idx {
                        contain_as_neighbor.push((piece_idx, piece.index));
                    }
                }
            }

            if (contain_as_neighbor.len() > 1) {
                let mut min_diff = u32::MAX;
                //TODO check safer solution
                let mut min_idx = 0;

                for contestant in &contain_as_neighbor {
                    let disputed_piece = &pieces_ro[contestant.0 as usize];
                    let contestant_piece = &pieces_ro[contestant.1 as usize];

                    let difference = comparing::compare_right_edge_euclidean(&contestant_piece.image, &disputed_piece.image);

                    if difference < min_diff {
                        min_diff = difference;
                        min_idx = contestant.1;
                    }
                }

                //Contestant with min diff gets disputed piece as right neighbors and others get None
                let mut winner = &mut pieces[min_idx as usize];
                winner.right_neighbor = Some(piece_idx);
                pieces_ro = pieces.clone();

                for contestant in &contain_as_neighbor {
                    if contestant.1 == min_idx {
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
    }


    let mut threshold = 0;
    loop {
    threshold += 100;
    //Finding right edge pieces
    for piece in &pieces_ro {
        // Slide across solved edge one pixel at at a time and compare
        let mut solved_edge = DynamicImage::new_rgba8(1, piece.image.height());

        let mut one_row_nly = 0;
        if solved_image.height() - piece.image.height() == 0 {
            one_row_nly = 1;
        }

        for y_idx in 0..solved_image.height() - piece.image.height() + one_row_nly {
            let mut solved_edge_idx = 0;
            for y in y_idx..y_idx + piece.image.height() - 1 {
                let mut pixel = solved_image.get_pixel(solved_image.width() - 1, y);
                solved_edge.put_pixel(0, solved_edge_idx, pixel);
                solved_edge_idx += 1;
            }

            let mut difference = comparing::compare_right_edge_euclidean(&piece.image, &solved_edge);

            //Matching
            //TODO: SCALE THRESHOLD
            // KADA GA ISKLJUCIM ZA SLIKU 1-1 SMANJI BROJ NEREFERENCIRANIH ZA 8
            //Threshold: each channel allowed deviation is 25
            // let threshold = piece.image.height() * 75;
            if difference < threshold {
                pieces[piece.index as usize].right_neighbor = None;
                break;
            }
        }
    }

    //UPDATING RO
    pieces_ro = pieces.clone();


    //Finding pieces that are on the left edge

    // Collect all unique indexes present in right_neighbor fields
    let indexes_in_right_neighbor: HashSet<_> = pieces.iter().filter_map(|piece| piece.right_neighbor).collect();

    // Create a set of all indexes
    let all_indexes: HashSet<_> = pieces.iter().map(|piece| piece.index).collect();

    // Find the difference to get indexes that are never in right_neighbor
    let unreferenced_indexes: HashSet<_> = all_indexes.difference(&indexes_in_right_neighbor).collect();

    println!("UNREF NUM: {}", unreferenced_indexes.len());
    if unreferenced_indexes.len() as u32 == vertical_pieces_num{
        break;
    }

}

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
