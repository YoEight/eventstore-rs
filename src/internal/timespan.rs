pub struct Timespan {
    pub ticks: i64
}

pub struct Builder {
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
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

    pub fn days(&mut self, days: i64) -> &mut Builder {
        self.days = days;

        self
    }

    pub fn hours(&mut self, hours: i64) -> &mut Builder {
        self.hours = hours;

        self
    }

    pub fn minutes(&mut self, minutes: i64) -> &mut Builder {
        self.minutes = minutes;

        self
    }

    pub fn seconds(&mut self, seconds: i64) -> &mut Builder {
        self.seconds = seconds;

        self
    }

    pub fn milliseconds(&mut self, milliseconds: i64) -> &mut Builder {
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
    pub fn from_ticks(ticks: i64) -> Timespan {
        Timespan {
            ticks,
        }
    }

    pub fn new_builder() -> Builder {
        Builder::new()
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
