use crate::structs::common::{Colour, ColourAlpha};
use std::collections::HashMap;
use std::path::PathBuf;

/// Mania skin configuration, containing configurations for different keycounts
#[derive(Debug, Clone, PartialEq)]
pub struct Mania {
    /// Map of keycount (1-18) to configuration
    pub key_configs: HashMap<u8, ManiaKeyConfig>,
}

impl Default for Mania {
    fn default() -> Self {
        Self {
            key_configs: HashMap::new(),
        }
    }
}

/// Configuration for a specific keycount
#[derive(Debug, Clone, PartialEq)]
pub struct ManiaKeyConfig {
    /// The keycount this configuration is for (required)
    pub keys: u8,
    pub columns: Column,
    pub positions: Position,
    pub style: Style,
    pub upside_down: UpsideDown,
    pub interface: Interface,
    pub colours: ColourMania,
    pub key_images: KeyImage,
    pub note_images: NoteImage,
    pub stage: Stage,
    pub visual_effects: VisualEffects,
    pub hit_effects: HitEffect,
}

impl Default for ManiaKeyConfig {
    fn default() -> Self {
        Self {
            keys: 4, // Default to 4K
            columns: Column::default(),
            positions: Position::default(),
            style: Style::default(),
            upside_down: UpsideDown::default(),
            interface: Interface::default(),
            colours: ColourMania::default(),
            key_images: KeyImage::default(),
            note_images: NoteImage::default(),
            stage: Stage::default(),
            visual_effects: VisualEffects::default(),
            hit_effects: HitEffect::default(),
        }
    }
}

/// Configuration des colonnes et de leur disposition
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub column_start: f32,
    pub column_right: f32,
    pub column_spacing: Vec<f32>,
    pub column_width: Vec<f32>,
    pub column_line_width: Vec<f32>,
    pub barline_height: f32,
    pub lighting_n_width: Vec<f32>,
    pub lighting_l_width: Vec<f32>,
    pub width_for_note_height_scale: Option<f32>,
}

impl Default for Column {
    fn default() -> Self {
        Self {
            column_start: 136.0,
            column_right: 19.0,
            column_spacing: Vec::new(),
            column_width: Vec::new(),
            column_line_width: Vec::new(),
            barline_height: 1.2,
            lighting_n_width: Vec::new(),
            lighting_l_width: Vec::new(),
            width_for_note_height_scale: None,
        }
    }
}

/// Configuration des positions des éléments d'interface
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub hit_position: i32,
    pub light_position: i32,
    pub score_position: Option<i32>,
    pub combo_position: Option<i32>,
    pub judgement_line: Option<bool>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            hit_position: 402,
            light_position: 413,
            score_position: None,
            combo_position: None,
            judgement_line: None,
        }
    }
}

/// Configuration du style et des animations
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub light_frame_per_second: Option<i32>,
    pub special_style: u8, // 0, 1, or 2
    pub combo_burst_style: u8, // 0, 1, or 2
    pub note_body_style: u8, // 0, 1, or 2 (default for all columns)
    pub note_body_style_per_column: HashMap<usize, u8>, // Column-specific (0-indexed)
}

impl Default for Style {
    fn default() -> Self {
        Self {
            light_frame_per_second: None,
            special_style: 0,
            combo_burst_style: 1,
            note_body_style: 1,
            note_body_style_per_column: HashMap::new(),
        }
    }
}

/// Configuration de l'affichage en mode inversé
#[derive(Debug, Clone, PartialEq)]
pub struct UpsideDown {
    pub upside_down: bool,
    pub key_flip_when_upside_down: bool,
    pub key_flip_when_upside_down_per_column: HashMap<usize, bool>, // Column-specific (0-indexed)
    pub key_flip_when_upside_down_d_per_column: HashMap<usize, bool>, // Column-specific pressed key (0-indexed)
    pub note_flip_when_upside_down: bool,
    pub note_flip_when_upside_down_per_column: HashMap<usize, bool>, // Column-specific (0-indexed)
    pub note_flip_when_upside_down_h_per_column: HashMap<usize, bool>, // Column-specific head (0-indexed)
    pub note_flip_when_upside_down_l_per_column: HashMap<usize, bool>, // Column-specific body (0-indexed)
    pub note_flip_when_upside_down_t_per_column: HashMap<usize, bool>, // Column-specific tail (0-indexed)
}

impl Default for UpsideDown {
    fn default() -> Self {
        Self {
            upside_down: false,
            key_flip_when_upside_down: true,
            key_flip_when_upside_down_per_column: HashMap::new(),
            key_flip_when_upside_down_d_per_column: HashMap::new(),
            note_flip_when_upside_down: true,
            note_flip_when_upside_down_per_column: HashMap::new(),
            note_flip_when_upside_down_h_per_column: HashMap::new(),
            note_flip_when_upside_down_l_per_column: HashMap::new(),
            note_flip_when_upside_down_t_per_column: HashMap::new(),
        }
    }
}

