use gvas::GvasFile;

pub fn get_poke_dollars(gvas_file: &GvasFile) -> Option<i32> {
    let val = gvas_file
        .properties
        .get("pokeDollars")?
        .get_int()?
        .value;

    Some(val)
}

pub fn get_poke_dollars_mut(gvas_file: &mut GvasFile) -> Option<&mut i32> {
    let val = &mut gvas_file
        .properties
        .get_mut("pokeDollars")?
        .get_int_mut()?
        .value;

    Some(val)
}
