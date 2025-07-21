use crate::structs::common::Colour;

pub struct Colours
{
    pub combo : Combo,
    pub input_overlay_text : Colour,
    pub menu_glow: Colour,
    pub slider : Slider,
    pub song_select_active_text : Colour,
    pub song_select_inactive_text : Colour,
    pub spinner_background : Colour,
    pub star_break_additive : Colour,
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

pub struct Slider
{
    pub ball: Colour,
    pub border : Colour,
    pub track_override : Option<Colour>,
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

pub struct Combo
{
    pub combo1 : Colour,
    pub combo2 : Colour,
    pub combo3 : Option<Colour>,
    pub combo4 : Option<Colour>,
    pub combo5 : Option<Colour>,
    pub combo6 : Option<Colour>,
    pub combo7 : Option<Colour>,
    pub combo8 : Option<Colour>,
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