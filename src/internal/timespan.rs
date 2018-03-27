/// This module will probably be deleted. There is no reason
/// to bring .NET madness into Rust.
use std::borrow::Borrow;
use time::Duration;
use serde::de::{ self, Visitor, Deserializer, Deserialize };
use serde::ser::{ Serialize, Serializer };

pub struct Timespan {
    pub ticks: u64
}

pub struct Builder {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
    milliseconds: u64,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        }
    }

    pub fn days(&mut self, days: u64) -> &mut Builder {
        self.days = days;

        self
    }

    pub fn hours(&mut self, hours: u64) -> &mut Builder {
        self.hours = hours;

        self
    }

    pub fn minutes(&mut self, minutes: u64) -> &mut Builder {
        self.minutes = minutes;

        self
    }

    pub fn seconds(&mut self, seconds: u64) -> &mut Builder {
        self.seconds = seconds;

        self
    }

    pub fn milliseconds(&mut self, milliseconds: u64) -> &mut Builder {
        self.milliseconds = milliseconds;

        self
    }

    pub fn build(&self) -> Timespan {
        let total_millis = self.days * 24 * 3600 + self.hours * 3600 + self.minutes * 60 + self.seconds;
        let total_millis = total_millis * 1000;
        let total_millis = total_millis + self.milliseconds;
        let ticks        = total_millis * TICKS_PER_MILLIS;

        Timespan::from_ticks(ticks)
    }
}

impl Timespan {
    fn from_ticks(ticks: u64) -> Timespan {
        Timespan {
            ticks,
        }
    }

    pub fn from_duration(duration: Duration) -> Timespan {
        let mut builder = Timespan::new_builder();

        builder.days(duration.num_days() as u64)
               .hours(duration.num_hours() as u64)
               .minutes(duration.num_minutes() as u64)
               .seconds(duration.num_seconds() as u64)
               .milliseconds(duration.num_milliseconds() as u64)
               .build()
    }

    pub fn new_builder() -> Builder {
        Builder::new()
    }

    pub fn days(&self) -> u64 {
        ((self.ticks as f64) / DAYS_PER_TICK) as u64
    }

    pub fn hours(&self) -> u64 {
        (((self.ticks as f64) / HOURS_PER_TICK) as u64) % 24
    }

    pub fn minutes(&self) -> u64 {
        (((self.ticks as f64) / MINUTES_PER_TICK) as u64) % 60
    }

    pub fn seconds(&self) -> u64 {
        (((self.ticks as f64) / SECONDS_PER_TICK) as u64) % 60
    }

    pub fn milliseconds(&self) -> u64 {
        (((self.ticks as f64) / MILLIS_PER_TICK) as u64) % 1000
    }

    fn str_repr(&self) -> String {
        let mut builder = String::new();
        let     days      = self.days();
        let     hours     = self.hours();
        let     minutes   = self.minutes();
        let     seconds   = self.seconds();
        let     fractions = self.ticks % TICKS_PER_DAY;

        if days > 0 {
            builder.push_str(format!("{}.", days).borrow());
        }

        builder.push_str(format!("{:02}:", hours).borrow());
        builder.push_str(format!("{:02}:", minutes).borrow());
        builder.push_str(format!("{:02}", seconds).borrow());

        if fractions > 0 {
            builder.push_str(format!(".{:07}", fractions).borrow());
        }

        builder
    }
}

impl Serialize for Timespan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.str_repr().borrow())
    }
}

struct ForTimespan;

enum Parse {
    Days,
    Hours,
    Minutes,
    Seconds,
    Fractions,
}

impl <'de> Visitor<'de> for ForTimespan {
    type Value = Timespan;

    fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(formatter, "a string representing a Timespan")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
    {

        fn to_u32(vec: &Vec<u32>) -> u32 {
            if vec.is_empty() {
                return 0;
            }

            let mut exp    = (vec.len() as u32) - 1;
            let mut result = 0;

            for value in vec {
                result += value * 10_u32.pow(exp);
                exp    -= 1;
            }

            result
        }

        let mut state   = Parse::Days;
        let mut buffer  = Vec::new();
        let mut builder = Timespan::new_builder();

        for c in value.chars() {
            match state {
                Parse::Days => {
                    if c == '.' {
                        let num = to_u32(&buffer);

                        builder.days(num as u64);
                        buffer.clear();
                        state = Parse::Hours;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Hours => {
                    if c == ':' {
                        let num = to_u32(&buffer);

                        builder.hours(num as u64);
                        buffer.clear();
                        state = Parse::Minutes;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Minutes => {
                    if c == ':' {
                        let num = to_u32(&buffer);

                        builder.minutes(num as u64);
                        buffer.clear();
                        state = Parse::Seconds;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Seconds => {
                    if c == '.' {
                        let num = to_u32(&buffer);

                        builder.seconds(num as u64);
                        buffer.clear();
                        state = Parse::Fractions;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Fractions => {
                    buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                }
            }
        }

        let num = to_u32(&buffer);

        // In case the Timespan string representation didn't contain fractions.
        if let Parse::Seconds = state {
            builder.seconds(num as u64);
        } else {
            // Implement fraction conversion.
            unimplemented!()
        }

        Ok(builder.build())
    }
}

impl <'de> Deserialize<'de> for Timespan {
    fn deserialize<D>(deserializer: D) -> Result<Timespan, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(ForTimespan)
    }
}

const BASE_10_RDX: u32 = 10;

const TICKS_PER_MILLIS: u64 = 10_000;
const TICKS_PER_SECONDS: u64 = TICKS_PER_MILLIS * 1_000;
const TICKS_PER_MINUTE: u64 = TICKS_PER_SECONDS * 60;
const TICKS_PER_HOUR: u64 = TICKS_PER_MINUTE * 60;
const TICKS_PER_DAY: u64 = TICKS_PER_HOUR * 24;

const DAYS_PER_TICK: f64 = 1.0 / (TICKS_PER_DAY as f64);
const HOURS_PER_TICK: f64 = 1.0 / (TICKS_PER_HOUR as f64);
const MINUTES_PER_TICK: f64 = 1.0 / (TICKS_PER_MINUTE as f64);
const SECONDS_PER_TICK: f64 = 1.0 / (TICKS_PER_SECONDS as f64);
const MILLIS_PER_TICK: f64 = 1.0 / (TICKS_PER_MILLIS as f64);
