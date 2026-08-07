use gvas::GvasFile;
use gvas::properties::text_property::FTextHistory;

pub fn get_name(gvas_file: &GvasFile) -> Option<&String> {
    let history = &gvas_file
        .properties
        .get("TrainerName")?
        .get_text()?
        .value
        .history;

    if let FTextHistory::None {
        culture_invariant_string,
    } = history
    {
        Some(culture_invariant_string.as_ref()?)
    } else {
        None
    }
}

pub fn get_name_mut(gvas_file: &mut GvasFile) -> Option<&mut String> {
    let history = &mut gvas_file
        .properties
        .get_mut("TrainerName")?
        .get_text_mut()?
        .value
        .history;

    if let FTextHistory::None {
        culture_invariant_string,
    } = history
    {
        Some(culture_invariant_string.as_mut()?)
    } else {
        None
    }
}
