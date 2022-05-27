# calendar
It is easy to compute date or time.
```
use calendar::*;

let mut date_time = Local.ymd(2022, 1, 29).and_hms(0, 0, 0);
date_time = date_time + Calendar::Years(1) - Calendar::Months(1) + Calendar::Days(10);
date_time = date_time + Calendar::Hours(12) + Calendar::Minutes(30) + Calendar::Seconds(60);
```
