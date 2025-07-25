use crate::structs::font::Font;
use ini::Ini;
use std::path::PathBuf;

impl Font {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut font = Font::default();

        let conf = Ini::load_from_file(path).unwrap();
        let font_section = conf.section(Some("Fonts")).unwrap();

        if let Some(hit_circle_prefix) = font_section.get("HitCirclePrefix") {
            font.hit_circle_prefix = PathBuf::from(hit_circle_prefix);
        }
        if let Some(hit_circle_overlap) = font_section.get("HitCircleOverlap") {
            font.hit_circle_overlap = hit_circle_overlap.parse::<i32>().unwrap();
        }
        if let Some(score_prefix) = font_section.get("ScorePrefix") {
            font.score_prefix = PathBuf::from(score_prefix.replace('\\', "/"));
            println!("{:?}", font.score_prefix);
        }
        if let Some(score_overlap) = font_section.get("ScoreOverlap") {
            font.score_overlap = score_overlap.parse::<i32>().unwrap();
        }
        if let Some(combo_prefix) = font_section.get("ComboPrefix") {
            font.combo_prefix = PathBuf::from(combo_prefix);
        }
        if let Some(combo_overlap) = font_section.get("ComboOverlap") {
            font.combo_overlap = combo_overlap.parse::<i32>().unwrap();
        }

        font
    }
}
