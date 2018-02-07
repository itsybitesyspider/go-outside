extern crate regex;
extern crate reqwest;
extern crate serde_json;

use regex::Regex;
use serde_json::{Value};

#[derive(Debug, Clone)]
struct WeatherPeriod {
  n : usize,
  score : f64,
  human_key : String,
  human_value: String,
}

fn main() {
  let uri = "https://api.weather.gov/points/35.7796,-78.6382/forecast";

  let mut resp = reqwest::get(uri).unwrap();
  assert!(resp.status().is_success());
  let json : Value = resp.json().unwrap();
  let periods = json["properties"]["periods"].as_array().unwrap();
  let mut results = vec![];
  let mut count : usize = 0;

  for p in periods {
    results.push(WeatherPeriod {
      n: count,
      score: score(p),
      human_key: p["name"].as_str().unwrap().to_string(),
      human_value: p["detailedForecast"].as_str().unwrap().to_string(),
    });

    count = count+1;
  }

  let now = results[0].clone();
  let mut best = results[0].clone();

  for r in results {
    if r.score < best.score {
      best = r.clone();
    }
  }
  
  if now.n == best.n {
    println!("{}: {}", now.human_key, now.human_value);
  }
}

fn score(period : &Value) -> f64 {
  score_temperature(period["temperature"].as_f64().unwrap()) +
    score_conditions(period["detailedForecast"].as_str().unwrap())
}

fn score_temperature(fahrenheits : f64) -> f64 {
  score_heat(fahrenheits) +
    score_cold(fahrenheits)
}

fn score_heat(fahrenheits : f64) -> f64 {
  let ideal = 75.0;
  let danger = 105.0;
  let range = danger - ideal;

  ((fahrenheits - ideal)/range).max(0.0)
}

fn score_cold(fahrenheits : f64) -> f64 {
  let ideal = 68.0;
  let danger = 0.0;
  let range = ideal - danger;
  
  ((ideal - fahrenheits)/range).max(0.0)
}

fn score_conditions(str : &str) -> f64 {
  let re = Regex::new(r"[Rr]ain|[Ss]now|[Pp]recipitation|[Ss]howers|[Dd]rizzle").unwrap();
  if re.is_match(str) {
    return 1.0;
  }

  0.0
}

