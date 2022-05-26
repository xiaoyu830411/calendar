pub mod calendar;

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local, Timelike, TimeZone};
    use super::calendar::Calendar;

    #[test]
    fn test_add() {
        let mut date = Local.ymd(2022, 1, 29).and_hms(0, 0, 0);
        date = date + Calendar::Years(1);
        assert_eq!(2023, date.year());

        date = date + Calendar::Months(1);
        assert_eq!(2023, date.year());
        assert_eq!(2, date.month());

        date = date + Calendar::Months(12);
        assert_eq!(2024, date.year());
        assert_eq!(2, date.month());

        date = date + Calendar::Days(1);
        assert_eq!(2024, date.year());
        assert_eq!(2, date.month());
        assert_eq!(29, date.day());

        date = date + Calendar::Hours(12) + Calendar::Minutes(13) + Calendar::Seconds(14);
        assert_eq!(12, date.hour());
        assert_eq!(13, date.minute());
        assert_eq!(14, date.second());
    }

    #[test]
    fn test_sub() {
        let mut date = Local.ymd(2022, 3, 29).and_hms(13, 15, 17);
        date = date - Calendar::Years(1);
        assert_eq!(2021, date.year());

        date = date - Calendar::Months(1);
        assert_eq!(2021, date.year());
        assert_eq!(2, date.month());

        date = date - Calendar::Months(12);
        assert_eq!(2020, date.year());

        date = date - Calendar::Days(1);
        assert_eq!(2020, date.year());
        assert_eq!(2, date.month());
        assert_eq!(27, date.day());

        date = date - Calendar::Hours(12) - Calendar::Minutes(13) - Calendar::Seconds(14);
        assert_eq!(1, date.hour());
        assert_eq!(2, date.minute());
        assert_eq!(3, date.second());
    }
}