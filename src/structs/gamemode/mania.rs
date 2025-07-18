use crate::structs::common::{Colour, ColourAlpha};
use std::path::PathBuf;

struct Mania{


}


struct Common
{
    column_start : u8, 
    column_right : u8,
    column_spacing : Vec<u8>, 	
    column_width : Vec<u8>,
    column_line_width : Vec<u8>,
    barline_height : f32,
    lighting_n_width : Vec<u8>,
    lighting_l_width : Vec<u8>,
    width_for_note_height_scale : f32,
    hit_position : u8,
    light_position : u8,
    score_position : u8,
    combo_position : u8,
    judgment_line : bool,
    light_frame_per_second : u8,
    special_style : u8, // possible value 0,1,2
    combo_burst_style : u8, // possible value 0,1,2
    split_stage : bool,
    stage_separation : u8,
    separate_score : bool,
    keys_under_notes : bool,
    upside_down : bool,
    key_flip_when_upside_down : bool,
    key_flip_when_upside_down_notes: Vec<bool>,	
    note_flip_when_upside_down: bool,
    note_flip_when_upside_down_notes: Vec<bool>,
    note_flip_when_upside_down_notes_d: Vec<bool>,
    note_flip_when_upside_down_notes_h: Vec<bool>,
    note_flip_when_upside_down_notes_l: Vec<bool>,
    note_flip_when_upside_down_notes_t: Vec<bool>,
    note_body_style : u8, // possible value 0,1,2
    note_body_style_notes: Vec<u8>,
    colour : Vec<Colour>, // column lane colour	
    colour_lighting : Vec<Colour>, // column lane lighting colour
    colour_column_line : ColourAlpha, // column lane line colour
    colour_barline : ColourAlpha, // barline colour
    colour_judgement_line : ColourAlpha, // judgement line colour
    colour_key_warning : Colour,
    colour_hold : ColourAlpha,
    colour_break: Colour,
    key_image : Vec<PathBuf>,
    key_image_d : Vec<PathBuf>,
    note_image : Vec<PathBuf>,
    note_image_d : Vec<PathBuf>,
    note_image_h : Vec<PathBuf>,
    note_image_l : Vec<PathBuf>,
    note_image_t : Vec<PathBuf>,
    stage_left : PathBuf,
    stage_right : PathBuf,
    stage_bottom : PathBuf,
    stage_hint : PathBuf,
    stage_light : PathBuf,
    lighting_n : PathBuf,
    lighting_l : PathBuf,
    warning_arrow : PathBuf,
    hit0 : PathBuf,
    hit50 : PathBuf,
    hit100 : PathBuf,
    hit300 : PathBuf,
    hit200 : PathBuf,
    hit300g : PathBuf,
}