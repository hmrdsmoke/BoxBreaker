use image::{DynamicImage, imageops::FilterType};

/// Generate a 64-bit perceptual hash from image bytes
pub fn phash_bytes(bytes: &[u8]) -> Option<u64> {
    let img = image::load_from_memory(bytes).ok()?;
    Some(phash(&img))
}

pub fn phash(img: &DynamicImage) -> u64 {
    // Resize to 32x32 grayscale
    let small = img.resize_exact(32, 32, FilterType::Lanczos3)
        .grayscale()
        .to_luma8();

    // Compute mean
    let pixels: Vec<f32> = small.pixels().map(|p| p.0[0] as f32).collect();
    let mean = pixels.iter().sum::<f32>() / pixels.len() as f32;

    // Build 64-bit hash from first 64 pixels (8x8 region)
    let mut hash: u64 = 0;
    for (i, &val) in pixels.iter().take(64).enumerate() {
        if val > mean {
            hash |= 1 << i;
        }
    }
    hash
}

/// Hamming distance between two hashes — lower = more similar
pub fn hamming(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}

/// Similarity score 0.0 to 1.0 — higher = more similar
pub fn similarity(a: u64, b: u64) -> f32 {
    1.0 - (hamming(a, b) as f32 / 64.0)
}
