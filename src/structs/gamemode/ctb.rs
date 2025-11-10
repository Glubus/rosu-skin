use crate::r;
use crate::structs::common::Colour;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchTheBeat {
    pub hyper_dash: Colour,
    pub hyper_dash_fruit: Option<Colour>,
    pub hyper_dash_after_image: Option<Colour>,
}

impl Default for CatchTheBeat {
    fn default() -> Self {
        Self {
            hyper_dash: Colour::from_string_rgba(r::defaults::HYPER_DASH)
                .unwrap_or(Colour { red: 255, green: 0, blue: 0 }),
            hyper_dash_fruit: None, // defaults to hyper_dash if not set
            hyper_dash_after_image: None, // defaults to hyper_dash if not set
        }
    }
}

impl CatchTheBeat {
    /// Get the hyper dash fruit colour, falling back to hyper_dash if not set
    pub fn get_hyper_dash_fruit(&self) -> Colour {
        self.hyper_dash_fruit.unwrap_or(self.hyper_dash)
    }

    /// Get the hyper dash after image colour, falling back to hyper_dash if not set
    pub fn get_hyper_dash_after_image(&self) -> Colour {
        self.hyper_dash_after_image.unwrap_or(self.hyper_dash)
    }
}
