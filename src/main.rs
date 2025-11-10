mod decode;
mod structs;

use std::path::PathBuf;
use structs::colours::Colours;
use structs::font::Font;
use structs::general::General;
use structs::gamemode::ctb::CatchTheBeat;
use structs::gamemode::mania::Mania;

fn main() {
    // Use relative path from project root
    let path = PathBuf::from("assets/bluchoco.ini");
    
    println!("=== General ===");
    let general = General::from_path(&path);
    println!("{:#?}\n", general);
    
    println!("=== Colours ===");
    let colours = Colours::from_path(&path);
    println!("{:#?}\n", colours);
    
    println!("=== Fonts ===");
    let font = Font::from_path(&path);
    println!("{:#?}\n", font);
    
    println!("=== CatchTheBeat ===");
    let ctb = CatchTheBeat::from_path(&path);
    println!("{:#?}\n", ctb);
    
    println!("=== Mania ===");
    let mania = Mania::from_path(&path);
    println!("{:#?}\n", mania);
}
