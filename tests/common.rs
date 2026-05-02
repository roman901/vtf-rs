use image::DynamicImage;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash(image: &DynamicImage) -> u64 {
    let mut hasher = DefaultHasher::new();
    image.to_rgba8().hash(&mut hasher);
    hasher.finish()
}
