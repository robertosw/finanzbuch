use crate::structs::*;

#[test]
fn add_or_get_year() {
    // if 0
    // 1 2 4 5
    // lt_index = 0
    // gt_index = None
    //
    // if 3
    // 1 2 4 5
    // lt_index = 2
    // gt_index = 1
    //
    // if 6
    // 1 2 4 5
    // lt_index = None
    // gt_index = 3

    let yamlfile = YamlFile {
        version: 1,
        goal: 0.0,
        years: vec![Year::default(2025), Year::default(2031)],
    };

    let yamlfile_added_front_manual = YamlFile {
        version: 1,
        goal: 0.0,
        years: vec![Year::default(2018), Year::default(2025), Year::default(2031)],
    };
    let yamlfile_added_middle_manual = YamlFile {
        version: 1,
        goal: 0.0,
        years: vec![Year::default(2025), Year::default(2028), Year::default(2031)],
    };
    let yamlfile_added_end_manual = YamlFile {
        version: 1,
        goal: 0.0,
        years: vec![Year::default(2025), Year::default(2031), Year::default(2032)],
    };

    let mut yamlfile_added_front_fn = yamlfile.clone();
    yamlfile_added_front_fn.add_or_get_year(2018);

    let mut yamlfile_added_middle_fn = yamlfile.clone();
    yamlfile_added_middle_fn.add_or_get_year(2028);

    let mut yamlfile_added_end_fn = yamlfile.clone();
    yamlfile_added_end_fn.add_or_get_year(2032);

    assert_eq!(yamlfile_added_front_fn.years, yamlfile_added_front_manual.years);
    assert_ne!(yamlfile_added_front_fn.years, yamlfile.years);

    assert_eq!(yamlfile_added_middle_fn.years, yamlfile_added_middle_manual.years);
    assert_ne!(yamlfile_added_middle_fn.years, yamlfile.years);

    assert_eq!(yamlfile_added_end_fn.years, yamlfile_added_end_manual.years);
    assert_ne!(yamlfile_added_end_fn.years, yamlfile.years);
}
