use image::{DynamicImage, GrayImage};

#[derive(Debug, Clone, PartialEq)]
pub enum Grade {
    NearMint,
    LightlyPlayed,
    ModeratelyPlayed,
    HeavilyPlayed,
    Poor,
}

impl Grade {
    pub fn label(&self) -> &str {
        match self {
            Grade::NearMint => "Near Mint",
            Grade::LightlyPlayed => "Lightly Played",
            Grade::ModeratelyPlayed => "Moderately Played",
            Grade::HeavilyPlayed => "Heavily Played",
            Grade::Poor => "Poor",
        }
    }
}

pub fn grade_bytes(bytes: &[u8]) -> Option<Grade> {
    let img = image::load_from_memory(bytes).ok()?;
    Some(grade(&img))
}

pub fn grade(img: &DynamicImage) -> Grade {
    let gray = img.grayscale().to_luma8();
    let (w, h) = (gray.width(), gray.height());

    let corner_wear = measure_corner_wear(&gray, w, h);
    let edge_wear = measure_edge_wear(&gray, w, h);
    let surface_noise = measure_surface_noise(&gray);

    // Weighted score 0.0 (mint) to 1.0 (poor)
    let score = corner_wear * 0.4 + edge_wear * 0.4 + surface_noise * 0.2;

    match score {
        s if s < 0.10 => Grade::NearMint,
        s if s < 0.25 => Grade::LightlyPlayed,
        s if s < 0.45 => Grade::ModeratelyPlayed,
        s if s < 0.65 => Grade::HeavilyPlayed,
        _ => Grade::Poor,
    }
}

/// Measure corner wear — corners of TCG cards are dark bordered;
/// whitening/damage shows as high variance in corner regions
fn measure_corner_wear(gray: &GrayImage, w: u32, h: u32) -> f32 {
    let size = (w.min(h) / 10).max(5);
    let corners = [
        (0, 0),
        (w - size, 0),
        (0, h - size),
        (w - size, h - size),
    ];

    let mut total_variance = 0.0f32;
    for (cx, cy) in corners {
        let pixels: Vec<f32> = (cy..cy + size)
            .flat_map(|y| (cx..cx + size).map(move |x| (x, y)))
            .filter(|&(x, y)| x < w && y < h)
            .map(|(x, y)| gray.get_pixel(x, y).0[0] as f32)
            .collect();

        if pixels.is_empty() { continue; }
        let mean = pixels.iter().sum::<f32>() / pixels.len() as f32;
        let variance = pixels.iter().map(|&p| (p - mean).powi(2)).sum::<f32>()
            / pixels.len() as f32;
        total_variance += variance;
    }

    // Normalize — high variance = more wear
    (total_variance / (4.0 * 255.0_f32.powi(2))).min(1.0)
}

/// Measure edge wear — whitening along card borders
fn measure_edge_wear(gray: &GrayImage, w: u32, h: u32) -> f32 {
    let border = 3u32;
    let mut bright_count = 0u32;
    let mut total = 0u32;

    for x in 0..w {
        for y in [0..border, h-border..h] {
            for yy in y {
                if yy < h {
                    let p = gray.get_pixel(x, yy).0[0];
                    if p > 200 { bright_count += 1; }
                    total += 1;
                }
            }
        }
    }
    for y in 0..h {
        for x in [0..border, w-border..w] {
            for xx in x {
                if xx < w {
                    let p = gray.get_pixel(xx, y).0[0];
                    if p > 200 { bright_count += 1; }
                    total += 1;
                }
            }
        }
    }

    if total == 0 { return 0.0; }
    bright_count as f32 / total as f32
}

/// Measure surface noise — scratches show as local high-frequency noise
fn measure_surface_noise(gray: &GrayImage) -> f32 {
    let (w, h) = (gray.width(), gray.height());
    let mut noise_sum = 0.0f32;
    let mut count = 0u32;

    for y in 1..h-1 {
        for x in 1..w-1 {
            let center = gray.get_pixel(x, y).0[0] as f32;
            let neighbors = [
                gray.get_pixel(x-1, y).0[0] as f32,
                gray.get_pixel(x+1, y).0[0] as f32,
                gray.get_pixel(x, y-1).0[0] as f32,
                gray.get_pixel(x, y+1).0[0] as f32,
            ];
            let avg_neighbor = neighbors.iter().sum::<f32>() / 4.0;
            noise_sum += (center - avg_neighbor).abs();
            count += 1;
        }
    }

    if count == 0 { return 0.0; }
    (noise_sum / (count as f32 * 255.0)).min(1.0)
}
