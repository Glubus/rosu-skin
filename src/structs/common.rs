#[derive(Debug)]
pub struct Colour
{
    pub red : u8,
    pub green : u8,
    pub blue : u8,
}
impl Colour {
    pub fn from_string(string: &str) -> Self {
        let colours = string
        .split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
        .ok()
        .unwrap();
        
    // if colours.len() != 3 {
    //     return None;
    // }

        Self {
            red: colours[0],
            green: colours[1],
            blue: colours[2],
        }
    }
}

#[derive(Debug)]
pub struct ColourAlpha
{
    color: Colour,
    alpha: u8,
}