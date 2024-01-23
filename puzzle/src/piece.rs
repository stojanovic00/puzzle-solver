use std::{fmt, fs};
use std::collections::HashMap;
use std::path::Path;
use image::DynamicImage;

#[derive(Clone)]
pub struct Piece{
    pub index: u32,
    pub image: DynamicImage,
    pub right_neighbor: Option<u32>,
    pub bottom_neighbor: Option<u32>,
    pub file_name: String
}
impl Piece{
    pub fn new(img: DynamicImage, idx: u32, filename: String) -> Self{
        Self{
            index : idx,
            image : img,
            right_neighbor : None,
            bottom_neighbor : None,
            file_name: filename
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Piece(index: {}, right_neighbor: {:?}, bottom_neighbor: {:?})",
            self.index,  self.right_neighbor, self.bottom_neighbor
        )
    }
}

pub fn load_images_from_folder(folder_path: &str) -> Vec<Piece> {
    let mut pieces = Vec::new();
    let mut idx_counter = 0;

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();

                if file_path.is_file() && is_image_file(&file_path) {
                    if let Ok(image) = image::open(&file_path) {
                        let new_piece = Piece::new(image, idx_counter, entry.file_name().to_str().unwrap().to_string());
                        pieces.push(new_piece);
                        idx_counter += 1;
                    } else {
                        eprintln!("Failed to open image: {:?}", file_path);
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read folder: {}", folder_path);
    }

    pieces
}

fn is_image_file(file_path: &Path) -> bool {
    if let Some(extension) = file_path.extension() {
        let valid_extensions = ["jpg", "jpeg", "png", "gif", "bmp"];
        valid_extensions.iter().any(|&ext| extension == ext)
    } else {
        false
    }
}

pub fn find_most_common_dimensions(pieces: &[Piece]) -> (u32, u32){
    let mut dimension_counts: HashMap<(u32, u32), usize> = HashMap::new();

    for piece in pieces {
        let dimensions = (piece.image.width(), piece.image.height());
        *dimension_counts.entry(dimensions).or_insert(0) += 1;
    }

    let most_common_dimensions = dimension_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(dimensions, _)| dimensions);

    most_common_dimensions.unwrap()
}




















