use image::{DynamicImage, ImageResult};

pub struct FinderPattern {
    x: u32,
    y: u32,
    size: u32,
}

pub fn find_finder_patterns(_image: &DynamicImage) -> Vec<FinderPattern> {
    let mut finder_patterns = Vec::new();
    finder_patterns.push(FinderPattern {
        x: 0,
        y: 0,
        size: 0,
    });
    finder_patterns
}

pub fn img_open(path: &str) -> ImageResult<DynamicImage> {
    image::open(path)
}
