use crate::structs::gamemode::ctb::CatchTheBeat;
use crate::structs::common::Colour;
use ini::Ini;
use std::path::PathBuf;

impl CatchTheBeat {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut ctb = CatchTheBeat::default();

        let conf = match Ini::load_from_file(path) {
            Ok(c) => c,
            Err(_) => return ctb,
        };

        let ctb_section = match conf.section(Some("CatchTheBeat")) {
            Some(s) => s,
            None => return ctb,
        };

        // HyperDash (required, has default)
        if let Some(hyper_dash) = ctb_section.get("HyperDash") {
            if let Some(col) = Colour::from_string_rgba(hyper_dash) {
                ctb.hyper_dash = col;
            }
        }

        // HyperDashFruit (optional, defaults to HyperDash)
        if let Some(hyper_dash_fruit) = ctb_section.get("HyperDashFruit") {
            if let Some(col) = Colour::from_string_rgba(hyper_dash_fruit) {
                ctb.hyper_dash_fruit = Some(col);
            }
        }

        // HyperDashAfterImage (optional, defaults to HyperDash)
        if let Some(hyper_dash_after_image) = ctb_section.get("HyperDashAfterImage") {
            if let Some(col) = Colour::from_string_rgba(hyper_dash_after_image) {
                ctb.hyper_dash_after_image = Some(col);
            }
        }

        ctb
    }
}

