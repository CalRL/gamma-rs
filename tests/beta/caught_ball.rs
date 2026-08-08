use crate::common;

use gamma_rs::save::beta::StorageType;
use gamma_rs::save::beta::caught_ball::{CaughtBall, CaughtBallMut};
use gamma_rs::save::beta::pokemon::ball::PokeBall;
use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::array_property::ArrayProperty;

struct CaughtBallCase<'a> {
    property_name: &'a str,
    storage_type: StorageType,
    idx: usize,
}

fn generate_case(property_name: &str, storage_type: StorageType, idx: usize) -> CaughtBallCase<'_> {
    CaughtBallCase {
        property_name,
        storage_type,
        idx,
    }
}

fn raw_caught_ball_at(gvas_file: &GvasFile, property_name: &str, idx: usize) -> PokeBall {
    let property = gvas_file
        .properties
        .get(property_name)
        .expect("caught ball property exists");
    let array = property
        .get_array()
        .expect("caught ball property is an array");
    let property = match array {
        ArrayProperty::Properties { properties, .. } => properties
            .get(idx)
            .expect("caught ball property exists at index"),
        _ => panic!("caught ball property has unexpected array type"),
    };
    let ball_enum = match property {
        Property::ObjectProperty(object) => object.value.as_str(),
        _ => panic!("caught ball entry has unexpected property type"),
    };

    PokeBall::from_enum(ball_enum).expect("caught ball enum is supported")
}

fn assert_caught_ball(gvas_file: &GvasFile, case: &CaughtBallCase<'_>) {
    let caught_ball =
        CaughtBall::new(gvas_file, case.storage_type.clone()).expect("caught ball wrapper exists");
    let expected = raw_caught_ball_at(gvas_file, case.property_name, case.idx);
    let actual = caught_ball
        .get_caught_ball_at(case.idx)
        .expect("caught ball exists at index");

    assert_eq!(actual, expected);
}

fn assert_sets_caught_ball(gvas_file: &GvasFile, case: &CaughtBallCase<'_>, new_ball: PokeBall) {
    let mut cloned_gvas_file = gvas_file.clone();
    let mut caught_ball = CaughtBallMut::new(&mut cloned_gvas_file, case.storage_type.clone())
        .expect("caught ball mut wrapper exists");

    assert!(caught_ball.set_ball_at(new_ball.clone(), case.idx).is_ok());
    drop(caught_ball);

    let caught_ball = CaughtBall::new(&cloned_gvas_file, case.storage_type.clone())
        .expect("caught ball wrapper exists");
    let actual = caught_ball
        .get_caught_ball_at(case.idx)
        .expect("caught ball exists at index");

    assert_eq!(actual, new_ball);
}

#[test]
fn reads_caught_balls_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        generate_case("PartyCaughtBall", StorageType::PARTY, 0),
        generate_case("Box1CaughtBall", StorageType::BOXES(1), 0),
    ];

    for case in cases {
        assert_caught_ball(&gvas_file, case);
    }
}

#[test]
fn sets_caught_balls_on_cloned_gvas_file() {
    let gvas_file = common::load_slot1();
    let cases = &[
        (
            generate_case("PartyCaughtBall", StorageType::PARTY, 0),
            PokeBall::UltraBall,
        ),
        (
            generate_case("Box1CaughtBall", StorageType::BOXES(1), 0),
            PokeBall::GreatBall,
        ),
    ];

    for (case, new_ball) in cases {
        assert_sets_caught_ball(&gvas_file, case, new_ball.clone());
    }
}
