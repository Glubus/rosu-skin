pub struct Colour
{
    pub red : u8,
    pub green : u8,
    pub blue : u8,
}
impl Colour {
    pub fn from_string(string: &str) -> Self {
        let colours = string.split(",").collect::<Vec<&str>>();
        Self {
            red: colours[0].parse::<u8>().unwrap(),
            green: colours[1].parse::<u8>().unwrap(),
            blue: colours[2].parse::<u8>().unwrap(),
        }
    }
}

pub struct ColourAlpha
{
    color: Colour,
    alpha: u8,
}