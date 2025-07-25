use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub name: String,
    pub author: Option<String>,
    pub version: String,
    pub cursor: Cursor,
    pub slider: Slider,
    pub animation_framerate: i8,
    pub hit_circle_overlay_above_number: bool,
    pub layered_hit_sounds: bool,
    pub combo_burst: ComboBurst,
    pub spinner: Spinner,
}

impl Default for General {
    fn default() -> Self {
        Self {
            name: String::from("Unknown"),
            author: None,
            version: String::from("latest"),
            cursor: Cursor::default(),
            slider: Slider::default(),
            animation_framerate: -1,
            hit_circle_overlay_above_number: true,
            layered_hit_sounds: true,
            combo_burst: ComboBurst::default(),
            spinner: Spinner::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComboBurst {
    pub burst_random: bool, // 0 = no, 1 = yes
    pub custom_burst_sounds: Option<u16>,
}
impl Default for ComboBurst {
    fn default() -> Self {
        Self {
            burst_random: false,
            custom_burst_sounds: None,
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
            centre: true,
            expand: true,
            rotate: true,
            trail_rotate: true,
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
            ball_flip: true,
            ball_tint: false,
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
            fade_playfield: false,
            frequency_modulate: true,
            no_blink: false,
        }
    }
}
