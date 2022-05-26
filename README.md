# calendar
It is easy to compute date or time.
```
let mut date = Local.ymd(2022, 1, 29).and_hms(0, 0, 0);
date = date + Calendar::Years(1) - Calendar::Months(1) + Calendar::Days(10);
date = date + Calendar::House(12) + Calendar::Minutes(30) + Calendar::Seconds(60);
```