/// Configuration de l'interface de jeu
#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    pub split_stages: Option<bool>,
    pub stage_separation: f32,
    pub separate_score: bool,
    pub keys_under_notes: bool,
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            split_stages: None,
            stage_separation: 40.0,
            separate_score: true,
            keys_under_notes: false,
        }
    }
}

/// Configuration des couleurs
#[derive(Debug, Clone, PartialEq)]
pub struct ColourMania {
    pub colour_per_column: HashMap<usize, ColourAlpha>, // Column-specific (1-indexed in ini, 0-indexed here)
    pub colour_light_per_column: HashMap<usize, Colour>, // Column-specific (1-indexed in ini, 0-indexed here)
    pub colour_column_line: ColourAlpha,
    pub colour_barline: ColourAlpha,
    pub colour_judgement_line: Colour,
    pub colour_key_warning: Colour,
    pub colour_hold: ColourAlpha,
    pub colour_break: Colour,
}

impl Default for ColourMania {
    fn default() -> Self {
        Self {
            colour_per_column: HashMap::new(),
            colour_light_per_column: HashMap::new(),
            colour_column_line: ColourAlpha {
                color: Colour { red: 255, green: 255, blue: 255 },
                alpha: 255,
            },
            colour_barline: ColourAlpha {
                color: Colour { red: 255, green: 255, blue: 255 },
                alpha: 255,
            },
            colour_judgement_line: Colour { red: 255, green: 255, blue: 255 },
            colour_key_warning: Colour { red: 0, green: 0, blue: 0 },
            colour_hold: ColourAlpha {
                color: Colour { red: 255, green: 191, blue: 51 },
                alpha: 255,
            },
            colour_break: Colour { red: 255, green: 0, blue: 0 },
        }
    }
}

/// Configuration des images des touches
#[derive(Debug, Clone, PartialEq)]
pub struct KeyImage {
    pub key_image_per_column: HashMap<usize, PathBuf>, // Column-specific (0-indexed)
    pub key_image_d_per_column: HashMap<usize, PathBuf>, // Column-specific pressed key (0-indexed)
}

impl Default for KeyImage {
    fn default() -> Self {
        Self {
            key_image_per_column: HashMap::new(),
            key_image_d_per_column: HashMap::new(),
        }
    }
}

/// Configuration des images des notes
#[derive(Debug, Clone, PartialEq)]
pub struct NoteImage {
    pub note_image_per_column: HashMap<usize, PathBuf>, // Column-specific (0-indexed)
    pub note_image_h_per_column: HashMap<usize, PathBuf>, // Column-specific head (0-indexed)
    pub note_image_l_per_column: HashMap<usize, PathBuf>, // Column-specific body (0-indexed)
    pub note_image_t_per_column: HashMap<usize, PathBuf>, // Column-specific tail (0-indexed)
}

impl Default for NoteImage {
    fn default() -> Self {
        Self {
            note_image_per_column: HashMap::new(),
            note_image_h_per_column: HashMap::new(),
            note_image_l_per_column: HashMap::new(),
            note_image_t_per_column: HashMap::new(),
        }
    }
}

/// Configuration des éléments de scène
#[derive(Debug, Clone, PartialEq)]
pub struct Stage {
    pub stage_left: Option<PathBuf>,
    pub stage_right: Option<PathBuf>,
    pub stage_bottom: Option<PathBuf>,
    pub stage_hint: Option<PathBuf>,
    pub stage_light: Option<PathBuf>,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            stage_left: None,
            stage_right: None,
            stage_bottom: None,
            stage_hint: None,
            stage_light: None,
        }
    }
}

/// Configuration des effets visuels
#[derive(Debug, Clone, PartialEq)]
pub struct VisualEffects {
    pub lighting_n: Option<PathBuf>,
    pub lighting_l: Option<PathBuf>,
    pub warning_arrow: Option<PathBuf>,
}

impl Default for VisualEffects {
    fn default() -> Self {
        Self {
            lighting_n: None,
            lighting_l: None,
            warning_arrow: None,
        }
    }
}

/// Configuration des effets de hit
#[derive(Debug, Clone, PartialEq)]
pub struct HitEffect {
    pub hit0: Option<PathBuf>,
    pub hit50: Option<PathBuf>,
    pub hit100: Option<PathBuf>,
    pub hit200: Option<PathBuf>,
    pub hit300: Option<PathBuf>,
    pub hit300g: Option<PathBuf>,
}

impl Default for HitEffect {
    fn default() -> Self {
        Self {
            hit0: None,
            hit50: None,
            hit100: None,
            hit200: None,
            hit300: None,
            hit300g: None,
        }
    }
}
