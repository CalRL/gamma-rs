pub mod beta;
pub mod properties;
pub mod version;

use crate::save::version::GameVersion;
use gvas::GvasFile;
use std::fs::File;

pub struct GammaFile(GvasFile, GameVersion);

impl GammaFile {
    pub fn test(&self) {
        ()
    }

    pub fn new(gvas_file: GvasFile, version: GameVersion) -> Result<GammaFile, Error> {
        Ok(GammaFile {
            0: gvas_file,
            1: version,
        })
    }
    pub fn save(self, file: &mut File) -> Result<(), gvas::error::Error> {
        self.0.write(file)
    }
}

pub enum Error {
    InvalidFormat,
    InvalidVersion,
    Gvas(gvas::error::Error),
}
mod tests {
    use crate::save;
    use crate::save::GammaFile;
    use gvas::GvasFile;
    use gvas::game_version::GameVersion;
    use std::fs::File;
    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/saves/Slot1.sav");
    #[test]
    fn slot1_sav_exists() {
        assert!(File::open(PATH).is_ok())
    }

    #[test]
    fn gammafile_test() {
        let mut file = File::open(PATH).expect("save file doesnt exist");
        let gvas = GvasFile::read(&mut file, GameVersion::Default).expect("gvas");
        assert!(GammaFile::new(gvas, save::GameVersion::Beta).is_ok())
    }
}
