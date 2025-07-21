mod structs;
mod decode;

use std::path::PathBuf;
use structs::general::General;

fn main() {
    let general = General::from_path(&PathBuf::from("F:/rosu-memory-lib/rosu-skin/assets/Cantarella.ini"));
    println!("{:?}", general);
}
