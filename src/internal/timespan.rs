pub struct Timespan {
    pub ticks: i64
}

impl Timespan {
    pub fn from_ticks(ticks: i64) -> Timespan {
        Timespan {
            ticks,
        }
    }

    pub fn from_hours_mins_secs(hours: i64,  minutes: i64, seconds: i64) -> Timespan {
        let total_secs = hours * 3600 + minutes * 60 + seconds;
        let ticks      = total_secs * TICKS_PER_SECONDS;

        Timespan::from_ticks(ticks)
    }

    pub fn from_days_hours_mins_secs(days: i64, hours: i64, minutes: i64, seconds: i64) -> Timespan {
        Timespan::from_days_hours_mins_secs_millis(days, hours, minutes, seconds, 0)
    }

    pub fn from_days_hours_mins_secs_millis(days: i64, hours: i64, minutes: i64, seconds: i64, milliseconds: i64) -> Timespan {
        let total_millis = days * 24 * 3600 + hours * 3600 + minutes * 60 + seconds;
        let total_millis = total_millis * 1000;
        let total_millis = total_millis + milliseconds;
        let ticks        = total_millis * TICKS_PER_MILLIS;

        Timespan::from_ticks(ticks)
    }

    pub fn days(&self) -> i64 {
        ((self.ticks as f64) / DAYS_PER_TICK) as i64
    }

    pub fn hours(&self) -> i64 {
        (((self.ticks as f64) / HOURS_PER_TICK) as i64) % 24
    }

    pub fn minutes(&self) -> i64 {
        (((self.ticks as f64) / MINUTES_PER_TICK) as i64) % 60
    }

    pub fn seconds(&self) -> i64 {
        (((self.ticks as f64) / SECONDS_PER_TICK) as i64) % 60
    }

    pub fn milliseconds(&self) -> i64 {
        (((self.ticks as f64) / MILLIS_PER_TICK) as i64) % 1000
    }
}

const MILLIS_PER_SECOND: i64 = 1_000;
const MILLIS_PER_MINUTE: i64 = MILLIS_PER_SECOND * 60;
const MILLIS_PER_HOUR: i64 = MILLIS_PER_MINUTE * 60;
const MILLIS_PER_DAY: i64 = MILLIS_PER_HOUR * 24;

const TICKS_PER_MILLIS: i64 = 10_000;
const TICKS_PER_SECONDS: i64 = TICKS_PER_MILLIS * 1_000;
const TICKS_PER_MINUTE: i64 = TICKS_PER_SECONDS * 60;
const TICKS_PER_HOUR: i64 = TICKS_PER_MINUTE * 60;
const TICKS_PER_DAY: i64 = TICKS_PER_HOUR * 24;

const DAYS_PER_TICK: f64 = 1.0 / (TICKS_PER_DAY as f64);
const HOURS_PER_TICK: f64 = 1.0 / (TICKS_PER_HOUR as f64);
const MINUTES_PER_TICK: f64 = 1.0 / (TICKS_PER_MINUTE as f64);
const SECONDS_PER_TICK: f64 = 1.0 / (TICKS_PER_SECONDS as f64);
const MILLIS_PER_TICK: f64 = 1.0 / (TICKS_PER_MILLIS as f64);
