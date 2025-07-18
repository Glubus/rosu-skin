

struct General {
    name: String,
    author: String,
    version: String,
    cursor: Cursor,
    slider: Slider,
    animation_framerate: u8,
    hit_circle_overlay_above_number: bool, 
    layered_hit_sounds: bool,
}

struct ComboBurst
{
    burst_random: bool, // 0 = no, 1 = yes
    custom_burst_sounds: u16,
}

struct Cursor {
    centre: bool,
    expand: bool,
    rotate: bool,
    trail_rotate: bool,
}

struct Slider
{
    ball_flip: bool,
    ball_tint: bool,
}

struct Spinner
{
    fade_playfield: bool,
    frequency_modulate: bool,
    no_blink: bool,
}