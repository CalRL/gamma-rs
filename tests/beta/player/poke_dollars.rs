use crate::common;
use gamma_rs::save::beta::poke_dollars::{get_poke_dollars, get_poke_dollars_mut};
use gvas::GvasFile;

fn raw_poke_dollars(gvas_file: &GvasFile) -> i32 {
    gvas_file
        .properties
        .get("pokeDollars")
        .expect("poke dollars property exists")
        .get_int()
        .expect("poke dollars property is an int")
        .value
}

#[test]
fn reads_poke_dollars_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let expected = raw_poke_dollars(&gvas_file);
    let actual = get_poke_dollars(&gvas_file).expect("poke dollars exist");

    assert_eq!(actual, expected);
}

#[test]
fn reads_expected_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let expected = 38352321i32;
    let actual = get_poke_dollars(&gvas_file).expect("poke dollars exist");

    assert_eq!(actual, expected);
}

#[test]
fn mutates_poke_dollars_on_cloned_gvas_file() {
    let mut gvas_file = common::load_slot1();
    let poke_dollars = get_poke_dollars_mut(&mut gvas_file).expect("poke dollars exist");

    *poke_dollars = 12345;

    assert_eq!(*poke_dollars, 12345);
}
