use crate::types;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Date {
    day: u32,
    month: u32,
    year: u32,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Time {
    hour: u32,
    minute: u32,
    second: u32,
}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DateTime {
    date: Date,
    time: Time,
}
impl Default for Time {
    fn default() -> Self {
        Time {
            hour: 1,
            minute: 1,
            second: 1900,
        }
    }
}
impl Default for Date {
    fn default() -> Self {
        Date {
            day: 1,
            month: 1,
            year: 1900,
        }
    }
}

impl FromStr for Date {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("/").collect();

        if v.len() != 3 {
            return Err(());
        }
        let year = v[0].parse().unwrap();
        let month = v[1].parse().unwrap();
        let day = v[2].parse().unwrap();

        Ok(Date { day, month, year })
    }
}

impl FromStr for Time {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(":").collect();

        if v.len() != 3 {
            return Err(());
        }

        let hour = v[0].parse().unwrap();
        let minute = v[1].parse().unwrap();
        let second = v[2].parse().unwrap();

        Ok(Time {
            hour,
            minute,
            second,
        })
    }
}
impl FromStr for DateTime {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(" - ").collect();

        if v.len() != 2 {
            return Err(());
        }
        let date = v[0].parse().unwrap();
        let time = v[1].parse().unwrap();

        Ok(DateTime { date, time })
    }
}
impl From<String> for DateTime {
    fn from(value: String) -> Self {
        match DateTime::from_str(value.as_str()) {
            Ok(v) => v,
            Err(_) => {
                println!("Error in impl &string for datetime");
                Self {
                    ..Default::default()
                }
            }
        }
    }
}
impl From<&str> for DateTime{
    fn from(value: &str) -> Self {
        DateTime::from_str(value).unwrap()
    }
}
impl From<&String> for DateTime {
    fn from(value: &String) -> Self {
        DateTime::from(value.to_owned())
    }
}
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}/{} - {}:{}:{}",
            self.date.day,
            self.date.month,
            self.date.year,
            self.time.hour,
            self.time.minute,
            self.time.second
        )
    }
}

impl types::Print for DateTime {
    fn print(&self) {
        println!(
            "{}/{}/{} - {}:{}:{}",
            self.date.day,
            self.date.month,
            self.date.year,
            self.time.hour,
            self.time.minute,
            self.time.second
        );
    }
}
