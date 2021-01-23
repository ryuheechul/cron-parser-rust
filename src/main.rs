mod cron;
mod cron_exp;
mod parser;

use parser::parse;
use std::env;

fn main() {
  let arg = env::args().skip(1).next();

  match arg {
    Some(a) => println!("{}", parse(&a)),
    _ => println!("no args were provided"),
  }
}
