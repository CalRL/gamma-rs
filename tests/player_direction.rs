mod common;

use gamma_rs::save::beta::player::direction::{get_direction, get_direction_mut};

fn assert_float_eq(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < f64::EPSILON);
}

#[test]
fn reads_player_direction_from_gvas_file() {
    let gvas_file = common::load_slot1();
    let direction = get_direction(&gvas_file).expect("player direction exists");

    assert_float_eq(*direction.x, -1.0);
    assert_float_eq(*direction.y, 0.0);
}

#[test]
fn mutates_player_direction_on_cloned_gvas_file() {
    let mut gvas_file = common::load_slot1();
    let direction = get_direction_mut(&mut gvas_file).expect("player direction exists");

    *direction.x = 0.5;
    *direction.y = -0.5;

    assert_float_eq(*direction.x, 0.5);
    assert_float_eq(*direction.y, -0.5);
}
