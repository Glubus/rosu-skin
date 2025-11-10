use crate::structs::gamemode::ctb::CatchTheBeat;
use crate::structs::common::Colour;
use crate::r;
use ini::Ini;
use std::path::PathBuf;

impl CatchTheBeat {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut ctb = CatchTheBeat::default();

        let conf = match Ini::load_from_file(path) {
            Ok(c) => c,
            Err(_) => return ctb,
        };

        let ctb_section = match conf.section(Some(r::sections::CATCH_THE_BEAT)) {
            Some(s) => s,
            None => return ctb,
        };

        // HyperDash (required, has default)
        if let Some(hyper_dash) = ctb_section.get(r::ctb_keys::HYPER_DASH) {
            if let Some(col) = Colour::from_string_rgba(hyper_dash) {
                ctb.hyper_dash = col;
            }
        }

        // HyperDashFruit (optional, defaults to HyperDash)
        if let Some(hyper_dash_fruit) = ctb_section.get(r::ctb_keys::HYPER_DASH_FRUIT) {
            if let Some(col) = Colour::from_string_rgba(hyper_dash_fruit) {
                ctb.hyper_dash_fruit = Some(col);
            }
        }

        // HyperDashAfterImage (optional, defaults to HyperDash)
        if let Some(hyper_dash_after_image) = ctb_section.get(r::ctb_keys::HYPER_DASH_AFTER_IMAGE) {
            if let Some(col) = Colour::from_string_rgba(hyper_dash_after_image) {
                ctb.hyper_dash_after_image = Some(col);
            }
        }

        ctb
    }
}

