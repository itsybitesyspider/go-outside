extern crate clap;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

mod common;
mod nws;

use serde_json::{Value};
use std::env;

struct Arguments {
  now : bool,
  best : bool,
  verbose : bool,
  better_than : f64,
  worse_than : f64,
  latitude: f64,
  longitude: f64
}

fn parse_args() -> Result<Arguments,String> {
  let mut result = Arguments {
    now : false,
    best : false,
    verbose : false,
    better_than : std::f64::INFINITY,
    worse_than : -std::f64::INFINITY,
    latitude: 0.0,
    longitude: 0.0,
  };

  let args : Vec<String> = env::args().collect();

  let mut i = 1;
  while i < args.len() {
    match args[i].as_ref() {
      "--now" => {
        result.now = true;
        i += 1;
      },
      "--best" => {
        result.best = true;
        i += 1;
      },
      "--verbose" => {
        result.verbose = true;
        i += 1;
      },
      "--worse-than" => {
        result.worse_than = args[i+1].parse::<f64>().map_err(|_| format!("--worse-than minimum score must be a number"))? / 100.0;
        i += 2;
      },
      "--better-than" => {
        result.better_than = args[i+1].parse::<f64>().map_err(|_| format!("--better-than maximum score must be a number"))? / 100.0;
        i += 2;
      },
      "--coordinates" => {
        let latitude = args[i+1].parse().map_err(|_| format!("--coordinates lat long, latitude must be a number"))?;
        let longitude = args[i+2].parse().map_err(|_| format!("--coordinates lat long, longitude must be a number"))?;
        result.latitude = latitude;
        result.longitude = longitude;
        i += 3;
      },
      arg => {
        return Err(format!("unrecognized argument: {}", arg));
      }
    }
  }

  Ok(result)
}

fn main() {
  let args = parse_args().unwrap();
  let uri : String = format!("https://api.weather.gov/points/{},{}/forecast", args.latitude, args.longitude);

  if args.verbose {
    println!("{}", &uri);
  }

  let mut resp = reqwest::get(&uri).unwrap();

  if args.verbose {
    println!("Response text: {}", resp.text().unwrap());
  }

  assert!(resp.status().is_success());
  let json : Value = resp.json().unwrap();

  let results = nws::interpret(&json).ok_or(format!("unexpected json response:\n {}", json)).unwrap();

  let now = common::get_now(&results).clone();
  let best = common::get_best(&results).clone();

  for result in results {
    if args.now && result.n != now.n {
      continue;
    }

    if args.best && result.n != best.n {
      continue;
    }

    if result.score <= args.worse_than {
      continue;
    }

    if result.score >= args.better_than {
      continue;
    }

    println!("({2}) {0}: {1}", result.human_key, result.human_value, (result.score*100.0).round());
  }
}

