use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct DateTime {
    month: Month,
    day: u8,
    year: usize,
    hour: u8,
    minutes: u8,
    seconds: u8,
    fraction: u32,
    #[allow(dead_code)]
    offset: i8,
}

impl Default for DateTime {
    fn default() -> Self {
        DateTime {
            year: 2000,
            month: Month::January,
            day: 1,
            hour: 0,
            minutes: 0,
            seconds: 0,
            fraction: 0,
            offset: 0,
        }
    }
}

impl ::std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}-{}-{}T{}:{}:{}.{}Z",
            self.year, self.month, self.day, self.hour, self.minutes, self.seconds, self.fraction
        )
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut split = s.split("T");
        println!("{:?}", split);
        let date = split.next().expect("Expected Date");
        println!("{:?}", date);
        let mut date = date.split("-");
        println!("{:?}", date);

        let year = date.next().expect("Expected Yaer");
        println!("{:?}", year);
        let month = date.next().expect("Expected Month");
        println!("{:?}", month);
        let day = date.next().expect("Expected Day");
        println!("{:?}", day);

        let time = split.next().expect("Expected Time");
        println!("{:?}", time);
        let mut time = time.split(":");
        println!("{:?}", time);

        let hour = time.next().expect("Expected hour");
        println!("{:?}", hour);
        let minute = time.next().expect("Expected minute");
        println!("{:?}", minute);
        let seconds = time.next().expect("Expected Seconds");
        println!("{:?}", seconds);

        let mut seconds = seconds.split(".");
        println!("{:?}", seconds);
        let second = seconds.next().expect("Expected Second");
        println!("{:?}", second);
        let fraction = seconds.next().expect("Expected Fraction");
        println!("{:?}", fraction);

        let d = DateTime {
            year: year.parse().unwrap(),
            month: month.parse::<u8>().unwrap().into(),
            day: day.parse().unwrap(),
            hour: hour.parse().unwrap(),
            minutes: minute.parse().unwrap(),
            seconds: second.parse().unwrap(),
            fraction: fraction.trim_right_matches("Z").parse().unwrap(),
            offset: i8::default(),
        };
        Ok(d)
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", *self);
        serializer.serialize_str(&s)
    }
}

#[derive(Debug)]
enum Month {
    January,
    Febuary,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl ::std::fmt::Display for Month {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Month::January => 1,
                Month::Febuary => 2,
                Month::March => 3,
                Month::April => 4,
                Month::May => 5,
                Month::June => 6,
                Month::July => 7,
                Month::August => 8,
                Month::September => 9,
                Month::October => 10,
                Month::November => 11,
                Month::December => 12,
            }
        )
    }
}

impl From<u8> for Month {
    fn from(i: u8) -> Self {
        match i {
            1 => Month::January,
            2 => Month::Febuary,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => unreachable!("You shouldn't be here"),
        }
    }
}
