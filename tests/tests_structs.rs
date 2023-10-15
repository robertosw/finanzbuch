// use std::collections::HashMap;

// use finance_yaml::structs::Year;
// use finance_yaml::Config;

// #[test]
// fn add_or_get_year() {
//     // if 0
//     // 1 2 4 5
//     // lt_index = 0
//     // gt_index = None
//     //
//     // if 3
//     // 1 2 4 5
//     // lt_index = 2
//     // gt_index = 1
//     //
//     // if 6
//     // 1 2 4 5
//     // lt_index = None
//     // gt_index = 3

//     let Config = Config {
//         version: 1,
//         goal: 0.0,
//         years: HashMap::from([(2025, Year::default(2025)), (2031, Year::default(2031))]),
//     };

//     let Config_added_front_manual = Config {
//         version: 1,
//         goal: 0.0,
//         years: HashMap::from([(2018, Year::default(2018)), (2025, Year::default(2025)), (2031, Year::default(2031))]),
//     };
//     let Config_added_middle_manual = Config {
//         version: 1,
//         goal: 0.0,
//         years: HashMap::from([(2025, Year::default(2025)), (2028, Year::default(2028)), (2031, Year::default(2031))]),
//     };
//     let Config_added_end_manual = Config {
//         version: 1,
//         goal: 0.0,
//         years: HashMap::from([(2025, Year::default(2025)), (2031, Year::default(2031)), (2032, Year::default(2032))]),
//     };

//     let mut Config_added_front_fn = Config.clone();
//     Config_added_front_fn.add_or_get_year(2018);

//     let mut Config_added_middle_fn = Config.clone();
//     Config_added_middle_fn.add_or_get_year(2028);

//     let mut Config_added_end_fn = Config.clone();
//     Config_added_end_fn.add_or_get_year(2032);

//     assert_eq!(Config_added_front_fn.years, Config_added_front_manual.years);
//     assert_ne!(Config_added_front_fn.years, Config.years);

//     assert_eq!(Config_added_middle_fn.years, Config_added_middle_manual.years);
//     assert_ne!(Config_added_middle_fn.years, Config.years);

//     assert_eq!(Config_added_end_fn.years, Config_added_end_manual.years);
//     assert_ne!(Config_added_end_fn.years, Config.years);
// }
