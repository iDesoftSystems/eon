use crate::chrono_ext::date::EndOfDay;

#[cfg(feature = "chrono-ext")]
impl EndOfDay for chrono::NaiveDate {
    fn end_of_day(&self) -> Option<chrono::NaiveDateTime> {
        self.and_hms_micro_opt(23, 59, 59, 999_999)
    }
}

#[cfg(test)]
#[cfg(feature = "chrono-ext")]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_end_of_day() {
        let expected = NaiveDate::from_ymd_opt(2025, 10, 31)
            .unwrap()
            .and_hms_micro_opt(23, 59, 59, 999_999);

        let date_end_of_day = NaiveDate::from_ymd_opt(2025, 10, 31).unwrap().end_of_day();

        assert_eq!(expected, date_end_of_day);
    }
}
