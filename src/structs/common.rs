#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Colour {
    /// Parse a colour from a string in RGB format (R, G, B)
    /// Returns None if parsing fails or if the format is invalid
    pub fn from_string(string: &str) -> Option<Self> {
        let colours: Vec<u8> = string
            .split(',')
            .map(|s| s.trim().parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        if colours.len() != 3 {
            return None;
        }

        Some(Self {
            red: colours[0],
            green: colours[1],
            blue: colours[2],
        })
    }

    /// Parse a colour from a string, accepting RGB or RGB(a) format
    /// For RGB(a), the alpha value is ignored and only RGB is returned
    pub fn from_string_rgba(string: &str) -> Option<Self> {
        let colours: Vec<u8> = string
            .split(',')
            .map(|s| s.trim().parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        if colours.len() < 3 || colours.len() > 4 {
            return None;
        }

        Some(Self {
            red: colours[0],
            green: colours[1],
            blue: colours[2],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColourAlpha {
    pub color: Colour,
    pub alpha: u8,
}

impl ColourAlpha {
    /// Parse a colour with alpha from a string in RGB(a) format
    /// If alpha is not provided, defaults to 255 (opaque)
    pub fn from_string(string: &str) -> Option<Self> {
        let colours: Vec<u8> = string
            .split(',')
            .map(|s| s.trim().parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        if colours.len() < 3 || colours.len() > 4 {
            return None;
        }

        Some(Self {
            color: Colour {
                red: colours[0],
                green: colours[1],
                blue: colours[2],
            },
            alpha: colours.get(3).copied().unwrap_or(255),
        })
    }

    /// Create a ColourAlpha from a Colour with default alpha (255)
    pub fn from_colour(colour: Colour) -> Self {
        Self {
            color: colour,
            alpha: 255,
        }
    }
}