use crate::r;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub name: String,
    pub author: Option<String>,
    pub version: String,
    pub cursor: Cursor,
    pub slider: Slider,
    pub animation_framerate: i32,
    pub hit_circle_overlay_above_number: bool,
    pub layered_hit_sounds: bool,
    pub combo_burst: ComboBurst,
    pub spinner: Spinner,
}

impl Default for General {
    fn default() -> Self {
        Self {
            name: String::from(r::defaults::SKIN_NAME),
            author: None,
            version: String::from(r::defaults::SKIN_VERSION),
            cursor: Cursor::default(),
            slider: Slider::default(),
            animation_framerate: r::defaults::ANIMATION_FRAMERATE as i32,
            hit_circle_overlay_above_number: r::defaults::HIT_CIRCLE_OVERLAY_ABOVE_NUMBER,
            layered_hit_sounds: r::defaults::LAYERED_HIT_SOUNDS,
            combo_burst: ComboBurst::default(),
            spinner: Spinner::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComboBurst {
    pub burst_random: bool, // 0 = no, 1 = yes
    pub custom_burst_sounds: Vec<u16>, // comma-split list of positive integers
}
impl Default for ComboBurst {
    fn default() -> Self {
        Self {
            burst_random: r::defaults::COMBO_BURST_RANDOM,
            custom_burst_sounds: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cursor {
    pub centre: bool,
    pub expand: bool,
    pub rotate: bool,
    pub trail_rotate: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            centre: r::defaults::CURSOR_CENTRE,
            expand: r::defaults::CURSOR_EXPAND,
            rotate: r::defaults::CURSOR_ROTATE,
            trail_rotate: r::defaults::CURSOR_TRAIL_ROTATE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slider {
    pub ball_flip: bool,
    pub ball_tint: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            ball_flip: r::defaults::SLIDER_BALL_FLIP,
            ball_tint: r::defaults::SLIDER_BALL_TINT,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spinner {
    pub fade_playfield: bool,
    pub frequency_modulate: bool,
    pub no_blink: bool,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            fade_playfield: r::defaults::SPINNER_FADE_PLAYFIELD,
            frequency_modulate: r::defaults::SPINNER_FREQUENCY_MODULATE,
            no_blink: r::defaults::SPINNER_NO_BLINK,
        }
    }
}
