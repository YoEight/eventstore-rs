#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(oneof = "uuid::Value", tags = "1, 2")]
    pub value: ::std::option::Option<uuid::Value>,
}
pub mod uuid {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Structured {
        #[prost(int64, tag = "1")]
        pub most_significant_bits: i64,
        #[prost(int64, tag = "2")]
        pub least_significant_bits: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Structured(Structured),
        #[prost(string, tag = "2")]
        String(std::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamIdentifier {
    #[prost(bytes, tag = "3")]
    pub stream_name: std::vec::Vec<u8>,
}
