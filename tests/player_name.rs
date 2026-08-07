mod common;

use gamma_rs::save::beta::player::name::{get_name, get_name_mut};
use gvas::GvasFile;
use gvas::properties::text_property::FTextHistory;

fn raw_trainer_name(gvas_file: &GvasFile) -> &String {
    let history = &gvas_file
        .properties
        .get("TrainerName")
        .expect("trainer name property exists")
        .get_text()
        .expect("trainer name property is text")
        .value
        .history;

    match history {
        FTextHistory::None {
            culture_invariant_string,
        } => culture_invariant_string
            .as_ref()
            .expect("trainer name has culture invariant string"),
        _ => panic!("trainer name has unexpected text history"),
    }
}

#[test]
fn reads_player_name_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let expected = raw_trainer_name(&gvas_file);
    let actual = get_name(&gvas_file).expect("player name exists");

    assert_eq!(actual, expected);
}

#[test]
fn reads_expected_name_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let expected = &String::from("CAL");
    let actual = get_name(&gvas_file).expect("player name exists");

    assert_eq!(actual, expected);
}

#[test]
fn mutates_player_name_on_cloned_gvas_file() {
    let mut gvas_file = common::load_slot1();
    let name = get_name_mut(&mut gvas_file).expect("player name exists");

    *name = "TEST_PLAYER".to_string();

    assert_eq!(name, "TEST_PLAYER");
}
