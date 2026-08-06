use gvas::GvasFile;
use gvas::game_version::GameVersion;
use std::fs::File;

const SLOT1_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/saves/Slot1.sav");

pub fn load_slot1() -> GvasFile {
    let mut file = File::open(SLOT1_PATH).expect("save file exists");
    GvasFile::read(&mut file, GameVersion::Default).expect("gvas file reads")
}
