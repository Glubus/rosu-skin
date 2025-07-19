use std::path::PathBuf;

struct Font
{
    hit_circle_prefix : PathBuf,
    hit_circle_overlap : i32, // negative add a gap
    score_prefix : PathBuf,
    score_overlap : i32, // negative add a gap
    combo_prefix : PathBuf,
    combo_overlap : i32, // negative add a gap
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