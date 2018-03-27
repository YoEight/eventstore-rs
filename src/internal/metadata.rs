use std::borrow::Borrow;
use std::collections::HashMap;

use serde::de::{ self, Visitor, Deserialize, Deserializer };
use serde::ser::{ Serialize, Serializer };
use serde_json::{ Value, to_value };
use time::Duration;
use internal::acl::StreamAcl;

// Hopefully `rustc` will consider this struct as a newtype.
pub struct DurationWrapper {
    pub inner: Duration,
}

impl DurationWrapper {
    fn new(inner: Duration) -> DurationWrapper {
        DurationWrapper {
            inner
        }
    }
}

impl Serialize for DurationWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(duration_str_repr(&self.inner).borrow())
    }
}

impl <'de> Deserialize<'de> for DurationWrapper {
    fn deserialize<D>(deserializer: D) -> Result<DurationWrapper, D::Error>
        where D: Deserializer<'de>
    {
        let duration = deserializer.deserialize_str(ForDuration)?;
        let res      = DurationWrapper { inner: duration };

        Ok(res)
    }
}

#[derive(Default)]
pub struct Builder {
    max_count: Option<u64>,
    max_age: Option<Duration>,
    truncate_before: Option<u64>,
    cache_control: Option<Duration>,
    acl: Option<StreamAcl>,
    properties: HashMap<String, Value>,
}

impl Builder {
    pub fn new() -> Builder {
        Default::default()
    }

    pub fn max_count(&mut self, value: u64) -> &mut Builder {
        self.max_count = Some(value);

        self
    }

    pub fn max_age(&mut self, value: Duration) -> &mut Builder {
        self.max_age = Some(value);

        self
    }

    pub fn truncate_before(&mut self, value: u64) -> &mut Builder {
        self.truncate_before = Some(value);

        self
    }

    pub fn cache_control(&mut self, value: Duration) -> &mut Builder {
        self.cache_control = Some(value);

        self
    }

    pub fn acl(&mut self, value: StreamAcl) -> &mut Builder {
        self.acl = Some(value);

        self
    }

    pub fn insert_custom_property<V>(&mut self, key: String, value: V) -> &mut Builder
        where V: Serialize
    {
        let serialized = to_value(value).unwrap();
        let _          = self.properties.insert(key, serialized);

        self
    }

    pub fn build(self) -> StreamMetadata {
        StreamMetadata {
            max_count: self.max_count,
            max_age: self.max_age.map(|d| DurationWrapper::new(d)),
            truncate_before: self.truncate_before,
            cache_control: self.cache_control.map(|d| DurationWrapper::new(d)),
            acl: self.acl.unwrap_or_else(|| Default::default()),
            custom_properties: self.properties,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StreamMetadata {
    #[serde(rename = "$maxCount")]
    pub max_count: Option<u64>,

    #[serde(rename = "$maxAge")]
    pub max_age: Option<DurationWrapper>,

    #[serde(rename = "$tb")]
    pub truncate_before: Option<u64>,

    #[serde(rename = "$cacheControl")]
    pub cache_control: Option<DurationWrapper>,

    #[serde(rename = "$acl")]
    pub acl: StreamAcl,

    #[serde(flatten)]
    pub custom_properties: HashMap<String, Value>,
}

impl StreamMetadata {
    pub fn new_builder() -> Builder {
        Builder::new()
    }
}

fn duration_str_repr(duration: &Duration) -> String {
    let mut builder      = String::new();
    let     days         = duration.num_days();
    let     hours        = duration.num_hours();
    let     minutes      = duration.num_minutes();
    let     seconds      = duration.num_seconds();
    let     milliseconds = duration.num_milliseconds();

    if days > 0 {
        builder.push_str(format!("{}.", days).borrow());
    }

    builder.push_str(format!("{:02}:", hours).borrow());
    builder.push_str(format!("{:02}:", minutes).borrow());
    builder.push_str(format!("{:02}", seconds).borrow());

    if milliseconds > 0 {
        builder.push_str(format!(".{:07}", milliseconds).borrow());
    }

    builder
}

struct ForDuration;

enum Parse {
    Days,
    Hours,
    Minutes,
    Seconds,
    Fractions,
}

impl <'de> Visitor<'de> for ForDuration {
    type Value = Duration;

    fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(formatter, "a string representing a Duration")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
    {

        fn to_u32(vec: &Vec<u32>) -> u32 {
            if vec.is_empty() {
                return 0;
            }

            let mut exp    = (vec.len() as u32) - 1;
            let mut result = 0;

            for value in vec {
                result += value * 10_u32.pow(exp);
                exp    -= 1;
            }

            result
        }

        let mut state   = Parse::Days;
        let mut buffer  = Vec::new();
        let mut builder = Duration::zero();

        for c in value.chars() {
            match state {
                Parse::Days => {
                    if c == '.' {
                        let num = to_u32(&buffer);

                        builder = builder + Duration::days(num as i64);
                        buffer.clear();
                        state = Parse::Hours;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Hours => {
                    if c == ':' {
                        let num = to_u32(&buffer);

                        builder = builder + Duration::hours(num as i64);
                        buffer.clear();
                        state = Parse::Minutes;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Minutes => {
                    if c == ':' {
                        let num = to_u32(&buffer);

                        builder = builder + Duration::minutes(num as i64);
                        buffer.clear();
                        state = Parse::Seconds;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Seconds => {
                    if c == '.' {
                        let num = to_u32(&buffer);

                        builder = builder + Duration::seconds(num as i64);
                        buffer.clear();
                        state = Parse::Fractions;
                    } else {
                        buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                    }
                },

                Parse::Fractions => {
                    buffer.push(c.to_digit(BASE_10_RDX).unwrap());
                }
            }
        }

        let num = to_u32(&buffer);

        // In case the Duration string representation didn't contain fractions.
        if let Parse::Seconds = state {
            builder = builder + Duration::seconds(num as i64);
        } else {
            // Hopefully, this will works :-D
            // TODO - Confirm we can send/receive back from GetEventStore driver without loosing
            // any informations.
            builder = builder + Duration::microseconds(num as i64);
        }

        Ok(builder)
    }
}

const BASE_10_RDX: u32 = 10;
