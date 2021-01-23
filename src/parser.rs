use crate::cron::Cron;

pub fn parse(line: &str) -> String {
  if let Some(cron) = Cron::new(line) {
    cron.output()
  } else {
    "Failed to parse".into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample() -> Cron {
    Cron {
      minutes: vec![0, 15, 30, 45],
      hours: vec![0],
      days_of_month: vec![1, 15],
      months: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
      days_of_week: vec![1, 2, 3, 4, 5],
      command: "/usr/bin/find".into(),
    }
  }

  #[test]
  fn test_new() {
    let input = "*/15 0 1,15 * 1-5 /usr/bin/find";

    if let Some(output) = Cron::new(input) {
      assert_eq!(sample(), output);
    }
  }

  #[test]
  fn test_parse() {
    let input = "*/15 0 1,15 * 1-5 /usr/bin/find";
    let output = parse(input);
    let expected = sample().output();

    assert_eq!(expected, output);
  }
}
