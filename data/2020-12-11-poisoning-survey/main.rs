#[macro_use]
extern crate serde_derive;

use std::{fmt, io};

use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    #[serde(rename = "Timestamp")]
    #[serde(deserialize_with = "deserialize_timestamp")]
    #[serde(serialize_with = "serialize_timestamp")]
    timestamp: OffsetDateTime,

    #[serde(
        rename = "Do you use the standard library’s lock types (Mutex or RwLock) in any of your projects?"
    )]
    use_stdlib_locks: Option<String>,

    #[serde(
        rename = "Do you use locks from outside the standard library (such as from antidote or parking_lot) in any of your projects?"
    )]
    use_non_stdlib_locks: Option<String>,

    #[serde(
        rename = "Which of these is closest to how you acquire locks? (Select all that apply)"
    )]
    #[serde(deserialize_with = "deserialize_semi_colon_list")]
    #[serde(default)]
    acquire_locks: Vec<String>,

    #[serde(rename = "Have you ever thought carefully about lock poisoning in your projects?")]
    thought_carefully_about_locks: Option<String>,

    #[serde(
        rename = "If so, what do you want to happen when a lock is poisoned? (Select all that apply)"
    )]
    #[serde(deserialize_with = "deserialize_semi_colon_list")]
    #[serde(default)]
    when_lock_is_poisoned: Vec<String>,

    #[serde(
        rename = "Do you think you’ve benefited from the standard library’s lock types providing poisoning by default?"
    )]
    benefitted_from_poisoning: Option<String>,

    #[serde(
        rename = "Do you use the standard library’s lock types or their guards in the public API of any of your projects?"
    )]
    stdlib_locks_in_pub: Option<String>,

    #[serde(
        rename = "If so, what’s an example of how the standard library’s lock or guard types appear in the public API of any of your projects? (Please keep accidental information leakage in mind here and consider replacing names with placeholders)"
    )]
    example_of_stdlib_locks_in_pub: Option<String>,

    #[serde(
        rename = "How much friction do you think would be involved in migrating any of your projects from the standard library’s poisoning lock types to a non-poisoning lock crate like antidote or parking_lot? (That would mean replacing .lock().unwrap() with .lock())"
    )]
    friction_in_migrating_from_poisoning: Option<String>,

    #[serde(
        rename = "Why do you use locks from outside the standard library? (Select all that apply)"
    )]
    #[serde(deserialize_with = "deserialize_semi_colon_list")]
    #[serde(default)]
    why_non_stdlib_locks: Vec<String>,

    #[serde(rename = "Do you implement poisoning some other way?")]
    alt_poisoning_impl: Option<String>,

    #[serde(
        rename = "If so, what do you want to happen when data is poisoned? (Select all that apply)"
    )]
    #[serde(deserialize_with = "deserialize_semi_colon_list")]
    #[serde(default)]
    when_alt_poisoning_is_poisoned: Vec<String>,

    #[serde(
        rename = "Would you use a poisoning implementation from the standard library if it was independent of Mutex or RwLock?"
    )]
    use_stdlib_alt_poisoning: Option<String>,
}

fn deserialize_semi_colon_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct SeparatedListVisitor(char);

    impl<'de> Visitor<'de> for SeparatedListVisitor {
        type Value = Vec<String>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a string of values separated by `{}`", self.0)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                return Ok(Vec::new());
            }

            Ok(v.split(self.0).map(|v| v.to_owned()).collect())
        }
    }

    deserializer.deserialize_str(SeparatedListVisitor(';'))
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct TimestampVisitor;

    impl<'de> Visitor<'de> for TimestampVisitor {
        type Value = OffsetDateTime;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a timestamp string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut parts = v.split("GMT");
            let dt = parts
                .next()
                .ok_or_else(|| E::custom("missing datetime part of timestamp"))?
                .trim();
            let offset = parts
                .next()
                .ok_or_else(|| E::custom("missing offset part of timestamp"))?
                .trim();

            let dt = time::PrimitiveDateTime::parse(dt, "%Y/%m/%d %-I:%M:%S %P")
                .map_err(|e| E::custom(e))?;

            Ok(dt.assume_offset(time::UtcOffset::hours({
                if offset.starts_with("+") {
                    &offset[1..]
                } else {
                    offset
                }
                .parse()
                .map_err(|e| E::custom(e))
            }?)))
        }
    }

    deserializer.deserialize_str(TimestampVisitor)
}

fn serialize_timestamp<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(&value.lazy_format("%Y-%0m-%0dT%0H:%0M:%0S%z"))
}

fn main() {
    let results = csv::ReaderBuilder::new()
        .from_path("poisoning-survey.csv")
        .expect("failed to create csv reader over results file")
        .deserialize()
        .map(|row| row.expect("failed to deserialize a row"))
        .collect::<Vec<Response>>();

    let stdout = io::stdout();
    serde_json::to_writer(stdout.lock(), &results).expect("failed to write JSON");
}
