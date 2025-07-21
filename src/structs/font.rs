use std::path::PathBuf;

pub struct Font
{
    pub hit_circle_prefix : PathBuf,
    pub hit_circle_overlap : i32, // negative add a gap
    pub score_prefix : PathBuf,
    pub score_overlap : i32, // negative add a gap
    pub combo_prefix : PathBuf,
    pub combo_overlap : i32, // negative add a gap
}

impl Default for Font {
    fn default() -> Self {
        Self {
            hit_circle_prefix: PathBuf::from("default"),
            hit_circle_overlap: -2,
            score_prefix: PathBuf::from("score"),
            score_overlap: 0,
            combo_prefix: PathBuf::from("score"),
            combo_overlap: 0,
        }
    }
}