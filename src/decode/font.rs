use crate::structs::font::Font;
use ini::Ini;
use std::path::PathBuf;

impl Font {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut font = Font::default();

        let conf = match Ini::load_from_file(path) {
            Ok(c) => c,
            Err(_) => return font,
        };

        let font_section = match conf.section(Some("Fonts")) {
            Some(s) => s,
            None => return font,
        };

        if let Some(hit_circle_prefix) = font_section.get("HitCirclePrefix") {
            font.hit_circle_prefix = PathBuf::from(hit_circle_prefix);
        }

        if let Some(hit_circle_overlap) = font_section.get("HitCircleOverlap") {
            if let Ok(val) = hit_circle_overlap.parse::<i32>() {
                font.hit_circle_overlap = val;
            }
        }

        if let Some(score_prefix) = font_section.get("ScorePrefix") {
            font.score_prefix = PathBuf::from(score_prefix.replace('\\', "/"));
        }

        if let Some(score_overlap) = font_section.get("ScoreOverlap") {
            if let Ok(val) = score_overlap.parse::<i32>() {
                font.score_overlap = val;
            }
        }

        if let Some(combo_prefix) = font_section.get("ComboPrefix") {
            font.combo_prefix = PathBuf::from(combo_prefix);
        }

        if let Some(combo_overlap) = font_section.get("ComboOverlap") {
            if let Ok(val) = combo_overlap.parse::<i32>() {
                font.combo_overlap = val;
            }
        }

        font
    }
}
