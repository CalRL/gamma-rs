mod common;

use gamma_rs::save::beta::player::transform::{get_transform, get_transform_mut};

fn assert_float_eq(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < f64::EPSILON);
}

#[test]
fn reads_player_transform_translation() {
    let gvas_file = common::load_slot1();
    let transform = get_transform(&gvas_file).expect("player transform exists");

    assert_float_eq(*transform.x, 142598.6875);
    assert_float_eq(*transform.y, 14589.3955078125);
    assert_float_eq(*transform.z, 66.54291326939514);
}

#[test]
fn mutates_player_transform_translation_on_cloned_gvas_file() {
    let mut gvas_file = common::load_slot1();
    let transform = get_transform_mut(&mut gvas_file).expect("player transform exists");

    *transform.x = 1.0;
    *transform.y = 2.0;
    *transform.z = 3.0;

    assert_float_eq(*transform.x, 1.0);
    assert_float_eq(*transform.y, 2.0);
    assert_float_eq(*transform.z, 3.0);
}
