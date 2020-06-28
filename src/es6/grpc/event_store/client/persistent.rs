#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadReq {
    #[prost(oneof = "read_req::Content", tags = "1, 2, 3")]
    pub content: ::std::option::Option<read_req::Content>,
}
pub mod read_req {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier: ::std::option::Option<super::super::shared::StreamIdentifier>,
        #[prost(string, tag = "2")]
        pub group_name: std::string::String,
        #[prost(int32, tag = "3")]
        pub buffer_size: i32,
        #[prost(message, optional, tag = "4")]
        pub uuid_option: ::std::option::Option<options::UuidOption>,
    }
    pub mod options {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UuidOption {
            #[prost(oneof = "uuid_option::Content", tags = "1, 2")]
            pub content: ::std::option::Option<uuid_option::Content>,
        }
        pub mod uuid_option {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Content {
                #[prost(message, tag = "1")]
                Structured(super::super::super::super::shared::Empty),
                #[prost(message, tag = "2")]
                String(super::super::super::super::shared::Empty),
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ack {
        #[prost(bytes, tag = "1")]
        pub id: std::vec::Vec<u8>,
        #[prost(message, repeated, tag = "2")]
        pub ids: ::std::vec::Vec<super::super::shared::Uuid>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Nack {
        #[prost(bytes, tag = "1")]
        pub id: std::vec::Vec<u8>,
        #[prost(message, repeated, tag = "2")]
        pub ids: ::std::vec::Vec<super::super::shared::Uuid>,
        #[prost(enumeration = "nack::Action", tag = "3")]
        pub action: i32,
        #[prost(string, tag = "4")]
        pub reason: std::string::String,
    }
    pub mod nack {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Action {
            Unknown = 0,
            Park = 1,
            Retry = 2,
            Skip = 3,
            Stop = 4,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        Options(Options),
        #[prost(message, tag = "2")]
        Ack(Ack),
        #[prost(message, tag = "3")]
        Nack(Nack),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResp {
    #[prost(oneof = "read_resp::Content", tags = "1, 2")]
    pub content: ::std::option::Option<read_resp::Content>,
}
pub mod read_resp {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReadEvent {
        #[prost(message, optional, tag = "1")]
        pub event: ::std::option::Option<read_event::RecordedEvent>,
        #[prost(message, optional, tag = "2")]
        pub link: ::std::option::Option<read_event::RecordedEvent>,
        #[prost(oneof = "read_event::Position", tags = "3, 4")]
        pub position: ::std::option::Option<read_event::Position>,
        #[prost(oneof = "read_event::Count", tags = "5, 6")]
        pub count: ::std::option::Option<read_event::Count>,
    }
    pub mod read_event {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RecordedEvent {
            #[prost(message, optional, tag = "1")]
            pub id: ::std::option::Option<super::super::super::shared::Uuid>,
            #[prost(message, optional, tag = "2")]
            pub stream_identifier:
                ::std::option::Option<super::super::super::shared::StreamIdentifier>,
            #[prost(uint64, tag = "3")]
            pub stream_revision: u64,
            #[prost(uint64, tag = "4")]
            pub prepare_position: u64,
            #[prost(uint64, tag = "5")]
            pub commit_position: u64,
            #[prost(map = "string, string", tag = "6")]
            pub metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
            #[prost(bytes, tag = "7")]
            pub custom_metadata: std::vec::Vec<u8>,
            #[prost(bytes, tag = "8")]
            pub data: std::vec::Vec<u8>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Position {
            #[prost(uint64, tag = "3")]
            CommitPosition(u64),
            #[prost(message, tag = "4")]
            NoPosition(super::super::super::shared::Empty),
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Count {
            #[prost(int32, tag = "5")]
            RetryCount(i32),
            #[prost(message, tag = "6")]
            NoRetryCount(super::super::super::shared::Empty),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubscriptionConfirmation {
        #[prost(string, tag = "1")]
        pub subscription_id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        Event(ReadEvent),
        #[prost(message, tag = "2")]
        SubscriptionConfirmation(SubscriptionConfirmation),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::std::option::Option<create_req::Options>,
}
pub mod create_req {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier: ::std::option::Option<super::super::shared::StreamIdentifier>,
        #[prost(string, tag = "2")]
        pub group_name: std::string::String,
        #[prost(message, optional, tag = "3")]
        pub settings: ::std::option::Option<Settings>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Settings {
        #[prost(bool, tag = "1")]
        pub resolve_links: bool,
        #[prost(uint64, tag = "2")]
        pub revision: u64,
        #[prost(bool, tag = "3")]
        pub extra_statistics: bool,
        #[prost(int64, tag = "4")]
        pub message_timeout: i64,
        #[prost(int32, tag = "5")]
        pub max_retry_count: i32,
        #[prost(int64, tag = "6")]
        pub checkpoint_after: i64,
        #[prost(int32, tag = "7")]
        pub min_checkpoint_count: i32,
        #[prost(int32, tag = "8")]
        pub max_checkpoint_count: i32,
        #[prost(int32, tag = "9")]
        pub max_subscriber_count: i32,
        #[prost(int32, tag = "10")]
        pub live_buffer_size: i32,
        #[prost(int32, tag = "11")]
        pub read_batch_size: i32,
        #[prost(int32, tag = "12")]
        pub history_buffer_size: i32,
        #[prost(enumeration = "ConsumerStrategy", tag = "13")]
        pub named_consumer_strategy: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConsumerStrategy {
        DispatchToSingle = 0,
        RoundRobin = 1,
        Pinned = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::std::option::Option<update_req::Options>,
}
pub mod update_req {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier: ::std::option::Option<super::super::shared::StreamIdentifier>,
        #[prost(string, tag = "2")]
        pub group_name: std::string::String,
        #[prost(message, optional, tag = "3")]
        pub settings: ::std::option::Option<Settings>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Settings {
        #[prost(bool, tag = "1")]
        pub resolve_links: bool,
        #[prost(uint64, tag = "2")]
        pub revision: u64,
        #[prost(bool, tag = "3")]
        pub extra_statistics: bool,
        #[prost(int64, tag = "4")]
        pub message_timeout: i64,
        #[prost(int32, tag = "5")]
        pub max_retry_count: i32,
        #[prost(int64, tag = "6")]
        pub checkpoint_after: i64,
        #[prost(int32, tag = "7")]
        pub min_checkpoint_count: i32,
        #[prost(int32, tag = "8")]
        pub max_checkpoint_count: i32,
        #[prost(int32, tag = "9")]
        pub max_subscriber_count: i32,
        #[prost(int32, tag = "10")]
        pub live_buffer_size: i32,
        #[prost(int32, tag = "11")]
        pub read_batch_size: i32,
        #[prost(int32, tag = "12")]
        pub history_buffer_size: i32,
        #[prost(enumeration = "ConsumerStrategy", tag = "13")]
        pub named_consumer_strategy: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConsumerStrategy {
        DispatchToSingle = 0,
        RoundRobin = 1,
        Pinned = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResp {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReq {
    #[prost(message, optional, tag = "1")]
    pub options: ::std::option::Option<delete_req::Options>,
}
pub mod delete_req {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        #[prost(message, optional, tag = "1")]
        pub stream_identifier: ::std::option::Option<super::super::shared::StreamIdentifier>,
        #[prost(string, tag = "2")]
        pub group_name: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResp {}
#[doc = r" Generated client implementations."]
pub mod persistent_subscriptions_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PersistentSubscriptionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PersistentSubscriptionsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PersistentSubscriptionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReq>,
        ) -> Result<tonic::Response<super::CreateResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReq>,
        ) -> Result<tonic::Response<super::UpdateResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReq>,
        ) -> Result<tonic::Response<super::DeleteResp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ReadReq>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadResp>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/event_store.client.persistent_subscriptions.PersistentSubscriptions/Read",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for PersistentSubscriptionsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PersistentSubscriptionsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PersistentSubscriptionsClient {{ ... }}")
        }
    }
}
