use crate::index::{HashIndex, MatchResult, find_matches};
use crate::condition::{grade_bytes, Grade};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub matches: Vec<MatchResult>,
    pub condition: Grade,
}

pub fn scan(image_bytes: &[u8], index: &HashIndex) -> ScanResult {
    let matches = find_matches(image_bytes, index, 3);
    let condition = grade_bytes(image_bytes).unwrap_or(Grade::NearMint);
    ScanResult { matches, condition }
}
