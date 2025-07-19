

struct General {
    name: String,
    author: Option<String>,
    version: String,
    cursor: Cursor,
    slider: Slider,
    animation_framerate: i8,
    hit_circle_overlay_above_number: bool, 
    layered_hit_sounds: bool,
    combo_burst: ComboBurst,
    spinner: Spinner,
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

struct ComboBurst
{
    burst_random: bool, // 0 = no, 1 = yes
    custom_burst_sounds: Option<u16>,
}
impl Default for ComboBurst {
    fn default() -> Self {
        Self {
            burst_random: false,
            custom_burst_sounds: None,
        }
    }
}
struct Cursor {
    centre: bool,
    expand: bool,
    rotate: bool,
    trail_rotate: bool,
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

struct Slider
{
    ball_flip: bool,
    ball_tint: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            ball_flip: true,
            ball_tint: false,
        }
    }
}

struct Spinner
{
    fade_playfield: bool,
    frequency_modulate: bool,
    no_blink: bool,
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