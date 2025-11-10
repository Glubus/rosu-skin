use crate::structs::gamemode::mania::*;
use crate::structs::common::{Colour, ColourAlpha};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Parse a comma-separated list of numbers
fn parse_comma_list_f32(s: &str) -> Vec<f32> {
    s.split(',')
        .map(|s| s.trim().parse::<f32>())
        .filter_map(Result::ok)
        .collect()
}

/// Parse a boolean from "0" or "1"
fn parse_bool(s: &str) -> Option<bool> {
    s.parse::<i8>().ok().map(|v| v == 1)
}

/// Parse a column index from a string (0-indexed in ini files, despite spec saying 1-indexed)
fn parse_column_index(s: &str) -> Option<usize> {
    s.parse::<usize>().ok()
}

/// Parse a u8 value clamped to max value
fn parse_u8_clamped(s: &str, max: u8) -> Option<u8> {
    s.parse::<u8>().ok().filter(|&v| v <= max)
}

/// Read and parse Mania sections from file
fn parse_mania_sections(path: &PathBuf) -> HashMap<u8, HashMap<String, String>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };

    let mut key_sections: HashMap<u8, HashMap<String, String>> = HashMap::new();
    let mut current_section: Option<u8> = None;
    let mut in_mania_section = false;

    for line in content.lines() {
        let line = line.trim();

        // Check for section header
        if line.starts_with('[') && line.ends_with(']') {
            let section_name = &line[1..line.len() - 1];
            in_mania_section = section_name == "Mania";
            current_section = None;
            continue;
        }

        if !in_mania_section {
            continue;
        }

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Parse key:value pairs (osu! uses : not =)
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim();
            let value = line[colon_pos + 1..].trim();

            if key == "Keys" {
                if let Ok(keys) = value.parse::<u8>() {
                    current_section = Some(keys);
                    key_sections.entry(keys).or_insert_with(HashMap::new);
                }
            } else if let Some(keys) = current_section {
                key_sections
                    .entry(keys)
                    .or_insert_with(HashMap::new)
                    .insert(key.to_string(), value.to_string());
            }
        }
    }

    key_sections
}

/// Parse column-related settings
fn parse_column_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "ColumnStart" => {
            if let Ok(v) = value.parse::<f32>() {
                config.columns.column_start = v;
            }
        }
        "ColumnRight" => {
            if let Ok(v) = value.parse::<f32>() {
                config.columns.column_right = v;
            }
        }
        "ColumnSpacing" => {
            config.columns.column_spacing = parse_comma_list_f32(value);
        }
        "ColumnWidth" => {
            config.columns.column_width = parse_comma_list_f32(value);
        }
        "ColumnLineWidth" => {
            config.columns.column_line_width = parse_comma_list_f32(value);
        }
        "BarlineHeight" => {
            if let Ok(v) = value.parse::<f32>() {
                config.columns.barline_height = v;
            }
        }
        "LightingNWidth" => {
            config.columns.lighting_n_width = parse_comma_list_f32(value);
        }
        "LightingLWidth" => {
            config.columns.lighting_l_width = parse_comma_list_f32(value);
        }
        "WidthForNoteHeightScale" => {
            if let Ok(v) = value.parse::<f32>() {
                config.columns.width_for_note_height_scale = Some(v);
            }
        }
        _ => {}
    }
}

/// Parse position-related settings
fn parse_position_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "HitPosition" => {
            if let Ok(v) = value.parse::<i32>() {
                config.positions.hit_position = v;
            }
        }
        "LightPosition" => {
            if let Ok(v) = value.parse::<i32>() {
                config.positions.light_position = v;
            }
        }
        "ScorePosition" => {
            if let Ok(v) = value.parse::<i32>() {
                config.positions.score_position = Some(v);
            }
        }
        "ComboPosition" => {
            if let Ok(v) = value.parse::<i32>() {
                config.positions.combo_position = Some(v);
            }
        }
        "JudgementLine" => {
            config.positions.judgement_line = parse_bool(value);
        }
        _ => {}
    }
}

