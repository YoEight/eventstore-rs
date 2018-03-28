use std::collections::HashMap;

use serde::ser::Serialize;
use serde_json::{ Value, to_value };
use internal::acl::StreamAcl;
use internal::timespan::Timespan;

#[derive(Default)]
pub struct Builder {
    max_count: Option<u64>,
    max_age: Option<Timespan>,
    truncate_before: Option<u64>,
    cache_control: Option<Timespan>,
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

    pub fn max_age(&mut self, value: Timespan) -> &mut Builder {
        self.max_age = Some(value);

        self
    }

    pub fn truncate_before(&mut self, value: u64) -> &mut Builder {
        self.truncate_before = Some(value);

        self
    }

    pub fn cache_control(&mut self, value: Timespan) -> &mut Builder {
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
            max_age: self.max_age,
            truncate_before: self.truncate_before,
            cache_control: self.cache_control,
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
    pub max_age: Option<Timespan>,

    #[serde(rename = "$tb")]
    pub truncate_before: Option<u64>,

    #[serde(rename = "$cacheControl")]
    pub cache_control: Option<Timespan>,

    #[serde(rename = "$acl")]
    pub acl: StreamAcl,

    #[serde(flatten)]
    pub custom_properties: HashMap<String, Value>,
}

impl StreamMetadata {
    pub fn builder() -> Builder {
        Builder::new()
    }
}
