use std::{fmt, fs};
use std::collections::HashMap;
use std::path::Path;
use image::DynamicImage;

#[derive(Clone)]
pub struct Piece{
    pub index: u32,
    pub image: DynamicImage,
    pub right_neighbor: Option<u32>,
    pub left_neighbor: Option<u32>,
    pub bottom_neighbor: Option<u32>,
    pub file_name: String,
    pub right_neighbor_diff: u32,
    pub left_neighbor_diff: u32
}
impl Piece{
    pub fn new(img: DynamicImage, idx: u32, filename: String) -> Self{
        Self{
            index : idx,
            image : img,
            right_neighbor : None,
            left_neighbor : None,
            bottom_neighbor : None,
            file_name: filename,
            right_neighbor_diff: u32::MAX,
            left_neighbor_diff: u32::MAX

        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Piece(index: {}, right_neighbor: {:?}, right_neighbor_diff {}, left_neighbor: {:?}, left_neighbor_diff {} bottom_neighbor: {:?}, file: {})",
            self.index, self.right_neighbor, self.right_neighbor_diff, self.left_neighbor, self.left_neighbor_diff, self.bottom_neighbor, self.file_name
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



impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.right_neighbor_diff == other.right_neighbor_diff
    }
}

impl Eq for Piece {}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.right_neighbor_diff.partial_cmp(&other.right_neighbor_diff)
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.right_neighbor_diff.cmp(&other.right_neighbor_diff)
    }
}
