/// Parse style-related settings
fn parse_style_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "LightFramePerSecond" => {
            if let Ok(v) = value.parse::<i32>() {
                config.style.light_frame_per_second = Some(v);
            }
        }
        "SpecialStyle" => {
            if let Some(v) = parse_u8_clamped(value, 2) {
                config.style.special_style = v;
            }
        }
        "ComboBurstStyle" => {
            if let Some(v) = parse_u8_clamped(value, 2) {
                config.style.combo_burst_style = v;
            } else {
                // Support text values: "Left", "Right", "Both"
                match value.trim() {
                    "Left" => config.style.combo_burst_style = 0,
                    "Right" => config.style.combo_burst_style = 1,
                    "Both" => config.style.combo_burst_style = 2,
                    _ => {}
                }
            }
        }
        "NoteBodyStyle" => {
            if let Some(v) = parse_u8_clamped(value, 2) {
                config.style.note_body_style = v;
            }
        }
        key if key.starts_with("NoteBodyStyle") && key.len() > 13 => {
            if let Some(col_str) = key.strip_prefix("NoteBodyStyle") {
                if let Some(idx) = parse_column_index(col_str) {
                    if let Some(v) = parse_u8_clamped(value, 2) {
                        config.style.note_body_style_per_column.insert(idx, v);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Parse upside-down related settings
fn parse_upside_down_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "UpsideDown" => {
            config.upside_down.upside_down = parse_bool(value).unwrap_or(false);
        }
        "KeyFlipWhenUpsideDown" => {
            config.upside_down.key_flip_when_upside_down = parse_bool(value).unwrap_or(true);
        }
        key if key.starts_with("KeyFlipWhenUpsideDown") => {
            if let Some(suffix) = key.strip_prefix("KeyFlipWhenUpsideDown") {
                if suffix.is_empty() {
                    // Already handled above
                } else if suffix == "D" {
                    // Global, skip
                } else if suffix.ends_with("D") {
                    // KeyFlipWhenUpsideDown#D
                    if let Some(col_str) = suffix.strip_suffix("D") {
                        if let Some(idx) = parse_column_index(col_str) {
                            if let Some(b) = parse_bool(value) {
                                config.upside_down.key_flip_when_upside_down_d_per_column.insert(idx, b);
                            }
                        }
                    }
                } else if let Some(idx) = parse_column_index(suffix) {
                    // KeyFlipWhenUpsideDown#
                    if let Some(b) = parse_bool(value) {
                        config.upside_down.key_flip_when_upside_down_per_column.insert(idx, b);
                    }
                }
            }
        }
        "NoteFlipWhenUpsideDown" => {
            config.upside_down.note_flip_when_upside_down = parse_bool(value).unwrap_or(true);
        }
        key if key.starts_with("NoteFlipWhenUpsideDown") => {
            if let Some(suffix) = key.strip_prefix("NoteFlipWhenUpsideDown") {
                if suffix.is_empty() {
                    // Already handled above
                } else if suffix.ends_with("H") {
                    // NoteFlipWhenUpsideDown#H
                    if let Some(col_str) = suffix.strip_suffix("H") {
                        if let Some(idx) = parse_column_index(col_str) {
                            if let Some(b) = parse_bool(value) {
                                config.upside_down.note_flip_when_upside_down_h_per_column.insert(idx, b);
                            }
                        }
                    }
                } else if suffix.ends_with("L") {
                    // NoteFlipWhenUpsideDown#L
                    if let Some(col_str) = suffix.strip_suffix("L") {
                        if let Some(idx) = parse_column_index(col_str) {
                            if let Some(b) = parse_bool(value) {
                                config.upside_down.note_flip_when_upside_down_l_per_column.insert(idx, b);
                            }
                        }
                    }
                } else if suffix.ends_with("T") {
                    // NoteFlipWhenUpsideDown#T
                    if let Some(col_str) = suffix.strip_suffix("T") {
                        if let Some(idx) = parse_column_index(col_str) {
                            if let Some(b) = parse_bool(value) {
                                config.upside_down.note_flip_when_upside_down_t_per_column.insert(idx, b);
                            }
                        }
                    }
                } else if let Some(idx) = parse_column_index(suffix) {
                    // NoteFlipWhenUpsideDown#
                    if let Some(b) = parse_bool(value) {
                        config.upside_down.note_flip_when_upside_down_per_column.insert(idx, b);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Parse interface-related settings
fn parse_interface_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "SplitStages" => {
            config.interface.split_stages = parse_bool(value);
        }
        "StageSeparation" => {
            if let Ok(v) = value.parse::<f32>() {
                config.interface.stage_separation = v;
            }
        }
        "SeparateScore" => {
            config.interface.separate_score = parse_bool(value).unwrap_or(true);
        }
        "KeysUnderNotes" => {
            config.interface.keys_under_notes = parse_bool(value).unwrap_or(false);
        }
        _ => {}
    }
}

/// Parse colour-related settings
fn parse_colour_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "ColourColumnLine" => {
            if let Some(col) = ColourAlpha::from_string(value) {
                config.colours.colour_column_line = col;
            }
        }
        "ColourBarline" => {
            if let Some(col) = ColourAlpha::from_string(value) {
                config.colours.colour_barline = col;
            }
        }
        "ColourJudgementLine" => {
            if let Some(col) = Colour::from_string_rgba(value) {
                config.colours.colour_judgement_line = col;
            }
        }
        "ColourKeyWarning" => {
            if let Some(col) = Colour::from_string_rgba(value) {
                config.colours.colour_key_warning = col;
            }
        }
        "ColourHold" => {
            if let Some(col) = ColourAlpha::from_string(value) {
                config.colours.colour_hold = col;
            }
        }
        "ColourBreak" => {
            if let Some(col) = Colour::from_string_rgba(value) {
                config.colours.colour_break = col;
            }
        }
        key if key.starts_with("Colour") && key.len() > 6 => {
            if let Some(col_str) = key.strip_prefix("Colour") {
                // Skip already handled keys
                if col_str == "ColumnLine"
                    || col_str == "Barline"
                    || col_str == "JudgementLine"
                    || col_str == "KeyWarning"
                    || col_str == "Hold"
                    || col_str == "Break"
                {
                    return;
                }

                if col_str.starts_with("Light") {
                    // ColourLight# (column-specific, 1-indexed)
                    if let Some(col_idx_str) = col_str.strip_prefix("Light") {
                        if let Some(idx) = parse_column_index(col_idx_str) {
                            if let Some(col) = Colour::from_string_rgba(value) {
                                config.colours.colour_light_per_column.insert(idx, col);
                            }
                        }
                    }
                } else if let Some(idx) = parse_column_index(col_str) {
                    // Colour# (column-specific, 1-indexed)
                    if let Some(col) = ColourAlpha::from_string(value) {
                        config.colours.colour_per_column.insert(idx, col);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Parse key image settings
fn parse_key_image_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    if !key.starts_with("KeyImage") {
        return;
    }

    if key == "KeyImage" || key == "KeyImageD" {
        // Global, not supported per spec
        return;
    }

    if let Some(suffix) = key.strip_prefix("KeyImage") {
        if suffix.ends_with("D") {
            // KeyImage#D (column-specific pressed key, 1-indexed)
            if let Some(col_str) = suffix.strip_suffix("D") {
                if let Some(idx) = parse_column_index(col_str) {
                    config.key_images.key_image_d_per_column.insert(idx, PathBuf::from(value));
                }
            }
        } else if let Some(idx) = parse_column_index(suffix) {
            // KeyImage# (column-specific, 1-indexed)
            config.key_images.key_image_per_column.insert(idx, PathBuf::from(value));
        }
    }
}

/// Parse note image settings
fn parse_note_image_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    if !key.starts_with("NoteImage") {
        return;
    }

    if key == "NoteImage" || key == "NoteImageH" || key == "NoteImageL" || key == "NoteImageT" {
        // Global, not supported
        return;
    }

    if let Some(suffix) = key.strip_prefix("NoteImage") {
        if suffix.ends_with("H") {
            // NoteImage#H (column-specific head, 1-indexed)
            if let Some(col_str) = suffix.strip_suffix("H") {
                if let Some(idx) = parse_column_index(col_str) {
                    config.note_images.note_image_h_per_column.insert(idx, PathBuf::from(value));
                }
            }
        } else if suffix.ends_with("L") {
            // NoteImage#L (column-specific body, 1-indexed)
            if let Some(col_str) = suffix.strip_suffix("L") {
                if let Some(idx) = parse_column_index(col_str) {
                    config.note_images.note_image_l_per_column.insert(idx, PathBuf::from(value));
                }
            }
        } else if suffix.ends_with("T") {
            // NoteImage#T (column-specific tail, 1-indexed)
            if let Some(col_str) = suffix.strip_suffix("T") {
                if let Some(idx) = parse_column_index(col_str) {
                    config.note_images.note_image_t_per_column.insert(idx, PathBuf::from(value));
                }
            }
        } else if let Some(idx) = parse_column_index(suffix) {
            // NoteImage# (column-specific, 1-indexed)
            config.note_images.note_image_per_column.insert(idx, PathBuf::from(value));
        }
    }
}

/// Parse stage image settings
fn parse_stage_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "StageLeft" => config.stage.stage_left = Some(PathBuf::from(value)),
        "StageRight" => config.stage.stage_right = Some(PathBuf::from(value)),
        "StageBottom" => config.stage.stage_bottom = Some(PathBuf::from(value)),
        "StageHint" => config.stage.stage_hint = Some(PathBuf::from(value)),
        "StageLight" => config.stage.stage_light = Some(PathBuf::from(value)),
        _ => {}
    }
}

/// Parse visual effects settings
fn parse_visual_effects_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "LightingN" => config.visual_effects.lighting_n = Some(PathBuf::from(value)),
        "LightingL" => config.visual_effects.lighting_l = Some(PathBuf::from(value)),
        "WarningArrow" => config.visual_effects.warning_arrow = Some(PathBuf::from(value)),
        _ => {}
    }
}

/// Parse hit effect settings
fn parse_hit_effect_settings(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    match key {
        "Hit0" => config.hit_effects.hit0 = Some(PathBuf::from(value)),
        "Hit50" => config.hit_effects.hit50 = Some(PathBuf::from(value)),
        "Hit100" => config.hit_effects.hit100 = Some(PathBuf::from(value)),
        "Hit200" => config.hit_effects.hit200 = Some(PathBuf::from(value)),
        "Hit300" => config.hit_effects.hit300 = Some(PathBuf::from(value)),
        "Hit300g" => config.hit_effects.hit300g = Some(PathBuf::from(value)),
        _ => {}
    }
}

/// Parse a single property into the config
fn parse_property(key: &str, value: &str, config: &mut ManiaKeyConfig) {
    // Skip Keys as it's already handled
    if key == "Keys" {
        if let Ok(k) = value.parse::<u8>() {
            config.keys = k;
        }
        return;
    }

    // Try each category
    parse_column_settings(key, value, config);
    parse_position_settings(key, value, config);
    parse_style_settings(key, value, config);
    parse_upside_down_settings(key, value, config);
    parse_interface_settings(key, value, config);
    parse_colour_settings(key, value, config);
    parse_key_image_settings(key, value, config);
    parse_note_image_settings(key, value, config);
    parse_stage_settings(key, value, config);
    parse_visual_effects_settings(key, value, config);
    parse_hit_effect_settings(key, value, config);
}

impl Mania {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut mania = Mania::default();
        let key_sections = parse_mania_sections(path);

        // Parse each keycount configuration
        for (keys, props) in key_sections {
            let mut config = ManiaKeyConfig::default();
            config.keys = keys;

            // Parse all properties
            for (key, value) in &props {
                parse_property(key, value, &mut config);
            }

            mania.key_configs.insert(keys, config);
        }

        mania
    }
}
