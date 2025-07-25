mod decode;
mod structs;

use std::path::PathBuf;
use structs::colours::Colours;
use structs::font::Font;
use structs::general::General;

fn main() {
    let general = General::from_path(&PathBuf::from(
        "F:/rosu-memory-lib/rosu-skin/assets/Cantarella.ini",
    ));
    println!("{:#?}", general);
    let colours = Colours::from_path(&PathBuf::from(
        "F:/rosu-memory-lib/rosu-skin/assets/Cantarella.ini",
    ));
    println!("{:#?}", colours);
    let font = Font::from_path(&PathBuf::from(
        "F:/rosu-memory-lib/rosu-skin/assets/Cantarella.ini",
    ));
    println!("{:#?}", font);
}
