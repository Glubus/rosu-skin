use crate::structs::common::Colour;

struct Colours
{
    combo : Combo,
    input_overlay_text : Colour,
    menu_glow: Colour,
    slider : Slider,
    song_select_active_text : Colour,
    song_select_inactive_text : Colour,
    spinner_background : Colour,
    star_break_additive : Colour,
}

impl Default for Colours {

    fn default() -> Self {
        Self {
            combo: Combo::default(),
            input_overlay_text: Colour { red: 0, green: 0, blue: 0 },
            menu_glow: Colour { red: 0, green: 78, blue: 155 },
            slider: Slider::default(),
            song_select_active_text: Colour { red: 0, green: 0, blue: 0 },
            song_select_inactive_text: Colour { red: 255, green: 255, blue: 255 },
            spinner_background: Colour { red: 100, green: 100, blue: 100 },
            star_break_additive: Colour { red: 255, green: 182, blue: 193 },
        }
    }
}

struct Slider
{
    ball: Colour,
    border : Colour,
    track_override : Option<Colour>,
}

impl Default for Slider {

    fn default() -> Self {
        Self {
            ball: Colour { red: 2, green: 170, blue: 255 },
            border: Colour { red: 255, green: 255, blue: 255 },
            track_override: None,
        }
    }
}

struct Combo
{
    combo1 : Colour,
    combo2 : Colour,
    combo3 : Option<Colour>,
    combo4 : Option<Colour>,
    combo5 : Option<Colour>,
    combo6 : Option<Colour>,
    combo7 : Option<Colour>,
    combo8 : Option<Colour>,
}

impl Default for Combo {
    fn default() -> Self {
        Self {
            combo1: Colour { red: 255, green: 192, blue: 0 },
            combo2: Colour { red: 0, green: 202, blue: 0 },
            combo3: None,
            combo4: None,
            combo5: None,
            combo6: None,
            combo7: None,
            combo8: None,
        }
    }
}