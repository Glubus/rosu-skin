use crate::r;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Font {
    pub hit_circle_prefix: PathBuf,
    pub hit_circle_overlap: i32, // negative add a gap
    pub score_prefix: PathBuf,
    pub score_overlap: i32, // negative add a gap
    pub combo_prefix: PathBuf,
    pub combo_overlap: i32, // negative add a gap
}

impl Default for Font {
    fn default() -> Self {
        Self {
            hit_circle_prefix: PathBuf::from(r::defaults::HIT_CIRCLE_PREFIX),
            hit_circle_overlap: r::defaults::HIT_CIRCLE_OVERLAP as i32,
            score_prefix: PathBuf::from(r::defaults::SCORE_PREFIX),
            score_overlap: r::defaults::SCORE_OVERLAP as i32,
            combo_prefix: PathBuf::from(r::defaults::COMBO_PREFIX),
            combo_overlap: r::defaults::COMBO_OVERLAP as i32,
        }
    }
}
