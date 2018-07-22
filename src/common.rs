#[derive(Debug, Clone)]
pub struct WeatherPeriod {
  pub n : usize,
  pub score : f64,
  pub human_key : String,
  pub human_value: String,
}

pub fn get_now(weather_periods : &Vec<WeatherPeriod>) -> &WeatherPeriod {
  &weather_periods[0]
}

pub fn get_best(weather_periods : &Vec<WeatherPeriod>) -> &WeatherPeriod {
  let mut best = &weather_periods[0];

  for r in weather_periods {
    if r.score < best.score {
      best = &r;
    }
  }

  &best
}

pub fn score_temperature(fahrenheits : f64) -> f64 {
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

