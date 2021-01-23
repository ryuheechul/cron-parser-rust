use crate::cron_exp::{CronExp, NumberOrAny};

#[derive(Debug, PartialEq)]
struct CronSlot {
  exp: CronExp,
  bound: (u8, u8),
}

impl CronSlot {
  fn new(text: &str, min: u8, max: u8) -> Self {
    CronSlot {
      exp: CronExp::from(text),
      bound: (min, max),
    }
  }

  fn vectorize(&self) -> Vec<u8> {
    let (s, e) = self.bound;

    match &self.exp {
      CronExp::Invalid => {
        vec![]
      }
      CronExp::List(l) => l.to_vec(),
      &CronExp::Range(s, e) => (s..e + 1).collect(),
      CronExp::Step(f, s) => {
        let &f = if let NumberOrAny::Number(n) = f {
          n
        } else {
          &0
        };

        let s: usize = *s as usize;

        (f..e + 1).step_by(s).collect()
      }
      &CronExp::All => (s..e + 1).collect(),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Cron {
  pub minutes: Vec<u8>,
  pub hours: Vec<u8>,
  pub days_of_month: Vec<u8>,
  pub months: Vec<u8>,
  pub days_of_week: Vec<u8>,
  pub command: String,
}

fn numvec_to_str(vec: &[u8]) -> String {
  let l: Vec<String> = vec.iter().map(|n| n.to_string()).collect();
  l.join(" ")
}

impl Cron {
  pub fn new(line: &str) -> Option<Self> {
    let mut slots = line.split(" ");

    let slots: Option<Vec<&str>> = [
      slots.next(),
      slots.next(),
      slots.next(),
      slots.next(),
      slots.next(),
      slots.next(),
    ]
    .iter()
    .copied()
    .collect();

    // exit with None if length of slots are smaller than 6
    let mut slots = slots?;

    let command = slots.pop()?;

    let bounds: [(u8, u8); 5] = [
      (0, 59), // minute
      (0, 23), // hour
      (1, 31), // day of month
      (1, 12), // month
      (0, 6),  // day of week
    ];

    let mut slots = slots
      .iter()
      .zip(bounds.iter())
      .map(|(text, &(s, e))| CronSlot::new(text, s, e))
      .map(|c| c.vectorize());

    Some(Cron {
      minutes: slots.next().unwrap(),
      hours: slots.next().unwrap(),
      days_of_month: slots.next().unwrap(),
      months: slots.next().unwrap(),
      days_of_week: slots.next().unwrap(),
      command: command.into(),
    })
  }

  pub fn output(&self) -> String {
    let lines: Vec<String> = vec![
      format!("minute\t\t{}", numvec_to_str(&self.minutes)),
      format!("hour\t\t{}", numvec_to_str(&self.hours)),
      format!("day of month\t{}", numvec_to_str(&self.days_of_month)),
      format!("month\t\t{}", numvec_to_str(&self.months)),
      format!("day of week\t{}", numvec_to_str(&self.days_of_week)),
      format!("command\t\t{}", self.command),
    ];

    lines.join("\n")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cronslot_new() {
    // minute
    assert_eq!(
      CronSlot::new("*/15", 0, 59),
      CronSlot {
        exp: CronExp::Step(NumberOrAny::Any, 15),
        bound: (0, 59),
      }
    );

    // hour
    assert_eq!(
      CronSlot::new("0", 0, 23),
      CronSlot {
        exp: CronExp::List(vec![0]),
        bound: (0, 23),
      }
    );

    // day of month
    assert_eq!(
      CronSlot::new("1,15", 1, 31),
      CronSlot {
        exp: CronExp::List(vec![1, 15]),
        bound: (1, 31),
      }
    );

    // month
    assert_eq!(
      CronSlot::new("*", 1, 12),
      CronSlot {
        exp: CronExp::All,
        bound: (1, 12),
      }
    );

    // day of week
    assert_eq!(
      CronSlot::new("1-5", 0, 6),
      CronSlot {
        exp: CronExp::Range(1, 5),
        bound: (0, 6),
      }
    );
  }
}
