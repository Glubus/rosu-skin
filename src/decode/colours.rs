use crate::structs::colours::Colours;
use crate::structs::common::Colour;
use ini::Ini;
use std::path::PathBuf;

impl Colours {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut colours = Colours::default();

        let conf = match Ini::load_from_file(path) {
            Ok(c) => c,
            Err(_) => return colours,
        };

        let colours_section = match conf.section(Some("Colours")) {
            Some(s) => s,
            None => return colours,
        };

        // Combo colours
        if let Some(combo) = colours_section.get("Combo1") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo1 = col;
            }
        }
        if let Some(combo) = colours_section.get("Combo2") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo2 = col;
            }
        }
        if let Some(combo) = colours_section.get("Combo3") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo3 = Some(col);
            }
        }
        if let Some(combo) = colours_section.get("Combo4") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo4 = Some(col);
            }
        }
        if let Some(combo) = colours_section.get("Combo5") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo5 = Some(col);
            }
        }
        if let Some(combo) = colours_section.get("Combo6") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo6 = Some(col);
            }
        }
        if let Some(combo) = colours_section.get("Combo7") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo7 = Some(col);
            }
        }
        if let Some(combo) = colours_section.get("Combo8") {
            if let Some(col) = Colour::from_string(combo) {
                colours.combo.combo8 = Some(col);
            }
        }

        // Other colours
        if let Some(menu_glow) = colours_section.get("MenuGlow") {
            if let Some(col) = Colour::from_string(menu_glow) {
                colours.menu_glow = col;
            }
        }

        if let Some(input_overlay_text) = colours_section.get("InputOverlayText") {
            if let Some(col) = Colour::from_string(input_overlay_text) {
                colours.input_overlay_text = col;
            }
        }

        // Slider colours
        if let Some(slider_border) = colours_section.get("SliderBorder") {
            if let Some(col) = Colour::from_string(slider_border) {
                colours.slider.border = col;
            }
        }
        if let Some(slider_track_override) = colours_section.get("SliderTrackOverride") {
            if let Some(col) = Colour::from_string(slider_track_override) {
                colours.slider.track_override = Some(col);
            }
        }
        if let Some(slider_ball) = colours_section.get("SliderBall") {
            if let Some(col) = Colour::from_string(slider_ball) {
                colours.slider.ball = col;
            }
        }

        // Song select colours
        if let Some(song_select_active_text) = colours_section.get("SongSelectActiveText") {
            if let Some(col) = Colour::from_string(song_select_active_text) {
                colours.song_select_active_text = col;
            }
        }
        if let Some(song_select_inactive_text) = colours_section.get("SongSelectInactiveText") {
            if let Some(col) = Colour::from_string(song_select_inactive_text) {
                colours.song_select_inactive_text = col;
            }
        }

        // Spinner and star colours
        if let Some(spinner_background) = colours_section.get("SpinnerBackground") {
            if let Some(col) = Colour::from_string(spinner_background) {
                colours.spinner_background = col;
            }
        }
        if let Some(star_break_additive) = colours_section.get("StarBreakAdditive") {
            if let Some(col) = Colour::from_string(star_break_additive) {
                colours.star_break_additive = col;
            }
        }

        colours
    }
}
