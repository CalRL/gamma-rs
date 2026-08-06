use crate::save::beta::boxes::row_id::RowID;
use crate::save::beta::boxes::slot_id::SlotID;
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxPosition {
    pub row: i32,
    pub slot: i32,
}

pub enum Error {
    OutOfBounds,
    NoRows,
    NoSlots,
    InvalidRowArray,
    InvalidSlotArray,
}
impl BoxPosition {
    pub fn new(row: i32, slot: i32) -> Result<BoxPosition, Error> {
        if !(0..3).contains(&row) || !(0..6).contains(&slot) {
            return Err(Error::OutOfBounds);
        }

        Ok(BoxPosition { row, slot })
    }

    pub fn exists_at(&self, gvas_file: &GvasFile, box_number: i32) -> Result<bool, Error> {
        let row_arr = RowID::new(gvas_file, box_number).ok_or(Error::NoRows)?;

        let slot_arr = SlotID::new(gvas_file, box_number).ok_or(Error::NoSlots)?;

        let row_prop = row_arr.as_list().ok_or(Error::InvalidRowArray)?;

        let slot_prop = slot_arr.as_list().ok_or(Error::InvalidSlotArray)?;

        let ArrayProperty::Ints { ints: row_ints } = &row_prop else {
            return Err(Error::InvalidRowArray);
        };

        let ArrayProperty::Ints { ints: slot_ints } = &slot_prop else {
            return Err(Error::InvalidSlotArray);
        };

        Ok(row_ints
            .iter()
            .zip(slot_ints.iter())
            .any(|(&row, &slot)| row == self.row && slot == self.slot))
    }
}
