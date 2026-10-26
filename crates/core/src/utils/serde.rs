use chrono::{
  DateTime,
  NaiveDate,
  NaiveDateTime,
  Utc,
};
use serde::{
  Deserialize,
  Deserializer
};

pub fn single_or_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
  T: Deserialize<'de>,
  D: Deserializer<'de>,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum Helper<T> {
    Single(T),
    Multiple(Vec<T>),
  }

  let helper = Helper::deserialize(deserializer)?;
  match helper {
    Helper::Single(val) => Ok(vec!(val)),
    Helper::Multiple(vals) => Ok(vals),
  }
}

pub fn date_or_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
  D: Deserializer<'de>,
{
  let opt_value: Option<toml::Value> = Option::deserialize(deserializer)?;
  let value = match opt_value {
    Some(v) => v,
    None => return Ok(None),
  };

  let date_str = match value {
    toml::Value::String(s) => s,
    toml::Value::Datetime(dt) => dt.to_string(),
    _ => {
      return Err(serde::de::Error::custom("Unknown date format"));
    },
  };

  if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
    return Ok(Some(dt.with_timezone(&Utc)));
  }

  if let Ok(naive_dt) = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S") {
    return Ok(Some(naive_dt.and_utc()));
  }

  if let Ok(naive_date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
    return Ok(Some(naive_date.and_hms_opt(0, 0, 0).unwrap().and_utc()));
  };

  Err(serde::de::Error::custom("Could not parse date"))
}
