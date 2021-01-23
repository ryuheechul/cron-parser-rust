#[derive(Debug, PartialEq)]
pub enum NumberOrAny {
  Number(u8),
  Any,
}

#[derive(Debug, PartialEq)]
pub enum CronExp {
  Range(u8, u8),         // in case of 1-2
  List(Vec<u8>),         // in case of `1,2,3,*` this should be any
  All,                   // in case of *
  Step(NumberOrAny, u8), // in case of */5 or 2/5
  Invalid,               // in case of things are not parse-able
}

impl CronExp {
  fn try_range(text: &str) -> Option<CronExp> {
    let range: Vec<&str> = text.split("-").collect();

    if range.len() != 2 {
      return None;
    }

    let numified: Option<Vec<u8>> = range.iter().map(|&s| s.parse::<u8>().ok()).collect();

    let nv = numified?;

    Some(CronExp::Range(nv[0], nv[1]))
  }

  fn try_list(text: &str) -> Option<CronExp> {
    let list: Vec<&str> = text.split(",").collect();

    if list.len() <= 1 {
      return None;
    }

    let numified: Option<Vec<u8>> = list.iter().map(|&s| s.parse::<u8>().ok()).collect();

    let nv = numified?;

    Some(CronExp::List(nv))
  }

  fn try_step(text: &str) -> Option<CronExp> {
    let step: Vec<&str> = text.split("/").collect();

    if step.len() != 2 {
      return None;
    }

    let left = match step[0] {
      "*" => Some(NumberOrAny::Any),
      _ => step[0].parse::<u8>().ok().map(|n| NumberOrAny::Number(n)),
    }?;

    let right = step[1].parse::<u8>().ok()?;

    Some(CronExp::Step(left, right))
  }

  fn try_single(text: &str) -> Option<CronExp> {
    let n = text.parse::<u8>().ok()?;
    Some(CronExp::List(vec![n]))
  }

  fn try_all(text: &str) -> Option<CronExp> {
    CronExp::try_range(text)
      .or(CronExp::try_list(text))
      .or(CronExp::try_step(text))
      .or(CronExp::try_single(text))
  }
}

impl From<&str> for CronExp {
  fn from(text: &str) -> Self {
    if text == "*" {
      return CronExp::All;
    }

    CronExp::try_all(text).unwrap_or(CronExp::Invalid)
  }
}
