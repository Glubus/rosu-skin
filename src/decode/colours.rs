use crate::structs::colours::Colours;
use ini::Ini;
use std::path::PathBuf;
use crate::structs::common::Colour;

impl Colours {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut colours = Colours::default();

        let conf = Ini::load_from_file(path).unwrap();
        let colours_section = conf.section(Some("Colours")).unwrap();

        if let Some(combo) = colours_section.get("Combo1") {
            colours.combo.combo1 = Colour::from_string(combo);
        }
        if let Some(combo) = colours_section.get("Combo2") {
            colours.combo.combo2 = Colour::from_string(combo);
        }
        if let Some(combo) = colours_section.get("Combo3") {
            colours.combo.combo3 = Some(Colour::from_string(combo));
        }
        if let Some(combo) = colours_section.get("Combo4") {
            colours.combo.combo4 = Some(Colour::from_string(combo));
        }
        if let Some(combo) = colours_section.get("Combo5") {
            colours.combo.combo5 = Some(Colour::from_string(combo));
        }
        if let Some(combo) = colours_section.get("Combo6") {
            colours.combo.combo6 = Some(Colour::from_string(combo));
        }
        if let Some(combo) = colours_section.get("Combo7") {
            colours.combo.combo7 = Some(Colour::from_string(combo));
        }
        if let Some(combo) = colours_section.get("Combo8") {
            colours.combo.combo8 = Some(Colour::from_string(combo));
        }


        if let Some(menu_glow) = colours_section.get("MenuGlow") {
            colours.menu_glow = Colour::from_string(menu_glow);
        }

        if let Some(input_overlay_text) = colours_section.get("InputOverlayText") {
            colours.input_overlay_text = Colour::from_string(input_overlay_text);
        }

        if let Some(slider_border) = colours_section.get("SliderBorder") {
            colours.slider.border = Colour::from_string(slider_border);
        }
        if let Some(slider_track_override) = colours_section.get("SliderTrackOverride") {
            colours.slider.track_override = Some(Colour::from_string(slider_track_override));
        }
        if let Some(slider_ball) = colours_section.get("SliderBall") {
            colours.slider.ball = Colour::from_string(slider_ball);
        }

        if let Some(song_select_active_text) = colours_section.get("SongSelectActiveText") {
            colours.song_select_active_text = Colour::from_string(song_select_active_text);
        }
        if let Some(song_select_inactive_text) = colours_section.get("SongSelectInactiveText") {
            colours.song_select_inactive_text = Colour::from_string(song_select_inactive_text);
        }
        if let Some(spinner_background) = colours_section.get("SpinnerBackground") {
            colours.spinner_background = Colour::from_string(spinner_background);
        }
        if let Some(star_break_additive) = colours_section.get("StarBreakAdditive") {
            colours.star_break_additive = Colour::from_string(star_break_additive);
        }


       colours
    }
}