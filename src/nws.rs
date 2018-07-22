use super::regex::Regex;
use super::serde_json::{Value};

use common::*;

pub fn interpret(json: &Value) -> Option<Vec<WeatherPeriod>> {
  let periods = json["properties"]["periods"].as_array()?;

  let mut results = vec![];
  let mut count : usize = 0;

  for p in periods {
    results.push(WeatherPeriod {
      n: count,
      score: score(p)?,
      human_key: p["name"].as_str()?.to_string(),
      human_value: p["detailedForecast"].as_str()?.to_string(),
    });

    count = count+1;
  }

  Some(results)
}

fn score(period : &Value) -> Option<f64> {
  Some(
    score_temperature(period["temperature"].as_f64()?) +
    score_conditions(period["detailedForecast"].as_str()?))
}

fn score_conditions(str : &str) -> f64 {
  let re = Regex::new("[Rr]ain|[Ss]now|[Pp]recipitation|[Ss]howers|[Dd]rizzle").unwrap();
  if re.is_match(str) {
    return 1.0;
  }

  0.0
}

