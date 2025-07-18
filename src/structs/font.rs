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