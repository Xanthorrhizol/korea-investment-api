use crate::Error;
use chrono::{DateTime, LocalResult};

#[derive(Debug, Clone)]
pub struct Time(DateTime<chrono_tz::Tz>);

impl Time {
    pub fn now() -> Self {
        Self(chrono::Utc::now().with_timezone(&chrono_tz::Asia::Seoul))
    }
    pub fn parse(s: &str, fmt: &str) -> Result<Self, Error> {
        let naive_dt = chrono::NaiveDateTime::parse_from_str(s, fmt)?;
        match naive_dt.and_local_timezone(chrono_tz::Asia::Seoul) {
            LocalResult::None => Err(Error::InvalidData),
            LocalResult::Single(t) => Ok(Time(t)),
            LocalResult::Ambiguous(_, _) => Err(Error::InvalidData),
        }
    }
    pub fn inner(&self) -> DateTime<chrono_tz::Tz> {
        self.0.clone()
    }
    pub fn date(&self) -> String {
        format!("{}", self.0.format("%Y%m%d"))
    }
}
