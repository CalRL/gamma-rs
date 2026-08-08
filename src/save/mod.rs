pub mod beta;
pub mod properties;
pub mod version;

use crate::save::version::GameVersion;
use gvas::GvasFile;
use std::fs::File;

pub struct GammaFile(GvasFile, GameVersion);

impl GammaFile {
    pub fn test(&self) {
        
    }

    pub fn new(gvas_file: GvasFile, version: GameVersion) -> Result<GammaFile, Error> {
        Ok(GammaFile(gvas_file, version))
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
