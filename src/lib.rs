use std::time::Duration;

use std::fmt::{Display, Formatter, Result};

const NANOS_PER_MICRO: u32 = 1000;
const NANOS_PER_MILLI: u32 = 1000_000;
const NANOS_PER_SEC: u32 = 1_000_000_000;
const MICROS_PER_SEC: u64 = 1000_000;
const MILLIS_PER_SEC: u64 = 1000;
const SECS_PER_MINUTE: u64 = 60;
const SECS_PER_HOUR: u64 = 3_600;
const SECS_PER_DAY: u64 = 86_400;
const SECS_PER_WEEK: u64 = 604_800;

pub struct Durado {
    duration: Duration,
}

impl Durado {
    pub fn new(duration: Duration) -> Durado {
        Durado { duration: duration }
    }
}

impl Display for Durado {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (secs, nanos) = (self.duration.as_secs(), self.duration.subsec_nanos());
        if self.duration.as_secs() == 0 {
            if nanos == 0 {
                try!(write!(f, "0s"));
            } else if nanos < NANOS_PER_MICRO {
                // ns
                try!(write!(f, "{}ns", nanos));
            } else if nanos < NANOS_PER_MILLI {
                // µs
                let micros = nanos / NANOS_PER_MICRO;
                try!(write!(f, "{}.{:03}µs", micros, (nanos - (micros * NANOS_PER_MICRO))));
            } else {
                // ms
                let millis = nanos / NANOS_PER_MILLI;
                try!(write!(f, "{}.{:06}ms", millis, (nanos - (millis * NANOS_PER_MILLI))));
            }
        } else {
            let weeks = secs / SECS_PER_WEEK;
            let days = (secs - (weeks * SECS_PER_WEEK)) / SECS_PER_DAY;
            let hours = (secs - (days * SECS_PER_DAY)) / SECS_PER_HOUR;
            let minutes = (secs - (hours * SECS_PER_HOUR)) / SECS_PER_MINUTE;
            let seconds = (secs - (minutes * SECS_PER_MINUTE));

            try!(write!(f, "{}w", weeks));
            try!(write!(f, "{}d", days));
            try!(write!(f, "{}h", hours));
            try!(write!(f, "{}m", minutes));

            if nanos == 0 {
                try!(write!(f, "{}s", secs));
            } else if nanos % NANOS_PER_MILLI == 0 {
                try!(write!(f, "{}.{:03}s", secs, nanos / NANOS_PER_MILLI));
            } else if nanos % NANOS_PER_MICRO == 0 {
                try!(write!(f, "{}.{:06}s", secs, nanos / NANOS_PER_MICRO));
            } else {
                try!(write!(f, "{}.{:09}s", secs, nanos));
            }
        }
        Ok(())
    }
}
