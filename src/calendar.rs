use std::ops::{Add, Sub};
use chrono::*;

#[derive(Copy, Clone)]
pub enum Calendar {
    Years(i32),
    Months(i32),
    Days(i64),
    Hours(i64),
    Minutes(i64),
    Seconds(i64),
}

impl<T: TimeZone> Add<DateTime<T>> for Calendar {
    type Output = DateTime<T>;

    fn add(self, rhs: DateTime<T>) -> Self::Output {
        add_for_date_time(rhs, self, false)
    }
}

impl<T: TimeZone> Sub<DateTime<T>> for Calendar {
    type Output = DateTime<T>;

    fn sub(self, rhs: DateTime<T>) -> Self::Output {
        add_for_date_time(rhs, self, true)
    }
}

impl<T: TimeZone> Add<Calendar> for DateTime<T> {
    type Output = DateTime<T>;

    fn add(self, rhs: Calendar) -> Self::Output {
        add_for_date_time(self, rhs, false)
    }
}

impl<T: TimeZone> Sub<Calendar> for DateTime<T> {
    type Output = DateTime<T>;

    fn sub(self, rhs: Calendar) -> Self::Output {
        add_for_date_time(self, rhs, true)
    }
}

fn add_for_date_time<T: TimeZone>(date: DateTime<T>, time_unit: Calendar, is_sub: bool) -> DateTime<T> {
    let a = {
        if is_sub {
            -1
        } else {
            1
        }
    };

    let result = match time_unit {
        Calendar::Years(i) => chronoutil::shift_years(date, i * a),
        Calendar::Months(i) => chronoutil::shift_months(date, i * a),
        Calendar::Days(i) => date + Duration::days(i * a as i64),
        Calendar::Hours(i) => date + Duration::hours(i * a as i64),
        Calendar::Minutes(i) => date + Duration::minutes(i * a as i64),
        Calendar::Seconds(i) => date + Duration::seconds(i * a as i64),
    };

    result
}

