use crate::save::beta::StorageType;
use crate::save::beta::row_id::RowID;
use crate::save::beta::slot_id::SlotID;
use gvas::GvasFile;

pub struct Matrix(Vec<Vec<i32>>);

pub struct MatrixMut(Vec<Vec<i32>>);

pub enum Error {
    InvalidRows,
    InvalidSlots,
}
impl Matrix {
    pub fn new(gvas_file: GvasFile, box_number: i32) -> Result<Matrix, Error> {
        let _rows =
            RowID::new(&gvas_file, StorageType::BOXES(box_number)).ok_or(Error::InvalidRows)?;
        let _cols =
            SlotID::new(&gvas_file, StorageType::BOXES(box_number)).ok_or(Error::InvalidSlots)?;

        Err(Error::InvalidSlots)
    }
}
