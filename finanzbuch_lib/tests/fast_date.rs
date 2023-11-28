use finanzbuch_lib::FastDate;

/// - 9.7.2023 is 190th day of the year:
/// - 31 + 28 + 31 + 30 + 31 + 30 + 9
/// - ceil(190 / 7) = 28
#[test]
fn new() { assert_eq!(FastDate::new(2023, 7, 9).unwrap().date(), (2023, 7, 9, 28)) }

#[test]
fn new_invalid_month() { assert!(FastDate::new(2023, 13, 23).is_err()) }

#[test]
fn new_invalid_day() { assert!(FastDate::new(2023, 11, 32).is_err()) }

#[test]
fn max_values() { assert_eq!(FastDate::new(2023, 12, 31).unwrap().date(), (2023, 12, 31, 53)) }

#[test]
fn min_values() { assert_eq!(FastDate::new(2023, 1, 1).unwrap().date(), (2023, 1, 1, 1)) }

#[test]
fn set_year()
{
    let mut date = FastDate::new(2023, 11, 23).unwrap();
    date.set_year(2024);
    assert_eq!(date.year(), 2024);
}

#[test]
fn set_month()
{
    let mut date = FastDate::new(2023, 11, 23).unwrap();
    assert!(date.set_month(13).is_err());
    assert!(date.set_month(0).is_err());
    assert!(date.set_month(12).is_ok());
}

#[test]
fn set_day()
{
    let mut date = FastDate::new(2023, 11, 23).unwrap();
    assert!(date.set_day(32).is_err());
    assert!(date.set_day(0).is_err());
    assert!(date.set_day(31).is_ok());
}

#[test]
fn comparison_smaller_larger()
{
    let past = FastDate::new(2000, 1, 1).unwrap();
    let future = FastDate::new(2000, 1, 2).unwrap();
    assert!(past < future);
    assert!(future > past);
}

#[test]
fn comparison_eq()
{
    let now = FastDate::new(2000, 11, 23).unwrap();
    assert!(now == now);
}

#[test]
fn comparison_smaller_eq()
{
    let past = FastDate::new(2000, 1, 1).unwrap();
    let future = FastDate::new(2000, 1, 2).unwrap();
    assert!(past < future);
    assert!(past <= past);
}

#[test]
fn bit_mask_negate()
{
    assert_eq!(0b0000_0000 as u8, !0b1111_1111 as u8);
    assert_ne!(0b1111_1111, !0b0000_0000);
    assert_ne!(0b1111 as u8, !0b0000 as u8);
}
