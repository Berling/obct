use chrono::{Datelike, NaiveDate};

fn month_diff(from: NaiveDate, to: NaiveDate) -> u32 {
    debug_assert!(from <= to);
    let from_month = from.month();
    let to_month = to.month();
    if from_month == 12u32 {
        to_month
    } else {
        to_month - from_month
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::month_diff;

    #[test]
    fn test_month_diff() {
        let from = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2023, 2, 23).unwrap();
        assert_eq!(month_diff(from, to), 1);

        let from = NaiveDate::from_ymd_opt(2023, 2, 3).unwrap();
        let to = NaiveDate::from_ymd_opt(2023, 6, 19).unwrap();
        assert_eq!(month_diff(from, to), 4);

        let from = NaiveDate::from_ymd_opt(2023, 12, 13).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        assert_eq!(month_diff(from, to), 1);

        let from = NaiveDate::from_ymd_opt(2023, 12, 30).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 2, 11).unwrap();
        assert_eq!(month_diff(from, to), 2);
    }
}
