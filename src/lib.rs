use std::time::Duration;

use std::fmt::{Display, Formatter, Result};

pub struct Durado {
  duration: Duration
}

impl  Durado {
  pub fn new(duration: Duration) -> Durado {
    Durado {
      duration: duration
    }
  }

  pub fn as_secs(&self) -> u64 {
    self.duration.as_secs()
  }

  pub fn subsec_nanos(&self) -> u32 {
    self.duration.subsec_nanos()
  }
}

impl Display for Durado {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let (secs, nanos) = (self.as_secs(), self.subsec_nanos());
    if secs == 0 {
      write!(f, "{} nanos", nanos)
    } else {
      write!(f, "{}", secs)
    }
  }
}
