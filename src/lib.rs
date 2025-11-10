use r_resources::include_resources;

include_resources!();

pub mod decode;
pub mod structs;

// Re-export commonly used types
pub use structs::colours::Colours;
pub use structs::common::{Colour, ColourAlpha};
pub use structs::font::Font;
pub use structs::general::General;
pub use structs::gamemode::ctb::CatchTheBeat;
pub use structs::gamemode::mania::Mania;

