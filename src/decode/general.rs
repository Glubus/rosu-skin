use crate::structs::general::General;
use ini::Ini;
use std::path::PathBuf;

impl General {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut general = General::default();

        let conf = Ini::load_from_file(path).unwrap();
        let general_section = conf.section(Some("General")).unwrap();

        if let Some(name) = general_section.get("Name") {
            general.name = general_section.get("Name").unwrap().to_string();
        }
        if let Some(author) = general_section.get("Author") {
            general.author = Some(author.to_string());
        }
        if let Some(version) = general_section.get("Version") {
            general.version = version.to_string();
        }
        if let Some(cursor_centre) = general_section.get("CursorCentre") {
            general.cursor.centre = cursor_centre.parse::<i8>().unwrap() == 1;
        }
        if let Some(cursor_expand) = general_section.get("CursorExpand") {
            general.cursor.expand = cursor_expand.parse::<i8>().unwrap() == 1;
        }
        if let Some(cursor_rotate) = general_section.get("CursorRotate") {
            general.cursor.rotate = cursor_rotate.parse::<i8>().unwrap() == 1;
        }
        if let Some(cursor_trail_rotate) = general_section.get("CursorTrailRotate") {
            general.cursor.trail_rotate = cursor_trail_rotate.parse::<i8>().unwrap() == 1;
        }
        if let Some(allow_slider_ball_tint) = general_section.get("AllowSliderBallTint") {
            general.slider.ball_flip = allow_slider_ball_tint.parse::<i8>().unwrap() == 1;
        }
        if let Some(slider_ball_flip) = general_section.get("SliderBallFlip") {
            general.slider.ball_tint = slider_ball_flip.parse::<i8>().unwrap() == 1;
        }
        if let Some(combo_burst_random) = general_section.get("ComboBurstRandom") {
            general.combo_burst.burst_random = combo_burst_random.parse::<i8>().unwrap() == 1;
        }
        if let Some(custom_burst_sounds) = general_section.get("CustomBurstSounds") {
            general.combo_burst.custom_burst_sounds =
                Some(custom_burst_sounds.to_string().parse::<u16>().unwrap());
        }
        if let Some(fade_playfield) = general_section.get("FadePlayfield") {
            general.spinner.fade_playfield = fade_playfield.parse::<i8>().unwrap() == 1;
        }
        if let Some(frequency_modulate) = general_section.get("FrequencyModulate") {
            general.spinner.frequency_modulate = frequency_modulate.parse::<i8>().unwrap() == 1;
        }
        if let Some(no_blink) = general_section.get("NoBlink") {
            general.spinner.no_blink = no_blink.parse::<i8>().unwrap() == 1;
        }
        if let Some(hit_circle_overlay_above_number) =
            general_section.get("HitCircleOverlayAboveNumber")
        {
            general.hit_circle_overlay_above_number =
                hit_circle_overlay_above_number.parse::<i8>().unwrap() == 1;
        }
        if let Some(layered_hit_sounds) = general_section.get("LayeredHitSounds") {
            general.layered_hit_sounds = layered_hit_sounds.parse::<i8>().unwrap() == 1;
        }
        if let Some(animation_framerate) = general_section.get("AnimationFramerate") {
            general.animation_framerate = animation_framerate.parse::<i8>().unwrap();
        }

        general
    }
}
