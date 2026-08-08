use gamma_rs::save::beta::BetaEnumStr;
use gamma_rs::save::beta::pokemon::natures::Nature;

struct NatureCase<'a> {
    enum_value: &'a str,
    expected_nature: Nature,
}

fn generate_case(enum_value: &str, expected_nature: Nature) -> NatureCase<'_> {
    NatureCase {
        enum_value,
        expected_nature,
    }
}

fn assert_nature(case: &NatureCase<'_>) {
    let enum_value = BetaEnumStr::try_from(case.enum_value).expect("beta enum string parses");
    let nature = Nature::try_from(enum_value).expect("nature parses");

    assert_eq!(nature, case.expected_nature);
}

#[test]
fn converts_valid_beta_enum_strings_to_natures() {
    let cases = [
        generate_case("ENUM_Natures::NewEnumerator0", Nature::Hardy),
        generate_case("ENUM_Natures::NewEnumerator1", Nature::Lonely),
        generate_case("ENUM_Natures::NewEnumerator2", Nature::Brave),
        generate_case("ENUM_Natures::NewEnumerator3", Nature::Adamant),
        generate_case("ENUM_Natures::NewEnumerator6", Nature::Docile),
        generate_case("ENUM_Natures::NewEnumerator7", Nature::Relaxed),
        generate_case("ENUM_Natures::NewEnumerator8", Nature::Impish),
        generate_case("ENUM_Natures::NewEnumerator9", Nature::Lax),
        generate_case("ENUM_Natures::NewEnumerator10", Nature::Timid),
        generate_case("ENUM_Natures::NewEnumerator11", Nature::Hasty),
        generate_case("ENUM_Natures::NewEnumerator12", Nature::Serious),
        generate_case("ENUM_Natures::NewEnumerator13", Nature::Jolly),
        generate_case("ENUM_Natures::NewEnumerator14", Nature::Naive),
        generate_case("ENUM_Natures::NewEnumerator15", Nature::Modest),
        generate_case("ENUM_Natures::NewEnumerator16", Nature::Mild),
        generate_case("ENUM_Natures::NewEnumerator17", Nature::Quiet),
        generate_case("ENUM_Natures::NewEnumerator18", Nature::Bashful),
        generate_case("ENUM_Natures::NewEnumerator19", Nature::Rash),
        generate_case("ENUM_Natures::NewEnumerator20", Nature::Calm),
        generate_case("ENUM_Natures::NewEnumerator21", Nature::Gentle),
        generate_case("ENUM_Natures::NewEnumerator22", Nature::Sassy),
        generate_case("ENUM_Natures::NewEnumerator23", Nature::Careful),
        generate_case("ENUM_Natures::NewEnumerator24", Nature::Quirky),
    ];

    for case in cases {
        assert_nature(&case);
    }
}

#[test]
fn rejects_invalid_nature_enum_numbers() {
    let cases = [
        "ENUM_Natures::NewEnumerator4",
        "ENUM_Natures::NewEnumerator5",
        "ENUM_Natures::NewEnumerator25",
    ];

    for case in cases {
        let enum_value = BetaEnumStr::try_from(case).expect("beta enum string parses");

        assert!(Nature::try_from(enum_value).is_err());
    }
}

#[test]
fn rejects_non_beta_enum_strings() {
    assert!(BetaEnumStr::try_from("Hardy").is_err());
}
