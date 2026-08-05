use crate::save::beta::r#box::row_id::RowID;
use crate::save::beta::r#box::slot_id::SlotID;
use gvas::GvasFile;

pub struct Matrix(Vec<Vec<i32>>);

pub struct MatrixMut(Vec<Vec<i32>>);

pub enum Error {
    InvalidRows,
    InvalidSlots,
}
impl Matrix {
    pub fn new(gvas_file: GvasFile, box_number: i32) -> Result<Matrix, Error> {
        let rows = RowID::new(&gvas_file, box_number).ok_or(Error::InvalidRows)?;
        let cols = SlotID::new(&gvas_file, box_number).ok_or(Error::InvalidSlots)?;

        Err(Error::InvalidSlots)
    }
}
