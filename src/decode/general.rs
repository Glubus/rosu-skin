use crate::structs::general::General;
use ini::Ini;
use std::path::PathBuf;

impl General {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut general = General::default();

        let conf = match Ini::load_from_file(path) {
            Ok(c) => c,
            Err(_) => return general, // Return default if file can't be loaded
        };

        let general_section = match conf.section(Some("General")) {
            Some(s) => s,
            None => return general, // Return default if section doesn't exist
        };

        if let Some(name) = general_section.get("Name") {
            general.name = name.to_string();
        }

        if let Some(author) = general_section.get("Author") {
            general.author = Some(author.to_string());
        }

        if let Some(version) = general_section.get("Version") {
            general.version = version.to_string();
        }

        // Cursor settings
        if let Some(cursor_centre) = general_section.get("CursorCentre") {
            if let Ok(val) = cursor_centre.parse::<i8>() {
                general.cursor.centre = val == 1;
            }
        }

        if let Some(cursor_expand) = general_section.get("CursorExpand") {
            if let Ok(val) = cursor_expand.parse::<i8>() {
                general.cursor.expand = val == 1;
            }
        }

        if let Some(cursor_rotate) = general_section.get("CursorRotate") {
            if let Ok(val) = cursor_rotate.parse::<i8>() {
                general.cursor.rotate = val == 1;
            }
        }

        if let Some(cursor_trail_rotate) = general_section.get("CursorTrailRotate") {
            if let Ok(val) = cursor_trail_rotate.parse::<i8>() {
                general.cursor.trail_rotate = val == 1;
            }
        }

        // Slider settings - FIXED: these were swapped!
        if let Some(allow_slider_ball_tint) = general_section.get("AllowSliderBallTint") {
            if let Ok(val) = allow_slider_ball_tint.parse::<i8>() {
                general.slider.ball_tint = val == 1;
            }
        }

        if let Some(slider_ball_flip) = general_section.get("SliderBallFlip") {
            if let Ok(val) = slider_ball_flip.parse::<i8>() {
                general.slider.ball_flip = val == 1;
            }
        }

        // Combo burst settings
        if let Some(combo_burst_random) = general_section.get("ComboBurstRandom") {
            if let Ok(val) = combo_burst_random.parse::<i8>() {
                general.combo_burst.burst_random = val == 1;
            }
        }

        // CustomComboBurstSounds: comma-split list of positive integers
        if let Some(custom_burst_sounds) = general_section.get("CustomComboBurstSounds") {
            general.combo_burst.custom_burst_sounds = custom_burst_sounds
                .split(',')
                .map(|s| s.trim().parse::<u16>())
                .filter_map(Result::ok)
                .collect();
        }

        // Spinner settings - FIXED: use correct field names
        if let Some(fade_playfield) = general_section.get("SpinnerFadePlayfield") {
            if let Ok(val) = fade_playfield.parse::<i8>() {
                general.spinner.fade_playfield = val == 1;
            }
        }

        if let Some(frequency_modulate) = general_section.get("SpinnerFrequencyModulate") {
            if let Ok(val) = frequency_modulate.parse::<i8>() {
                general.spinner.frequency_modulate = val == 1;
            }
        }

        if let Some(no_blink) = general_section.get("SpinnerNoBlink") {
            if let Ok(val) = no_blink.parse::<i8>() {
                general.spinner.no_blink = val == 1;
            }
        }

        // Hit circle overlay - support both correct and legacy typo
        if let Some(hit_circle_overlay_above_number) =
            general_section.get("HitCircleOverlayAboveNumber")
        {
            if let Ok(val) = hit_circle_overlay_above_number.parse::<i8>() {
                general.hit_circle_overlay_above_number = val == 1;
            }
        } else if let Some(hit_circle_overlay_above_number) =
            general_section.get("HitCircleOverlayAboveNumer") // Legacy typo support
        {
            if let Ok(val) = hit_circle_overlay_above_number.parse::<i8>() {
                general.hit_circle_overlay_above_number = val == 1;
            }
        }

        if let Some(layered_hit_sounds) = general_section.get("LayeredHitSounds") {
            if let Ok(val) = layered_hit_sounds.parse::<i8>() {
                general.layered_hit_sounds = val == 1;
            }
        }

        // AnimationFramerate: can be -1 or positive integer, use i32
        if let Some(animation_framerate) = general_section.get("AnimationFramerate") {
            if let Ok(val) = animation_framerate.parse::<i32>() {
                general.animation_framerate = val;
            }
        }

        general
    }
}
