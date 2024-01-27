use aoc_2023_rs::almanac::{map_range::MapRange, Almanac};

#[test]
fn get_lowest_location_gets_answer_for_example_data() {
    let almanac = Almanac::from_file("./data/day_5/example.txt");
    let actual = almanac.get_lowest_location();
    assert_eq!(actual, 35);
}

#[test]
fn get_lowest_location_gets_answer_for_real_data() {
    let almanac = Almanac::from_file("./data/day_5/data.txt");
    let actual = almanac.get_lowest_location();
    assert_eq!(actual, 227653707);
}

#[test]
fn get_lowest_location_ranged_gets_answer_for_example_data() {
    let almanac = Almanac::from_file("./data/day_5/example.txt");
    let actual = almanac.get_lowest_location_ranged();
    assert_eq!(actual, 46);
}

#[test]
fn get_lowest_location_ranged_gets_answer_for_real_data() {
    let almanac = Almanac::from_file("./data/day_5/data.txt");
    let actual = almanac.get_lowest_location_ranged();
    assert_eq!(actual, 78775051);
}

#[test]
fn almanac_from_file_works_for_example_data() {
    let actual = Almanac::from_file("./data/day_5/example.txt");
    let expected_seeds = vec![79, 14, 55, 13];
    let expected_seed_ranges = vec![(79, 14), (55, 13)];
    let expected_maps = vec![
        vec![
            MapRange {
                source: 50,
                destination: 52,
                range: 48,
            },
            MapRange {
                source: 98,
                destination: 50,
                range: 2,
            },
        ],
        vec![
            MapRange {
                source: 0,
                destination: 39,
                range: 15,
            },
            MapRange {
                source: 15,
                destination: 0,
                range: 37,
            },
            MapRange {
                source: 52,
                destination: 37,
                range: 2,
            },
        ],
        vec![
            MapRange {
                source: 0,
                destination: 42,
                range: 7,
            },
            MapRange {
                source: 7,
                destination: 57,
                range: 4,
            },
            MapRange {
                source: 11,
                destination: 0,
                range: 42,
            },
            MapRange {
                source: 53,
                destination: 49,
                range: 8,
            },
        ],
        vec![
            MapRange {
                source: 18,
                destination: 88,
                range: 7,
            },
            MapRange {
                source: 25,
                destination: 18,
                range: 70,
            },
        ],
        vec![
            MapRange {
                source: 45,
                destination: 81,
                range: 19,
            },
            MapRange {
                source: 64,
                destination: 68,
                range: 13,
            },
            MapRange {
                source: 77,
                destination: 45,
                range: 23,
            },
        ],
        vec![
            MapRange {
                source: 0,
                destination: 1,
                range: 69,
            },
            MapRange {
                source: 69,
                destination: 0,
                range: 1,
            },
        ],
        vec![
            MapRange {
                source: 56,
                destination: 60,
                range: 37,
            },
            MapRange {
                source: 93,
                destination: 56,
                range: 4,
            },
        ],
    ];
    assert_eq!(actual.seeds, expected_seeds);
    assert_eq!(actual.seed_ranges, expected_seed_ranges);
    assert_eq!(actual.maps, expected_maps);
}
