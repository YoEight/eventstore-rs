// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct NewEvent {
    // message fields
    event_id: ::std::option::Option<::bytes::Bytes>,
    event_type: ::std::option::Option<::protobuf::chars::Chars>,
    data_content_type: ::std::option::Option<i32>,
    metadata_content_type: ::std::option::Option<i32>,
    data: ::std::option::Option<::bytes::Bytes>,
    metadata: ::std::option::Option<::bytes::Bytes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewEvent {}

impl NewEvent {
    pub fn new() -> NewEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewEvent {
        static mut instance: ::protobuf::lazy::Lazy<NewEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewEvent,
        };
        unsafe {
            instance.get(NewEvent::new)
        }
    }

    // required bytes event_id = 1;

    pub fn clear_event_id(&mut self) {
        self.event_id = ::std::option::Option::None;
    }

    pub fn has_event_id(&self) -> bool {
        self.event_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_id(&mut self, v: ::bytes::Bytes) {
        self.event_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_id(&mut self) -> &mut ::bytes::Bytes {
        if self.event_id.is_none() {
            self.event_id = ::std::option::Option::Some(::bytes::Bytes::new());
        }
        self.event_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_id(&mut self) -> ::bytes::Bytes {
        self.event_id.take().unwrap_or_else(|| ::bytes::Bytes::new())
    }

    pub fn get_event_id(&self) -> &[u8] {
        match self.event_id.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    fn get_event_id_for_reflect(&self) -> &::std::option::Option<::bytes::Bytes> {
        &self.event_id
    }

    fn mut_event_id_for_reflect(&mut self) -> &mut ::std::option::Option<::bytes::Bytes> {
        &mut self.event_id
    }

    // required string event_type = 2;

    pub fn clear_event_type(&mut self) {
        self.event_type = ::std::option::Option::None;
    }

    pub fn has_event_type(&self) -> bool {
        self.event_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_type(&mut self, v: ::protobuf::chars::Chars) {
        self.event_type = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_type(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_type.is_none() {
            self.event_type = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_type(&mut self) -> ::protobuf::chars::Chars {
        self.event_type.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_type(&self) -> &str {
        match self.event_type.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_type_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_type
    }

    fn mut_event_type_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_type
    }

    // required int32 data_content_type = 3;

    pub fn clear_data_content_type(&mut self) {
        self.data_content_type = ::std::option::Option::None;
    }

    pub fn has_data_content_type(&self) -> bool {
        self.data_content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_content_type(&mut self, v: i32) {
        self.data_content_type = ::std::option::Option::Some(v);
    }

    pub fn get_data_content_type(&self) -> i32 {
        self.data_content_type.unwrap_or(0)
    }

    fn get_data_content_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.data_content_type
    }

    fn mut_data_content_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.data_content_type
    }

    // required int32 metadata_content_type = 4;

    pub fn clear_metadata_content_type(&mut self) {
        self.metadata_content_type = ::std::option::Option::None;
    }

    pub fn has_metadata_content_type(&self) -> bool {
        self.metadata_content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata_content_type(&mut self, v: i32) {
        self.metadata_content_type = ::std::option::Option::Some(v);
    }

    pub fn get_metadata_content_type(&self) -> i32 {
        self.metadata_content_type.unwrap_or(0)
    }

    fn get_metadata_content_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.metadata_content_type
    }

    fn mut_metadata_content_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.metadata_content_type
    }

    // required bytes data = 5;

    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::bytes::Bytes) {
        self.data = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::bytes::Bytes {
        if self.data.is_none() {
            self.data = ::std::option::Option::Some(::bytes::Bytes::new());
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::bytes::Bytes {
        self.data.take().unwrap_or_else(|| ::bytes::Bytes::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::std::option::Option<::bytes::Bytes> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::option::Option<::bytes::Bytes> {
        &mut self.data
    }

    // optional bytes metadata = 6;

    pub fn clear_metadata(&mut self) {
        self.metadata = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::bytes::Bytes) {
        self.metadata = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::bytes::Bytes {
        if self.metadata.is_none() {
            self.metadata = ::std::option::Option::Some(::bytes::Bytes::new());
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::bytes::Bytes {
        self.metadata.take().unwrap_or_else(|| ::bytes::Bytes::new())
    }

    pub fn get_metadata(&self) -> &[u8] {
        match self.metadata.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    fn get_metadata_for_reflect(&self) -> &::std::option::Option<::bytes::Bytes> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::std::option::Option<::bytes::Bytes> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for NewEvent {
    fn is_initialized(&self) -> bool {
        if self.event_id.is_none() {
            return false;
        }
        if self.event_type.is_none() {
            return false;
        }
        if self.data_content_type.is_none() {
            return false;
        }
        if self.metadata_content_type.is_none() {
            return false;
        }
        if self.data.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_bytes_into(wire_type, is, &mut self.event_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_type)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.data_content_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.metadata_content_type = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_carllerche_bytes_into(wire_type, is, &mut self.data)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_carllerche_bytes_into(wire_type, is, &mut self.metadata)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.event_type.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.data_content_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.metadata_content_type {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_id.as_ref() {
            os.write_bytes(1, v)?;
        }
        if let Some(ref v) = self.event_type.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.data_content_type {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.metadata_content_type {
            os.write_int32(4, v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(5, v)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_bytes(6, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NewEvent {
    fn new() -> NewEvent {
        NewEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "event_id",
                    NewEvent::get_event_id_for_reflect,
                    NewEvent::mut_event_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_type",
                    NewEvent::get_event_type_for_reflect,
                    NewEvent::mut_event_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "data_content_type",
                    NewEvent::get_data_content_type_for_reflect,
                    NewEvent::mut_data_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "metadata_content_type",
                    NewEvent::get_metadata_content_type_for_reflect,
                    NewEvent::mut_metadata_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "data",
                    NewEvent::get_data_for_reflect,
                    NewEvent::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "metadata",
                    NewEvent::get_metadata_for_reflect,
                    NewEvent::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NewEvent>(
                    "NewEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewEvent {
    fn clear(&mut self) {
        self.clear_event_id();
        self.clear_event_type();
        self.clear_data_content_type();
        self.clear_metadata_content_type();
        self.clear_data();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventRecord {
    // message fields
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    event_number: ::std::option::Option<i64>,
    event_id: ::std::option::Option<::bytes::Bytes>,
    event_type: ::std::option::Option<::protobuf::chars::Chars>,
    data_content_type: ::std::option::Option<i32>,
    metadata_content_type: ::std::option::Option<i32>,
    data: ::std::option::Option<::bytes::Bytes>,
    metadata: ::std::option::Option<::bytes::Bytes>,
    created: ::std::option::Option<i64>,
    created_epoch: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EventRecord {}

impl EventRecord {
    pub fn new() -> EventRecord {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventRecord {
        static mut instance: ::protobuf::lazy::Lazy<EventRecord> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventRecord,
        };
        unsafe {
            instance.get(EventRecord::new)
        }
    }

    // required string event_stream_id = 1;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required int64 event_number = 2;

    pub fn clear_event_number(&mut self) {
        self.event_number = ::std::option::Option::None;
    }

    pub fn has_event_number(&self) -> bool {
        self.event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_number(&mut self, v: i64) {
        self.event_number = ::std::option::Option::Some(v);
    }

    pub fn get_event_number(&self) -> i64 {
        self.event_number.unwrap_or(0)
    }

    fn get_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.event_number
    }

    fn mut_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.event_number
    }

    // required bytes event_id = 3;

    pub fn clear_event_id(&mut self) {
        self.event_id = ::std::option::Option::None;
    }

    pub fn has_event_id(&self) -> bool {
        self.event_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_id(&mut self, v: ::bytes::Bytes) {
        self.event_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_id(&mut self) -> &mut ::bytes::Bytes {
        if self.event_id.is_none() {
            self.event_id = ::std::option::Option::Some(::bytes::Bytes::new());
        }
        self.event_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_id(&mut self) -> ::bytes::Bytes {
        self.event_id.take().unwrap_or_else(|| ::bytes::Bytes::new())
    }

    pub fn get_event_id(&self) -> &[u8] {
        match self.event_id.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    fn get_event_id_for_reflect(&self) -> &::std::option::Option<::bytes::Bytes> {
        &self.event_id
    }

    fn mut_event_id_for_reflect(&mut self) -> &mut ::std::option::Option<::bytes::Bytes> {
        &mut self.event_id
    }

    // required string event_type = 4;

    pub fn clear_event_type(&mut self) {
        self.event_type = ::std::option::Option::None;
    }

    pub fn has_event_type(&self) -> bool {
        self.event_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_type(&mut self, v: ::protobuf::chars::Chars) {
        self.event_type = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_type(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_type.is_none() {
            self.event_type = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_type(&mut self) -> ::protobuf::chars::Chars {
        self.event_type.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_type(&self) -> &str {
        match self.event_type.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_type_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_type
    }

    fn mut_event_type_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_type
    }

    // required int32 data_content_type = 5;

    pub fn clear_data_content_type(&mut self) {
        self.data_content_type = ::std::option::Option::None;
    }

    pub fn has_data_content_type(&self) -> bool {
        self.data_content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_content_type(&mut self, v: i32) {
        self.data_content_type = ::std::option::Option::Some(v);
    }

    pub fn get_data_content_type(&self) -> i32 {
        self.data_content_type.unwrap_or(0)
    }

    fn get_data_content_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.data_content_type
    }

    fn mut_data_content_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.data_content_type
    }

    // required int32 metadata_content_type = 6;

    pub fn clear_metadata_content_type(&mut self) {
        self.metadata_content_type = ::std::option::Option::None;
    }

    pub fn has_metadata_content_type(&self) -> bool {
        self.metadata_content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata_content_type(&mut self, v: i32) {
        self.metadata_content_type = ::std::option::Option::Some(v);
    }

    pub fn get_metadata_content_type(&self) -> i32 {
        self.metadata_content_type.unwrap_or(0)
    }

    fn get_metadata_content_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.metadata_content_type
    }

    fn mut_metadata_content_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.metadata_content_type
    }

    // required bytes data = 7;

    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::bytes::Bytes) {
        self.data = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::bytes::Bytes {
        if self.data.is_none() {
            self.data = ::std::option::Option::Some(::bytes::Bytes::new());
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::bytes::Bytes {
        self.data.take().unwrap_or_else(|| ::bytes::Bytes::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::std::option::Option<::bytes::Bytes> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::option::Option<::bytes::Bytes> {
        &mut self.data
    }

    // optional bytes metadata = 8;

    pub fn clear_metadata(&mut self) {
        self.metadata = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::bytes::Bytes) {
        self.metadata = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::bytes::Bytes {
        if self.metadata.is_none() {
            self.metadata = ::std::option::Option::Some(::bytes::Bytes::new());
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::bytes::Bytes {
        self.metadata.take().unwrap_or_else(|| ::bytes::Bytes::new())
    }

    pub fn get_metadata(&self) -> &[u8] {
        match self.metadata.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    fn get_metadata_for_reflect(&self) -> &::std::option::Option<::bytes::Bytes> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::std::option::Option<::bytes::Bytes> {
        &mut self.metadata
    }

    // optional int64 created = 9;

    pub fn clear_created(&mut self) {
        self.created = ::std::option::Option::None;
    }

    pub fn has_created(&self) -> bool {
        self.created.is_some()
    }

    // Param is passed by value, moved
    pub fn set_created(&mut self, v: i64) {
        self.created = ::std::option::Option::Some(v);
    }

    pub fn get_created(&self) -> i64 {
        self.created.unwrap_or(0)
    }

    fn get_created_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.created
    }

    fn mut_created_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.created
    }

    // optional int64 created_epoch = 10;

    pub fn clear_created_epoch(&mut self) {
        self.created_epoch = ::std::option::Option::None;
    }

    pub fn has_created_epoch(&self) -> bool {
        self.created_epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_created_epoch(&mut self, v: i64) {
        self.created_epoch = ::std::option::Option::Some(v);
    }

    pub fn get_created_epoch(&self) -> i64 {
        self.created_epoch.unwrap_or(0)
    }

    fn get_created_epoch_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.created_epoch
    }

    fn mut_created_epoch_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.created_epoch
    }
}

impl ::protobuf::Message for EventRecord {
    fn is_initialized(&self) -> bool {
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.event_number.is_none() {
            return false;
        }
        if self.event_id.is_none() {
            return false;
        }
        if self.event_type.is_none() {
            return false;
        }
        if self.data_content_type.is_none() {
            return false;
        }
        if self.metadata_content_type.is_none() {
            return false;
        }
        if self.data.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.event_number = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_carllerche_bytes_into(wire_type, is, &mut self.event_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_type)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.data_content_type = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.metadata_content_type = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_carllerche_bytes_into(wire_type, is, &mut self.data)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_carllerche_bytes_into(wire_type, is, &mut self.metadata)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.created = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.created_epoch = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.event_number {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.event_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.event_type.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.data_content_type {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.metadata_content_type {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        }
        if let Some(v) = self.created {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.created_epoch {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.event_number {
            os.write_int64(2, v)?;
        }
        if let Some(ref v) = self.event_id.as_ref() {
            os.write_bytes(3, v)?;
        }
        if let Some(ref v) = self.event_type.as_ref() {
            os.write_string(4, v)?;
        }
        if let Some(v) = self.data_content_type {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.metadata_content_type {
            os.write_int32(6, v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(7, v)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_bytes(8, v)?;
        }
        if let Some(v) = self.created {
            os.write_int64(9, v)?;
        }
        if let Some(v) = self.created_epoch {
            os.write_int64(10, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EventRecord {
    fn new() -> EventRecord {
        EventRecord::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventRecord>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    EventRecord::get_event_stream_id_for_reflect,
                    EventRecord::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "event_number",
                    EventRecord::get_event_number_for_reflect,
                    EventRecord::mut_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "event_id",
                    EventRecord::get_event_id_for_reflect,
                    EventRecord::mut_event_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_type",
                    EventRecord::get_event_type_for_reflect,
                    EventRecord::mut_event_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "data_content_type",
                    EventRecord::get_data_content_type_for_reflect,
                    EventRecord::mut_data_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "metadata_content_type",
                    EventRecord::get_metadata_content_type_for_reflect,
                    EventRecord::mut_metadata_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "data",
                    EventRecord::get_data_for_reflect,
                    EventRecord::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "metadata",
                    EventRecord::get_metadata_for_reflect,
                    EventRecord::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "created",
                    EventRecord::get_created_for_reflect,
                    EventRecord::mut_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "created_epoch",
                    EventRecord::get_created_epoch_for_reflect,
                    EventRecord::mut_created_epoch_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventRecord>(
                    "EventRecord",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventRecord {
    fn clear(&mut self) {
        self.clear_event_stream_id();
        self.clear_event_number();
        self.clear_event_id();
        self.clear_event_type();
        self.clear_data_content_type();
        self.clear_metadata_content_type();
        self.clear_data();
        self.clear_metadata();
        self.clear_created();
        self.clear_created_epoch();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventRecord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventRecord {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResolvedIndexedEvent {
    // message fields
    event: ::protobuf::SingularPtrField<EventRecord>,
    link: ::protobuf::SingularPtrField<EventRecord>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResolvedIndexedEvent {}

impl ResolvedIndexedEvent {
    pub fn new() -> ResolvedIndexedEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResolvedIndexedEvent {
        static mut instance: ::protobuf::lazy::Lazy<ResolvedIndexedEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResolvedIndexedEvent,
        };
        unsafe {
            instance.get(ResolvedIndexedEvent::new)
        }
    }

    // required .EventStore.Client.Messages.EventRecord event = 1;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: EventRecord) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut EventRecord {
        if self.event.is_none() {
            self.event.set_default();
        }
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> EventRecord {
        self.event.take().unwrap_or_else(|| EventRecord::new())
    }

    pub fn get_event(&self) -> &EventRecord {
        self.event.as_ref().unwrap_or_else(|| EventRecord::default_instance())
    }

    fn get_event_for_reflect(&self) -> &::protobuf::SingularPtrField<EventRecord> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EventRecord> {
        &mut self.event
    }

    // optional .EventStore.Client.Messages.EventRecord link = 2;

    pub fn clear_link(&mut self) {
        self.link.clear();
    }

    pub fn has_link(&self) -> bool {
        self.link.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link(&mut self, v: EventRecord) {
        self.link = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_link(&mut self) -> &mut EventRecord {
        if self.link.is_none() {
            self.link.set_default();
        }
        self.link.as_mut().unwrap()
    }

    // Take field
    pub fn take_link(&mut self) -> EventRecord {
        self.link.take().unwrap_or_else(|| EventRecord::new())
    }

    pub fn get_link(&self) -> &EventRecord {
        self.link.as_ref().unwrap_or_else(|| EventRecord::default_instance())
    }

    fn get_link_for_reflect(&self) -> &::protobuf::SingularPtrField<EventRecord> {
        &self.link
    }

    fn mut_link_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EventRecord> {
        &mut self.link
    }
}

impl ::protobuf::Message for ResolvedIndexedEvent {
    fn is_initialized(&self) -> bool {
        if self.event.is_none() {
            return false;
        }
        for v in &self.event {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.link {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.link)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.link.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.link.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResolvedIndexedEvent {
    fn new() -> ResolvedIndexedEvent {
        ResolvedIndexedEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResolvedIndexedEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EventRecord>>(
                    "event",
                    ResolvedIndexedEvent::get_event_for_reflect,
                    ResolvedIndexedEvent::mut_event_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EventRecord>>(
                    "link",
                    ResolvedIndexedEvent::get_link_for_reflect,
                    ResolvedIndexedEvent::mut_link_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResolvedIndexedEvent>(
                    "ResolvedIndexedEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResolvedIndexedEvent {
    fn clear(&mut self) {
        self.clear_event();
        self.clear_link();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResolvedIndexedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolvedIndexedEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResolvedEvent {
    // message fields
    event: ::protobuf::SingularPtrField<EventRecord>,
    link: ::protobuf::SingularPtrField<EventRecord>,
    commit_position: ::std::option::Option<i64>,
    prepare_position: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResolvedEvent {}

impl ResolvedEvent {
    pub fn new() -> ResolvedEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResolvedEvent {
        static mut instance: ::protobuf::lazy::Lazy<ResolvedEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResolvedEvent,
        };
        unsafe {
            instance.get(ResolvedEvent::new)
        }
    }

    // required .EventStore.Client.Messages.EventRecord event = 1;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: EventRecord) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut EventRecord {
        if self.event.is_none() {
            self.event.set_default();
        }
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> EventRecord {
        self.event.take().unwrap_or_else(|| EventRecord::new())
    }

    pub fn get_event(&self) -> &EventRecord {
        self.event.as_ref().unwrap_or_else(|| EventRecord::default_instance())
    }

    fn get_event_for_reflect(&self) -> &::protobuf::SingularPtrField<EventRecord> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EventRecord> {
        &mut self.event
    }

    // optional .EventStore.Client.Messages.EventRecord link = 2;

    pub fn clear_link(&mut self) {
        self.link.clear();
    }

    pub fn has_link(&self) -> bool {
        self.link.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link(&mut self, v: EventRecord) {
        self.link = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_link(&mut self) -> &mut EventRecord {
        if self.link.is_none() {
            self.link.set_default();
        }
        self.link.as_mut().unwrap()
    }

    // Take field
    pub fn take_link(&mut self) -> EventRecord {
        self.link.take().unwrap_or_else(|| EventRecord::new())
    }

    pub fn get_link(&self) -> &EventRecord {
        self.link.as_ref().unwrap_or_else(|| EventRecord::default_instance())
    }

    fn get_link_for_reflect(&self) -> &::protobuf::SingularPtrField<EventRecord> {
        &self.link
    }

    fn mut_link_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EventRecord> {
        &mut self.link
    }

    // required int64 commit_position = 3;

    pub fn clear_commit_position(&mut self) {
        self.commit_position = ::std::option::Option::None;
    }

    pub fn has_commit_position(&self) -> bool {
        self.commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_position(&mut self, v: i64) {
        self.commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_commit_position(&self) -> i64 {
        self.commit_position.unwrap_or(0)
    }

    fn get_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.commit_position
    }

    fn mut_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.commit_position
    }

    // required int64 prepare_position = 4;

    pub fn clear_prepare_position(&mut self) {
        self.prepare_position = ::std::option::Option::None;
    }

    pub fn has_prepare_position(&self) -> bool {
        self.prepare_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prepare_position(&mut self, v: i64) {
        self.prepare_position = ::std::option::Option::Some(v);
    }

    pub fn get_prepare_position(&self) -> i64 {
        self.prepare_position.unwrap_or(0)
    }

    fn get_prepare_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.prepare_position
    }

    fn mut_prepare_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.prepare_position
    }
}

impl ::protobuf::Message for ResolvedEvent {
    fn is_initialized(&self) -> bool {
        if self.event.is_none() {
            return false;
        }
        if self.commit_position.is_none() {
            return false;
        }
        if self.prepare_position.is_none() {
            return false;
        }
        for v in &self.event {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.link {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.link)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_position = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.prepare_position = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.link.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.commit_position {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prepare_position {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.link.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.commit_position {
            os.write_int64(3, v)?;
        }
        if let Some(v) = self.prepare_position {
            os.write_int64(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResolvedEvent {
    fn new() -> ResolvedEvent {
        ResolvedEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResolvedEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EventRecord>>(
                    "event",
                    ResolvedEvent::get_event_for_reflect,
                    ResolvedEvent::mut_event_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EventRecord>>(
                    "link",
                    ResolvedEvent::get_link_for_reflect,
                    ResolvedEvent::mut_link_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_position",
                    ResolvedEvent::get_commit_position_for_reflect,
                    ResolvedEvent::mut_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "prepare_position",
                    ResolvedEvent::get_prepare_position_for_reflect,
                    ResolvedEvent::mut_prepare_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResolvedEvent>(
                    "ResolvedEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResolvedEvent {
    fn clear(&mut self) {
        self.clear_event();
        self.clear_link();
        self.clear_commit_position();
        self.clear_prepare_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResolvedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolvedEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteEvents {
    // message fields
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    expected_version: ::std::option::Option<i64>,
    events: ::protobuf::RepeatedField<NewEvent>,
    require_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteEvents {}

impl WriteEvents {
    pub fn new() -> WriteEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteEvents {
        static mut instance: ::protobuf::lazy::Lazy<WriteEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteEvents,
        };
        unsafe {
            instance.get(WriteEvents::new)
        }
    }

    // required string event_stream_id = 1;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required int64 expected_version = 2;

    pub fn clear_expected_version(&mut self) {
        self.expected_version = ::std::option::Option::None;
    }

    pub fn has_expected_version(&self) -> bool {
        self.expected_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expected_version(&mut self, v: i64) {
        self.expected_version = ::std::option::Option::Some(v);
    }

    pub fn get_expected_version(&self) -> i64 {
        self.expected_version.unwrap_or(0)
    }

    fn get_expected_version_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.expected_version
    }

    fn mut_expected_version_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.expected_version
    }

    // repeated .EventStore.Client.Messages.NewEvent events = 3;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<NewEvent>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<NewEvent> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<NewEvent> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[NewEvent] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<NewEvent> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NewEvent> {
        &mut self.events
    }

    // required bool require_master = 4;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }
}

impl ::protobuf::Message for WriteEvents {
    fn is_initialized(&self) -> bool {
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.expected_version.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        for v in &self.events {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expected_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.expected_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.expected_version {
            os.write_int64(2, v)?;
        }
        for v in &self.events {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.require_master {
            os.write_bool(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteEvents {
    fn new() -> WriteEvents {
        WriteEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    WriteEvents::get_event_stream_id_for_reflect,
                    WriteEvents::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expected_version",
                    WriteEvents::get_expected_version_for_reflect,
                    WriteEvents::mut_expected_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NewEvent>>(
                    "events",
                    WriteEvents::get_events_for_reflect,
                    WriteEvents::mut_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    WriteEvents::get_require_master_for_reflect,
                    WriteEvents::mut_require_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteEvents>(
                    "WriteEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteEvents {
    fn clear(&mut self) {
        self.clear_event_stream_id();
        self.clear_expected_version();
        self.clear_events();
        self.clear_require_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteEventsCompleted {
    // message fields
    result: ::std::option::Option<OperationResult>,
    message: ::std::option::Option<::protobuf::chars::Chars>,
    first_event_number: ::std::option::Option<i64>,
    last_event_number: ::std::option::Option<i64>,
    prepare_position: ::std::option::Option<i64>,
    commit_position: ::std::option::Option<i64>,
    current_version: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteEventsCompleted {}

impl WriteEventsCompleted {
    pub fn new() -> WriteEventsCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteEventsCompleted {
        static mut instance: ::protobuf::lazy::Lazy<WriteEventsCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteEventsCompleted,
        };
        unsafe {
            instance.get(WriteEventsCompleted::new)
        }
    }

    // required .EventStore.Client.Messages.OperationResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: OperationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> OperationResult {
        self.result.unwrap_or(OperationResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<OperationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<OperationResult> {
        &mut self.result
    }

    // optional string message = 2;

    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None;
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::protobuf::chars::Chars) {
        self.message = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::protobuf::chars::Chars {
        self.message.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.message
    }

    // required int64 first_event_number = 3;

    pub fn clear_first_event_number(&mut self) {
        self.first_event_number = ::std::option::Option::None;
    }

    pub fn has_first_event_number(&self) -> bool {
        self.first_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_event_number(&mut self, v: i64) {
        self.first_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_first_event_number(&self) -> i64 {
        self.first_event_number.unwrap_or(0)
    }

    fn get_first_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.first_event_number
    }

    fn mut_first_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.first_event_number
    }

    // required int64 last_event_number = 4;

    pub fn clear_last_event_number(&mut self) {
        self.last_event_number = ::std::option::Option::None;
    }

    pub fn has_last_event_number(&self) -> bool {
        self.last_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_event_number(&mut self, v: i64) {
        self.last_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_last_event_number(&self) -> i64 {
        self.last_event_number.unwrap_or(0)
    }

    fn get_last_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_event_number
    }

    fn mut_last_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_event_number
    }

    // optional int64 prepare_position = 5;

    pub fn clear_prepare_position(&mut self) {
        self.prepare_position = ::std::option::Option::None;
    }

    pub fn has_prepare_position(&self) -> bool {
        self.prepare_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prepare_position(&mut self, v: i64) {
        self.prepare_position = ::std::option::Option::Some(v);
    }

    pub fn get_prepare_position(&self) -> i64 {
        self.prepare_position.unwrap_or(0)
    }

    fn get_prepare_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.prepare_position
    }

    fn mut_prepare_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.prepare_position
    }

    // optional int64 commit_position = 6;

    pub fn clear_commit_position(&mut self) {
        self.commit_position = ::std::option::Option::None;
    }

    pub fn has_commit_position(&self) -> bool {
        self.commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_position(&mut self, v: i64) {
        self.commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_commit_position(&self) -> i64 {
        self.commit_position.unwrap_or(0)
    }

    fn get_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.commit_position
    }

    fn mut_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.commit_position
    }

    // optional int64 current_version = 7;

    pub fn clear_current_version(&mut self) {
        self.current_version = ::std::option::Option::None;
    }

    pub fn has_current_version(&self) -> bool {
        self.current_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_version(&mut self, v: i64) {
        self.current_version = ::std::option::Option::Some(v);
    }

    pub fn get_current_version(&self) -> i64 {
        self.current_version.unwrap_or(0)
    }

    fn get_current_version_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.current_version
    }

    fn mut_current_version_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.current_version
    }
}

impl ::protobuf::Message for WriteEventsCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        if self.first_event_number.is_none() {
            return false;
        }
        if self.last_event_number.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.first_event_number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_event_number = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.prepare_position = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_position = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.current_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.first_event_number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.last_event_number {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prepare_position {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.commit_position {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.current_version {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.first_event_number {
            os.write_int64(3, v)?;
        }
        if let Some(v) = self.last_event_number {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.prepare_position {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.commit_position {
            os.write_int64(6, v)?;
        }
        if let Some(v) = self.current_version {
            os.write_int64(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteEventsCompleted {
    fn new() -> WriteEventsCompleted {
        WriteEventsCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteEventsCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperationResult>>(
                    "result",
                    WriteEventsCompleted::get_result_for_reflect,
                    WriteEventsCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "message",
                    WriteEventsCompleted::get_message_for_reflect,
                    WriteEventsCompleted::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "first_event_number",
                    WriteEventsCompleted::get_first_event_number_for_reflect,
                    WriteEventsCompleted::mut_first_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_event_number",
                    WriteEventsCompleted::get_last_event_number_for_reflect,
                    WriteEventsCompleted::mut_last_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "prepare_position",
                    WriteEventsCompleted::get_prepare_position_for_reflect,
                    WriteEventsCompleted::mut_prepare_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_position",
                    WriteEventsCompleted::get_commit_position_for_reflect,
                    WriteEventsCompleted::mut_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "current_version",
                    WriteEventsCompleted::get_current_version_for_reflect,
                    WriteEventsCompleted::mut_current_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteEventsCompleted>(
                    "WriteEventsCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteEventsCompleted {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_message();
        self.clear_first_event_number();
        self.clear_last_event_number();
        self.clear_prepare_position();
        self.clear_commit_position();
        self.clear_current_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteEventsCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteEventsCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteStream {
    // message fields
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    expected_version: ::std::option::Option<i64>,
    require_master: ::std::option::Option<bool>,
    hard_delete: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteStream {}

impl DeleteStream {
    pub fn new() -> DeleteStream {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteStream {
        static mut instance: ::protobuf::lazy::Lazy<DeleteStream> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteStream,
        };
        unsafe {
            instance.get(DeleteStream::new)
        }
    }

    // required string event_stream_id = 1;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required int64 expected_version = 2;

    pub fn clear_expected_version(&mut self) {
        self.expected_version = ::std::option::Option::None;
    }

    pub fn has_expected_version(&self) -> bool {
        self.expected_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expected_version(&mut self, v: i64) {
        self.expected_version = ::std::option::Option::Some(v);
    }

    pub fn get_expected_version(&self) -> i64 {
        self.expected_version.unwrap_or(0)
    }

    fn get_expected_version_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.expected_version
    }

    fn mut_expected_version_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.expected_version
    }

    // required bool require_master = 3;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }

    // optional bool hard_delete = 4;

    pub fn clear_hard_delete(&mut self) {
        self.hard_delete = ::std::option::Option::None;
    }

    pub fn has_hard_delete(&self) -> bool {
        self.hard_delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hard_delete(&mut self, v: bool) {
        self.hard_delete = ::std::option::Option::Some(v);
    }

    pub fn get_hard_delete(&self) -> bool {
        self.hard_delete.unwrap_or(false)
    }

    fn get_hard_delete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hard_delete
    }

    fn mut_hard_delete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hard_delete
    }
}

impl ::protobuf::Message for DeleteStream {
    fn is_initialized(&self) -> bool {
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.expected_version.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expected_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hard_delete = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.expected_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        if let Some(v) = self.hard_delete {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.expected_version {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.require_master {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.hard_delete {
            os.write_bool(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteStream {
    fn new() -> DeleteStream {
        DeleteStream::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteStream>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    DeleteStream::get_event_stream_id_for_reflect,
                    DeleteStream::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expected_version",
                    DeleteStream::get_expected_version_for_reflect,
                    DeleteStream::mut_expected_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    DeleteStream::get_require_master_for_reflect,
                    DeleteStream::mut_require_master_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hard_delete",
                    DeleteStream::get_hard_delete_for_reflect,
                    DeleteStream::mut_hard_delete_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteStream>(
                    "DeleteStream",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteStream {
    fn clear(&mut self) {
        self.clear_event_stream_id();
        self.clear_expected_version();
        self.clear_require_master();
        self.clear_hard_delete();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteStream {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteStreamCompleted {
    // message fields
    result: ::std::option::Option<OperationResult>,
    message: ::std::option::Option<::protobuf::chars::Chars>,
    prepare_position: ::std::option::Option<i64>,
    commit_position: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteStreamCompleted {}

impl DeleteStreamCompleted {
    pub fn new() -> DeleteStreamCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteStreamCompleted {
        static mut instance: ::protobuf::lazy::Lazy<DeleteStreamCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteStreamCompleted,
        };
        unsafe {
            instance.get(DeleteStreamCompleted::new)
        }
    }

    // required .EventStore.Client.Messages.OperationResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: OperationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> OperationResult {
        self.result.unwrap_or(OperationResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<OperationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<OperationResult> {
        &mut self.result
    }

    // optional string message = 2;

    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None;
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::protobuf::chars::Chars) {
        self.message = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::protobuf::chars::Chars {
        self.message.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.message
    }

    // optional int64 prepare_position = 3;

    pub fn clear_prepare_position(&mut self) {
        self.prepare_position = ::std::option::Option::None;
    }

    pub fn has_prepare_position(&self) -> bool {
        self.prepare_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prepare_position(&mut self, v: i64) {
        self.prepare_position = ::std::option::Option::Some(v);
    }

    pub fn get_prepare_position(&self) -> i64 {
        self.prepare_position.unwrap_or(0)
    }

    fn get_prepare_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.prepare_position
    }

    fn mut_prepare_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.prepare_position
    }

    // optional int64 commit_position = 4;

    pub fn clear_commit_position(&mut self) {
        self.commit_position = ::std::option::Option::None;
    }

    pub fn has_commit_position(&self) -> bool {
        self.commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_position(&mut self, v: i64) {
        self.commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_commit_position(&self) -> i64 {
        self.commit_position.unwrap_or(0)
    }

    fn get_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.commit_position
    }

    fn mut_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.commit_position
    }
}

impl ::protobuf::Message for DeleteStreamCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.prepare_position = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_position = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.prepare_position {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.commit_position {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.prepare_position {
            os.write_int64(3, v)?;
        }
        if let Some(v) = self.commit_position {
            os.write_int64(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteStreamCompleted {
    fn new() -> DeleteStreamCompleted {
        DeleteStreamCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteStreamCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperationResult>>(
                    "result",
                    DeleteStreamCompleted::get_result_for_reflect,
                    DeleteStreamCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "message",
                    DeleteStreamCompleted::get_message_for_reflect,
                    DeleteStreamCompleted::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "prepare_position",
                    DeleteStreamCompleted::get_prepare_position_for_reflect,
                    DeleteStreamCompleted::mut_prepare_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_position",
                    DeleteStreamCompleted::get_commit_position_for_reflect,
                    DeleteStreamCompleted::mut_commit_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteStreamCompleted>(
                    "DeleteStreamCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteStreamCompleted {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_message();
        self.clear_prepare_position();
        self.clear_commit_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteStreamCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteStreamCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionStart {
    // message fields
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    expected_version: ::std::option::Option<i64>,
    require_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionStart {}

impl TransactionStart {
    pub fn new() -> TransactionStart {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionStart {
        static mut instance: ::protobuf::lazy::Lazy<TransactionStart> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionStart,
        };
        unsafe {
            instance.get(TransactionStart::new)
        }
    }

    // required string event_stream_id = 1;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required int64 expected_version = 2;

    pub fn clear_expected_version(&mut self) {
        self.expected_version = ::std::option::Option::None;
    }

    pub fn has_expected_version(&self) -> bool {
        self.expected_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expected_version(&mut self, v: i64) {
        self.expected_version = ::std::option::Option::Some(v);
    }

    pub fn get_expected_version(&self) -> i64 {
        self.expected_version.unwrap_or(0)
    }

    fn get_expected_version_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.expected_version
    }

    fn mut_expected_version_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.expected_version
    }

    // required bool require_master = 3;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }
}

impl ::protobuf::Message for TransactionStart {
    fn is_initialized(&self) -> bool {
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.expected_version.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.expected_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.expected_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.expected_version {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.require_master {
            os.write_bool(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionStart {
    fn new() -> TransactionStart {
        TransactionStart::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionStart>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    TransactionStart::get_event_stream_id_for_reflect,
                    TransactionStart::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "expected_version",
                    TransactionStart::get_expected_version_for_reflect,
                    TransactionStart::mut_expected_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    TransactionStart::get_require_master_for_reflect,
                    TransactionStart::mut_require_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionStart>(
                    "TransactionStart",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionStart {
    fn clear(&mut self) {
        self.clear_event_stream_id();
        self.clear_expected_version();
        self.clear_require_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionStart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionStart {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionStartCompleted {
    // message fields
    transaction_id: ::std::option::Option<i64>,
    result: ::std::option::Option<OperationResult>,
    message: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionStartCompleted {}

impl TransactionStartCompleted {
    pub fn new() -> TransactionStartCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionStartCompleted {
        static mut instance: ::protobuf::lazy::Lazy<TransactionStartCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionStartCompleted,
        };
        unsafe {
            instance.get(TransactionStartCompleted::new)
        }
    }

    // required int64 transaction_id = 1;

    pub fn clear_transaction_id(&mut self) {
        self.transaction_id = ::std::option::Option::None;
    }

    pub fn has_transaction_id(&self) -> bool {
        self.transaction_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_id(&mut self, v: i64) {
        self.transaction_id = ::std::option::Option::Some(v);
    }

    pub fn get_transaction_id(&self) -> i64 {
        self.transaction_id.unwrap_or(0)
    }

    fn get_transaction_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.transaction_id
    }

    fn mut_transaction_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.transaction_id
    }

    // required .EventStore.Client.Messages.OperationResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: OperationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> OperationResult {
        self.result.unwrap_or(OperationResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<OperationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<OperationResult> {
        &mut self.result
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None;
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::protobuf::chars::Chars) {
        self.message = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::protobuf::chars::Chars {
        self.message.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.message
    }
}

impl ::protobuf::Message for TransactionStartCompleted {
    fn is_initialized(&self) -> bool {
        if self.transaction_id.is_none() {
            return false;
        }
        if self.result.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.transaction_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.transaction_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transaction_id {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.result {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionStartCompleted {
    fn new() -> TransactionStartCompleted {
        TransactionStartCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionStartCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "transaction_id",
                    TransactionStartCompleted::get_transaction_id_for_reflect,
                    TransactionStartCompleted::mut_transaction_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperationResult>>(
                    "result",
                    TransactionStartCompleted::get_result_for_reflect,
                    TransactionStartCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "message",
                    TransactionStartCompleted::get_message_for_reflect,
                    TransactionStartCompleted::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionStartCompleted>(
                    "TransactionStartCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionStartCompleted {
    fn clear(&mut self) {
        self.clear_transaction_id();
        self.clear_result();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionStartCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionStartCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionWrite {
    // message fields
    transaction_id: ::std::option::Option<i64>,
    events: ::protobuf::RepeatedField<NewEvent>,
    require_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionWrite {}

impl TransactionWrite {
    pub fn new() -> TransactionWrite {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionWrite {
        static mut instance: ::protobuf::lazy::Lazy<TransactionWrite> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionWrite,
        };
        unsafe {
            instance.get(TransactionWrite::new)
        }
    }

    // required int64 transaction_id = 1;

    pub fn clear_transaction_id(&mut self) {
        self.transaction_id = ::std::option::Option::None;
    }

    pub fn has_transaction_id(&self) -> bool {
        self.transaction_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_id(&mut self, v: i64) {
        self.transaction_id = ::std::option::Option::Some(v);
    }

    pub fn get_transaction_id(&self) -> i64 {
        self.transaction_id.unwrap_or(0)
    }

    fn get_transaction_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.transaction_id
    }

    fn mut_transaction_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.transaction_id
    }

    // repeated .EventStore.Client.Messages.NewEvent events = 2;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<NewEvent>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<NewEvent> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<NewEvent> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[NewEvent] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<NewEvent> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NewEvent> {
        &mut self.events
    }

    // required bool require_master = 3;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }
}

impl ::protobuf::Message for TransactionWrite {
    fn is_initialized(&self) -> bool {
        if self.transaction_id.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        for v in &self.events {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.transaction_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.transaction_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transaction_id {
            os.write_int64(1, v)?;
        }
        for v in &self.events {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.require_master {
            os.write_bool(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionWrite {
    fn new() -> TransactionWrite {
        TransactionWrite::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionWrite>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "transaction_id",
                    TransactionWrite::get_transaction_id_for_reflect,
                    TransactionWrite::mut_transaction_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NewEvent>>(
                    "events",
                    TransactionWrite::get_events_for_reflect,
                    TransactionWrite::mut_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    TransactionWrite::get_require_master_for_reflect,
                    TransactionWrite::mut_require_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionWrite>(
                    "TransactionWrite",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionWrite {
    fn clear(&mut self) {
        self.clear_transaction_id();
        self.clear_events();
        self.clear_require_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionWrite {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionWrite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionWriteCompleted {
    // message fields
    transaction_id: ::std::option::Option<i64>,
    result: ::std::option::Option<OperationResult>,
    message: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionWriteCompleted {}

impl TransactionWriteCompleted {
    pub fn new() -> TransactionWriteCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionWriteCompleted {
        static mut instance: ::protobuf::lazy::Lazy<TransactionWriteCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionWriteCompleted,
        };
        unsafe {
            instance.get(TransactionWriteCompleted::new)
        }
    }

    // required int64 transaction_id = 1;

    pub fn clear_transaction_id(&mut self) {
        self.transaction_id = ::std::option::Option::None;
    }

    pub fn has_transaction_id(&self) -> bool {
        self.transaction_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_id(&mut self, v: i64) {
        self.transaction_id = ::std::option::Option::Some(v);
    }

    pub fn get_transaction_id(&self) -> i64 {
        self.transaction_id.unwrap_or(0)
    }

    fn get_transaction_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.transaction_id
    }

    fn mut_transaction_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.transaction_id
    }

    // required .EventStore.Client.Messages.OperationResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: OperationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> OperationResult {
        self.result.unwrap_or(OperationResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<OperationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<OperationResult> {
        &mut self.result
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None;
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::protobuf::chars::Chars) {
        self.message = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::protobuf::chars::Chars {
        self.message.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.message
    }
}

impl ::protobuf::Message for TransactionWriteCompleted {
    fn is_initialized(&self) -> bool {
        if self.transaction_id.is_none() {
            return false;
        }
        if self.result.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.transaction_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.transaction_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transaction_id {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.result {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionWriteCompleted {
    fn new() -> TransactionWriteCompleted {
        TransactionWriteCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionWriteCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "transaction_id",
                    TransactionWriteCompleted::get_transaction_id_for_reflect,
                    TransactionWriteCompleted::mut_transaction_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperationResult>>(
                    "result",
                    TransactionWriteCompleted::get_result_for_reflect,
                    TransactionWriteCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "message",
                    TransactionWriteCompleted::get_message_for_reflect,
                    TransactionWriteCompleted::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionWriteCompleted>(
                    "TransactionWriteCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionWriteCompleted {
    fn clear(&mut self) {
        self.clear_transaction_id();
        self.clear_result();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionWriteCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionWriteCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionCommit {
    // message fields
    transaction_id: ::std::option::Option<i64>,
    require_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionCommit {}

impl TransactionCommit {
    pub fn new() -> TransactionCommit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionCommit {
        static mut instance: ::protobuf::lazy::Lazy<TransactionCommit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionCommit,
        };
        unsafe {
            instance.get(TransactionCommit::new)
        }
    }

    // required int64 transaction_id = 1;

    pub fn clear_transaction_id(&mut self) {
        self.transaction_id = ::std::option::Option::None;
    }

    pub fn has_transaction_id(&self) -> bool {
        self.transaction_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_id(&mut self, v: i64) {
        self.transaction_id = ::std::option::Option::Some(v);
    }

    pub fn get_transaction_id(&self) -> i64 {
        self.transaction_id.unwrap_or(0)
    }

    fn get_transaction_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.transaction_id
    }

    fn mut_transaction_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.transaction_id
    }

    // required bool require_master = 2;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }
}

impl ::protobuf::Message for TransactionCommit {
    fn is_initialized(&self) -> bool {
        if self.transaction_id.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.transaction_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.transaction_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transaction_id {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.require_master {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionCommit {
    fn new() -> TransactionCommit {
        TransactionCommit::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionCommit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "transaction_id",
                    TransactionCommit::get_transaction_id_for_reflect,
                    TransactionCommit::mut_transaction_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    TransactionCommit::get_require_master_for_reflect,
                    TransactionCommit::mut_require_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionCommit>(
                    "TransactionCommit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionCommit {
    fn clear(&mut self) {
        self.clear_transaction_id();
        self.clear_require_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionCommit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionCommit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransactionCommitCompleted {
    // message fields
    transaction_id: ::std::option::Option<i64>,
    result: ::std::option::Option<OperationResult>,
    message: ::std::option::Option<::protobuf::chars::Chars>,
    first_event_number: ::std::option::Option<i64>,
    last_event_number: ::std::option::Option<i64>,
    prepare_position: ::std::option::Option<i64>,
    commit_position: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransactionCommitCompleted {}

impl TransactionCommitCompleted {
    pub fn new() -> TransactionCommitCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionCommitCompleted {
        static mut instance: ::protobuf::lazy::Lazy<TransactionCommitCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionCommitCompleted,
        };
        unsafe {
            instance.get(TransactionCommitCompleted::new)
        }
    }

    // required int64 transaction_id = 1;

    pub fn clear_transaction_id(&mut self) {
        self.transaction_id = ::std::option::Option::None;
    }

    pub fn has_transaction_id(&self) -> bool {
        self.transaction_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_id(&mut self, v: i64) {
        self.transaction_id = ::std::option::Option::Some(v);
    }

    pub fn get_transaction_id(&self) -> i64 {
        self.transaction_id.unwrap_or(0)
    }

    fn get_transaction_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.transaction_id
    }

    fn mut_transaction_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.transaction_id
    }

    // required .EventStore.Client.Messages.OperationResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: OperationResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> OperationResult {
        self.result.unwrap_or(OperationResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<OperationResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<OperationResult> {
        &mut self.result
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None;
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::protobuf::chars::Chars) {
        self.message = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::protobuf::chars::Chars {
        self.message.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.message
    }

    // required int64 first_event_number = 4;

    pub fn clear_first_event_number(&mut self) {
        self.first_event_number = ::std::option::Option::None;
    }

    pub fn has_first_event_number(&self) -> bool {
        self.first_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_event_number(&mut self, v: i64) {
        self.first_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_first_event_number(&self) -> i64 {
        self.first_event_number.unwrap_or(0)
    }

    fn get_first_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.first_event_number
    }

    fn mut_first_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.first_event_number
    }

    // required int64 last_event_number = 5;

    pub fn clear_last_event_number(&mut self) {
        self.last_event_number = ::std::option::Option::None;
    }

    pub fn has_last_event_number(&self) -> bool {
        self.last_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_event_number(&mut self, v: i64) {
        self.last_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_last_event_number(&self) -> i64 {
        self.last_event_number.unwrap_or(0)
    }

    fn get_last_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_event_number
    }

    fn mut_last_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_event_number
    }

    // optional int64 prepare_position = 6;

    pub fn clear_prepare_position(&mut self) {
        self.prepare_position = ::std::option::Option::None;
    }

    pub fn has_prepare_position(&self) -> bool {
        self.prepare_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prepare_position(&mut self, v: i64) {
        self.prepare_position = ::std::option::Option::Some(v);
    }

    pub fn get_prepare_position(&self) -> i64 {
        self.prepare_position.unwrap_or(0)
    }

    fn get_prepare_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.prepare_position
    }

    fn mut_prepare_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.prepare_position
    }

    // optional int64 commit_position = 7;

    pub fn clear_commit_position(&mut self) {
        self.commit_position = ::std::option::Option::None;
    }

    pub fn has_commit_position(&self) -> bool {
        self.commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_position(&mut self, v: i64) {
        self.commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_commit_position(&self) -> i64 {
        self.commit_position.unwrap_or(0)
    }

    fn get_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.commit_position
    }

    fn mut_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.commit_position
    }
}

impl ::protobuf::Message for TransactionCommitCompleted {
    fn is_initialized(&self) -> bool {
        if self.transaction_id.is_none() {
            return false;
        }
        if self.result.is_none() {
            return false;
        }
        if self.first_event_number.is_none() {
            return false;
        }
        if self.last_event_number.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.transaction_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.message)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.first_event_number = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_event_number = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.prepare_position = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_position = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.transaction_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.first_event_number {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.last_event_number {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prepare_position {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.commit_position {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transaction_id {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.result {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(3, v)?;
        }
        if let Some(v) = self.first_event_number {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.last_event_number {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.prepare_position {
            os.write_int64(6, v)?;
        }
        if let Some(v) = self.commit_position {
            os.write_int64(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionCommitCompleted {
    fn new() -> TransactionCommitCompleted {
        TransactionCommitCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionCommitCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "transaction_id",
                    TransactionCommitCompleted::get_transaction_id_for_reflect,
                    TransactionCommitCompleted::mut_transaction_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperationResult>>(
                    "result",
                    TransactionCommitCompleted::get_result_for_reflect,
                    TransactionCommitCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "message",
                    TransactionCommitCompleted::get_message_for_reflect,
                    TransactionCommitCompleted::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "first_event_number",
                    TransactionCommitCompleted::get_first_event_number_for_reflect,
                    TransactionCommitCompleted::mut_first_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_event_number",
                    TransactionCommitCompleted::get_last_event_number_for_reflect,
                    TransactionCommitCompleted::mut_last_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "prepare_position",
                    TransactionCommitCompleted::get_prepare_position_for_reflect,
                    TransactionCommitCompleted::mut_prepare_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_position",
                    TransactionCommitCompleted::get_commit_position_for_reflect,
                    TransactionCommitCompleted::mut_commit_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionCommitCompleted>(
                    "TransactionCommitCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionCommitCompleted {
    fn clear(&mut self) {
        self.clear_transaction_id();
        self.clear_result();
        self.clear_message();
        self.clear_first_event_number();
        self.clear_last_event_number();
        self.clear_prepare_position();
        self.clear_commit_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionCommitCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionCommitCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadEvent {
    // message fields
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    event_number: ::std::option::Option<i64>,
    resolve_link_tos: ::std::option::Option<bool>,
    require_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadEvent {}

impl ReadEvent {
    pub fn new() -> ReadEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadEvent {
        static mut instance: ::protobuf::lazy::Lazy<ReadEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadEvent,
        };
        unsafe {
            instance.get(ReadEvent::new)
        }
    }

    // required string event_stream_id = 1;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required int64 event_number = 2;

    pub fn clear_event_number(&mut self) {
        self.event_number = ::std::option::Option::None;
    }

    pub fn has_event_number(&self) -> bool {
        self.event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_number(&mut self, v: i64) {
        self.event_number = ::std::option::Option::Some(v);
    }

    pub fn get_event_number(&self) -> i64 {
        self.event_number.unwrap_or(0)
    }

    fn get_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.event_number
    }

    fn mut_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.event_number
    }

    // required bool resolve_link_tos = 3;

    pub fn clear_resolve_link_tos(&mut self) {
        self.resolve_link_tos = ::std::option::Option::None;
    }

    pub fn has_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolve_link_tos(&mut self, v: bool) {
        self.resolve_link_tos = ::std::option::Option::Some(v);
    }

    pub fn get_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.unwrap_or(false)
    }

    fn get_resolve_link_tos_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.resolve_link_tos
    }

    fn mut_resolve_link_tos_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.resolve_link_tos
    }

    // required bool require_master = 4;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }
}

impl ::protobuf::Message for ReadEvent {
    fn is_initialized(&self) -> bool {
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.event_number.is_none() {
            return false;
        }
        if self.resolve_link_tos.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.event_number = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.resolve_link_tos = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.event_number {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.resolve_link_tos {
            my_size += 2;
        }
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.event_number {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.resolve_link_tos {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.require_master {
            os.write_bool(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadEvent {
    fn new() -> ReadEvent {
        ReadEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    ReadEvent::get_event_stream_id_for_reflect,
                    ReadEvent::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "event_number",
                    ReadEvent::get_event_number_for_reflect,
                    ReadEvent::mut_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "resolve_link_tos",
                    ReadEvent::get_resolve_link_tos_for_reflect,
                    ReadEvent::mut_resolve_link_tos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    ReadEvent::get_require_master_for_reflect,
                    ReadEvent::mut_require_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadEvent>(
                    "ReadEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadEvent {
    fn clear(&mut self) {
        self.clear_event_stream_id();
        self.clear_event_number();
        self.clear_resolve_link_tos();
        self.clear_require_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadEventCompleted {
    // message fields
    result: ::std::option::Option<ReadEventCompleted_ReadEventResult>,
    event: ::protobuf::SingularPtrField<ResolvedIndexedEvent>,
    error: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadEventCompleted {}

impl ReadEventCompleted {
    pub fn new() -> ReadEventCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadEventCompleted {
        static mut instance: ::protobuf::lazy::Lazy<ReadEventCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadEventCompleted,
        };
        unsafe {
            instance.get(ReadEventCompleted::new)
        }
    }

    // required .EventStore.Client.Messages.ReadEventCompleted.ReadEventResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ReadEventCompleted_ReadEventResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ReadEventCompleted_ReadEventResult {
        self.result.unwrap_or(ReadEventCompleted_ReadEventResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ReadEventCompleted_ReadEventResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ReadEventCompleted_ReadEventResult> {
        &mut self.result
    }

    // required .EventStore.Client.Messages.ResolvedIndexedEvent event = 2;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: ResolvedIndexedEvent) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut ResolvedIndexedEvent {
        if self.event.is_none() {
            self.event.set_default();
        }
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> ResolvedIndexedEvent {
        self.event.take().unwrap_or_else(|| ResolvedIndexedEvent::new())
    }

    pub fn get_event(&self) -> &ResolvedIndexedEvent {
        self.event.as_ref().unwrap_or_else(|| ResolvedIndexedEvent::default_instance())
    }

    fn get_event_for_reflect(&self) -> &::protobuf::SingularPtrField<ResolvedIndexedEvent> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResolvedIndexedEvent> {
        &mut self.event
    }

    // optional string error = 3;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::chars::Chars) {
        self.error = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::chars::Chars {
        self.error.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.error
    }
}

impl ::protobuf::Message for ReadEventCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        if self.event.is_none() {
            return false;
        }
        for v in &self.event {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.event.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadEventCompleted {
    fn new() -> ReadEventCompleted {
        ReadEventCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadEventCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReadEventCompleted_ReadEventResult>>(
                    "result",
                    ReadEventCompleted::get_result_for_reflect,
                    ReadEventCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResolvedIndexedEvent>>(
                    "event",
                    ReadEventCompleted::get_event_for_reflect,
                    ReadEventCompleted::mut_event_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "error",
                    ReadEventCompleted::get_error_for_reflect,
                    ReadEventCompleted::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadEventCompleted>(
                    "ReadEventCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadEventCompleted {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_event();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadEventCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadEventCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReadEventCompleted_ReadEventResult {
    Success = 0,
    NotFound = 1,
    NoStream = 2,
    StreamDeleted = 3,
    Error = 4,
    AccessDenied = 5,
}

impl ::protobuf::ProtobufEnum for ReadEventCompleted_ReadEventResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReadEventCompleted_ReadEventResult> {
        match value {
            0 => ::std::option::Option::Some(ReadEventCompleted_ReadEventResult::Success),
            1 => ::std::option::Option::Some(ReadEventCompleted_ReadEventResult::NotFound),
            2 => ::std::option::Option::Some(ReadEventCompleted_ReadEventResult::NoStream),
            3 => ::std::option::Option::Some(ReadEventCompleted_ReadEventResult::StreamDeleted),
            4 => ::std::option::Option::Some(ReadEventCompleted_ReadEventResult::Error),
            5 => ::std::option::Option::Some(ReadEventCompleted_ReadEventResult::AccessDenied),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReadEventCompleted_ReadEventResult] = &[
            ReadEventCompleted_ReadEventResult::Success,
            ReadEventCompleted_ReadEventResult::NotFound,
            ReadEventCompleted_ReadEventResult::NoStream,
            ReadEventCompleted_ReadEventResult::StreamDeleted,
            ReadEventCompleted_ReadEventResult::Error,
            ReadEventCompleted_ReadEventResult::AccessDenied,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReadEventCompleted_ReadEventResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReadEventCompleted_ReadEventResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReadEventCompleted_ReadEventResult {
}

impl ::protobuf::reflect::ProtobufValue for ReadEventCompleted_ReadEventResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadStreamEvents {
    // message fields
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    from_event_number: ::std::option::Option<i64>,
    max_count: ::std::option::Option<i32>,
    resolve_link_tos: ::std::option::Option<bool>,
    require_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadStreamEvents {}

impl ReadStreamEvents {
    pub fn new() -> ReadStreamEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadStreamEvents {
        static mut instance: ::protobuf::lazy::Lazy<ReadStreamEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadStreamEvents,
        };
        unsafe {
            instance.get(ReadStreamEvents::new)
        }
    }

    // required string event_stream_id = 1;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required int64 from_event_number = 2;

    pub fn clear_from_event_number(&mut self) {
        self.from_event_number = ::std::option::Option::None;
    }

    pub fn has_from_event_number(&self) -> bool {
        self.from_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_event_number(&mut self, v: i64) {
        self.from_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_from_event_number(&self) -> i64 {
        self.from_event_number.unwrap_or(0)
    }

    fn get_from_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.from_event_number
    }

    fn mut_from_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.from_event_number
    }

    // required int32 max_count = 3;

    pub fn clear_max_count(&mut self) {
        self.max_count = ::std::option::Option::None;
    }

    pub fn has_max_count(&self) -> bool {
        self.max_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_count(&mut self, v: i32) {
        self.max_count = ::std::option::Option::Some(v);
    }

    pub fn get_max_count(&self) -> i32 {
        self.max_count.unwrap_or(0)
    }

    fn get_max_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_count
    }

    fn mut_max_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_count
    }

    // required bool resolve_link_tos = 4;

    pub fn clear_resolve_link_tos(&mut self) {
        self.resolve_link_tos = ::std::option::Option::None;
    }

    pub fn has_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolve_link_tos(&mut self, v: bool) {
        self.resolve_link_tos = ::std::option::Option::Some(v);
    }

    pub fn get_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.unwrap_or(false)
    }

    fn get_resolve_link_tos_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.resolve_link_tos
    }

    fn mut_resolve_link_tos_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.resolve_link_tos
    }

    // required bool require_master = 5;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }
}

impl ::protobuf::Message for ReadStreamEvents {
    fn is_initialized(&self) -> bool {
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.from_event_number.is_none() {
            return false;
        }
        if self.max_count.is_none() {
            return false;
        }
        if self.resolve_link_tos.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.from_event_number = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.resolve_link_tos = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.from_event_number {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max_count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.resolve_link_tos {
            my_size += 2;
        }
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.from_event_number {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.max_count {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.resolve_link_tos {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.require_master {
            os.write_bool(5, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadStreamEvents {
    fn new() -> ReadStreamEvents {
        ReadStreamEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadStreamEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    ReadStreamEvents::get_event_stream_id_for_reflect,
                    ReadStreamEvents::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "from_event_number",
                    ReadStreamEvents::get_from_event_number_for_reflect,
                    ReadStreamEvents::mut_from_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_count",
                    ReadStreamEvents::get_max_count_for_reflect,
                    ReadStreamEvents::mut_max_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "resolve_link_tos",
                    ReadStreamEvents::get_resolve_link_tos_for_reflect,
                    ReadStreamEvents::mut_resolve_link_tos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    ReadStreamEvents::get_require_master_for_reflect,
                    ReadStreamEvents::mut_require_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadStreamEvents>(
                    "ReadStreamEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadStreamEvents {
    fn clear(&mut self) {
        self.clear_event_stream_id();
        self.clear_from_event_number();
        self.clear_max_count();
        self.clear_resolve_link_tos();
        self.clear_require_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadStreamEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadStreamEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadStreamEventsCompleted {
    // message fields
    events: ::protobuf::RepeatedField<ResolvedIndexedEvent>,
    result: ::std::option::Option<ReadStreamEventsCompleted_ReadStreamResult>,
    next_event_number: ::std::option::Option<i64>,
    last_event_number: ::std::option::Option<i64>,
    is_end_of_stream: ::std::option::Option<bool>,
    last_commit_position: ::std::option::Option<i64>,
    error: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadStreamEventsCompleted {}

impl ReadStreamEventsCompleted {
    pub fn new() -> ReadStreamEventsCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadStreamEventsCompleted {
        static mut instance: ::protobuf::lazy::Lazy<ReadStreamEventsCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadStreamEventsCompleted,
        };
        unsafe {
            instance.get(ReadStreamEventsCompleted::new)
        }
    }

    // repeated .EventStore.Client.Messages.ResolvedIndexedEvent events = 1;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<ResolvedIndexedEvent>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<ResolvedIndexedEvent> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<ResolvedIndexedEvent> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[ResolvedIndexedEvent] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<ResolvedIndexedEvent> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResolvedIndexedEvent> {
        &mut self.events
    }

    // required .EventStore.Client.Messages.ReadStreamEventsCompleted.ReadStreamResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ReadStreamEventsCompleted_ReadStreamResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ReadStreamEventsCompleted_ReadStreamResult {
        self.result.unwrap_or(ReadStreamEventsCompleted_ReadStreamResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ReadStreamEventsCompleted_ReadStreamResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ReadStreamEventsCompleted_ReadStreamResult> {
        &mut self.result
    }

    // required int64 next_event_number = 3;

    pub fn clear_next_event_number(&mut self) {
        self.next_event_number = ::std::option::Option::None;
    }

    pub fn has_next_event_number(&self) -> bool {
        self.next_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_event_number(&mut self, v: i64) {
        self.next_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_next_event_number(&self) -> i64 {
        self.next_event_number.unwrap_or(0)
    }

    fn get_next_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.next_event_number
    }

    fn mut_next_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.next_event_number
    }

    // required int64 last_event_number = 4;

    pub fn clear_last_event_number(&mut self) {
        self.last_event_number = ::std::option::Option::None;
    }

    pub fn has_last_event_number(&self) -> bool {
        self.last_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_event_number(&mut self, v: i64) {
        self.last_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_last_event_number(&self) -> i64 {
        self.last_event_number.unwrap_or(0)
    }

    fn get_last_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_event_number
    }

    fn mut_last_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_event_number
    }

    // required bool is_end_of_stream = 5;

    pub fn clear_is_end_of_stream(&mut self) {
        self.is_end_of_stream = ::std::option::Option::None;
    }

    pub fn has_is_end_of_stream(&self) -> bool {
        self.is_end_of_stream.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_end_of_stream(&mut self, v: bool) {
        self.is_end_of_stream = ::std::option::Option::Some(v);
    }

    pub fn get_is_end_of_stream(&self) -> bool {
        self.is_end_of_stream.unwrap_or(false)
    }

    fn get_is_end_of_stream_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_end_of_stream
    }

    fn mut_is_end_of_stream_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_end_of_stream
    }

    // required int64 last_commit_position = 6;

    pub fn clear_last_commit_position(&mut self) {
        self.last_commit_position = ::std::option::Option::None;
    }

    pub fn has_last_commit_position(&self) -> bool {
        self.last_commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_commit_position(&mut self, v: i64) {
        self.last_commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_last_commit_position(&self) -> i64 {
        self.last_commit_position.unwrap_or(0)
    }

    fn get_last_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_commit_position
    }

    fn mut_last_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_commit_position
    }

    // optional string error = 7;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::chars::Chars) {
        self.error = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::chars::Chars {
        self.error.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.error
    }
}

impl ::protobuf::Message for ReadStreamEventsCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        if self.next_event_number.is_none() {
            return false;
        }
        if self.last_event_number.is_none() {
            return false;
        }
        if self.is_end_of_stream.is_none() {
            return false;
        }
        if self.last_commit_position.is_none() {
            return false;
        }
        for v in &self.events {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
                },
                2 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 2, &mut self.unknown_fields)?
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.next_event_number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_event_number = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_end_of_stream = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_commit_position = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.next_event_number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.last_event_number {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_end_of_stream {
            my_size += 2;
        }
        if let Some(v) = self.last_commit_position {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.events {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.result {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.next_event_number {
            os.write_int64(3, v)?;
        }
        if let Some(v) = self.last_event_number {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.is_end_of_stream {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.last_commit_position {
            os.write_int64(6, v)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadStreamEventsCompleted {
    fn new() -> ReadStreamEventsCompleted {
        ReadStreamEventsCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadStreamEventsCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResolvedIndexedEvent>>(
                    "events",
                    ReadStreamEventsCompleted::get_events_for_reflect,
                    ReadStreamEventsCompleted::mut_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReadStreamEventsCompleted_ReadStreamResult>>(
                    "result",
                    ReadStreamEventsCompleted::get_result_for_reflect,
                    ReadStreamEventsCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "next_event_number",
                    ReadStreamEventsCompleted::get_next_event_number_for_reflect,
                    ReadStreamEventsCompleted::mut_next_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_event_number",
                    ReadStreamEventsCompleted::get_last_event_number_for_reflect,
                    ReadStreamEventsCompleted::mut_last_event_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_end_of_stream",
                    ReadStreamEventsCompleted::get_is_end_of_stream_for_reflect,
                    ReadStreamEventsCompleted::mut_is_end_of_stream_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_commit_position",
                    ReadStreamEventsCompleted::get_last_commit_position_for_reflect,
                    ReadStreamEventsCompleted::mut_last_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "error",
                    ReadStreamEventsCompleted::get_error_for_reflect,
                    ReadStreamEventsCompleted::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadStreamEventsCompleted>(
                    "ReadStreamEventsCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadStreamEventsCompleted {
    fn clear(&mut self) {
        self.clear_events();
        self.clear_result();
        self.clear_next_event_number();
        self.clear_last_event_number();
        self.clear_is_end_of_stream();
        self.clear_last_commit_position();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadStreamEventsCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadStreamEventsCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReadStreamEventsCompleted_ReadStreamResult {
    Success = 0,
    NoStream = 1,
    StreamDeleted = 2,
    NotModified = 3,
    Error = 4,
    AccessDenied = 5,
}

impl ::protobuf::ProtobufEnum for ReadStreamEventsCompleted_ReadStreamResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReadStreamEventsCompleted_ReadStreamResult> {
        match value {
            0 => ::std::option::Option::Some(ReadStreamEventsCompleted_ReadStreamResult::Success),
            1 => ::std::option::Option::Some(ReadStreamEventsCompleted_ReadStreamResult::NoStream),
            2 => ::std::option::Option::Some(ReadStreamEventsCompleted_ReadStreamResult::StreamDeleted),
            3 => ::std::option::Option::Some(ReadStreamEventsCompleted_ReadStreamResult::NotModified),
            4 => ::std::option::Option::Some(ReadStreamEventsCompleted_ReadStreamResult::Error),
            5 => ::std::option::Option::Some(ReadStreamEventsCompleted_ReadStreamResult::AccessDenied),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReadStreamEventsCompleted_ReadStreamResult] = &[
            ReadStreamEventsCompleted_ReadStreamResult::Success,
            ReadStreamEventsCompleted_ReadStreamResult::NoStream,
            ReadStreamEventsCompleted_ReadStreamResult::StreamDeleted,
            ReadStreamEventsCompleted_ReadStreamResult::NotModified,
            ReadStreamEventsCompleted_ReadStreamResult::Error,
            ReadStreamEventsCompleted_ReadStreamResult::AccessDenied,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReadStreamEventsCompleted_ReadStreamResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReadStreamEventsCompleted_ReadStreamResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReadStreamEventsCompleted_ReadStreamResult {
}

impl ::protobuf::reflect::ProtobufValue for ReadStreamEventsCompleted_ReadStreamResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadAllEvents {
    // message fields
    commit_position: ::std::option::Option<i64>,
    prepare_position: ::std::option::Option<i64>,
    max_count: ::std::option::Option<i32>,
    resolve_link_tos: ::std::option::Option<bool>,
    require_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadAllEvents {}

impl ReadAllEvents {
    pub fn new() -> ReadAllEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadAllEvents {
        static mut instance: ::protobuf::lazy::Lazy<ReadAllEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadAllEvents,
        };
        unsafe {
            instance.get(ReadAllEvents::new)
        }
    }

    // required int64 commit_position = 1;

    pub fn clear_commit_position(&mut self) {
        self.commit_position = ::std::option::Option::None;
    }

    pub fn has_commit_position(&self) -> bool {
        self.commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_position(&mut self, v: i64) {
        self.commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_commit_position(&self) -> i64 {
        self.commit_position.unwrap_or(0)
    }

    fn get_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.commit_position
    }

    fn mut_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.commit_position
    }

    // required int64 prepare_position = 2;

    pub fn clear_prepare_position(&mut self) {
        self.prepare_position = ::std::option::Option::None;
    }

    pub fn has_prepare_position(&self) -> bool {
        self.prepare_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prepare_position(&mut self, v: i64) {
        self.prepare_position = ::std::option::Option::Some(v);
    }

    pub fn get_prepare_position(&self) -> i64 {
        self.prepare_position.unwrap_or(0)
    }

    fn get_prepare_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.prepare_position
    }

    fn mut_prepare_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.prepare_position
    }

    // required int32 max_count = 3;

    pub fn clear_max_count(&mut self) {
        self.max_count = ::std::option::Option::None;
    }

    pub fn has_max_count(&self) -> bool {
        self.max_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_count(&mut self, v: i32) {
        self.max_count = ::std::option::Option::Some(v);
    }

    pub fn get_max_count(&self) -> i32 {
        self.max_count.unwrap_or(0)
    }

    fn get_max_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_count
    }

    fn mut_max_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_count
    }

    // required bool resolve_link_tos = 4;

    pub fn clear_resolve_link_tos(&mut self) {
        self.resolve_link_tos = ::std::option::Option::None;
    }

    pub fn has_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolve_link_tos(&mut self, v: bool) {
        self.resolve_link_tos = ::std::option::Option::Some(v);
    }

    pub fn get_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.unwrap_or(false)
    }

    fn get_resolve_link_tos_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.resolve_link_tos
    }

    fn mut_resolve_link_tos_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.resolve_link_tos
    }

    // required bool require_master = 5;

    pub fn clear_require_master(&mut self) {
        self.require_master = ::std::option::Option::None;
    }

    pub fn has_require_master(&self) -> bool {
        self.require_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_master(&mut self, v: bool) {
        self.require_master = ::std::option::Option::Some(v);
    }

    pub fn get_require_master(&self) -> bool {
        self.require_master.unwrap_or(false)
    }

    fn get_require_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_master
    }

    fn mut_require_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_master
    }
}

impl ::protobuf::Message for ReadAllEvents {
    fn is_initialized(&self) -> bool {
        if self.commit_position.is_none() {
            return false;
        }
        if self.prepare_position.is_none() {
            return false;
        }
        if self.max_count.is_none() {
            return false;
        }
        if self.resolve_link_tos.is_none() {
            return false;
        }
        if self.require_master.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_position = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.prepare_position = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.resolve_link_tos = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.commit_position {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prepare_position {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max_count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.resolve_link_tos {
            my_size += 2;
        }
        if let Some(v) = self.require_master {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.commit_position {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.prepare_position {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.max_count {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.resolve_link_tos {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.require_master {
            os.write_bool(5, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadAllEvents {
    fn new() -> ReadAllEvents {
        ReadAllEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadAllEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_position",
                    ReadAllEvents::get_commit_position_for_reflect,
                    ReadAllEvents::mut_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "prepare_position",
                    ReadAllEvents::get_prepare_position_for_reflect,
                    ReadAllEvents::mut_prepare_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_count",
                    ReadAllEvents::get_max_count_for_reflect,
                    ReadAllEvents::mut_max_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "resolve_link_tos",
                    ReadAllEvents::get_resolve_link_tos_for_reflect,
                    ReadAllEvents::mut_resolve_link_tos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_master",
                    ReadAllEvents::get_require_master_for_reflect,
                    ReadAllEvents::mut_require_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadAllEvents>(
                    "ReadAllEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadAllEvents {
    fn clear(&mut self) {
        self.clear_commit_position();
        self.clear_prepare_position();
        self.clear_max_count();
        self.clear_resolve_link_tos();
        self.clear_require_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadAllEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadAllEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadAllEventsCompleted {
    // message fields
    commit_position: ::std::option::Option<i64>,
    prepare_position: ::std::option::Option<i64>,
    events: ::protobuf::RepeatedField<ResolvedEvent>,
    next_commit_position: ::std::option::Option<i64>,
    next_prepare_position: ::std::option::Option<i64>,
    result: ::std::option::Option<ReadAllEventsCompleted_ReadAllResult>,
    error: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadAllEventsCompleted {}

impl ReadAllEventsCompleted {
    pub fn new() -> ReadAllEventsCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadAllEventsCompleted {
        static mut instance: ::protobuf::lazy::Lazy<ReadAllEventsCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadAllEventsCompleted,
        };
        unsafe {
            instance.get(ReadAllEventsCompleted::new)
        }
    }

    // required int64 commit_position = 1;

    pub fn clear_commit_position(&mut self) {
        self.commit_position = ::std::option::Option::None;
    }

    pub fn has_commit_position(&self) -> bool {
        self.commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_position(&mut self, v: i64) {
        self.commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_commit_position(&self) -> i64 {
        self.commit_position.unwrap_or(0)
    }

    fn get_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.commit_position
    }

    fn mut_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.commit_position
    }

    // required int64 prepare_position = 2;

    pub fn clear_prepare_position(&mut self) {
        self.prepare_position = ::std::option::Option::None;
    }

    pub fn has_prepare_position(&self) -> bool {
        self.prepare_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prepare_position(&mut self, v: i64) {
        self.prepare_position = ::std::option::Option::Some(v);
    }

    pub fn get_prepare_position(&self) -> i64 {
        self.prepare_position.unwrap_or(0)
    }

    fn get_prepare_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.prepare_position
    }

    fn mut_prepare_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.prepare_position
    }

    // repeated .EventStore.Client.Messages.ResolvedEvent events = 3;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<ResolvedEvent>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<ResolvedEvent> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<ResolvedEvent> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[ResolvedEvent] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<ResolvedEvent> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResolvedEvent> {
        &mut self.events
    }

    // required int64 next_commit_position = 4;

    pub fn clear_next_commit_position(&mut self) {
        self.next_commit_position = ::std::option::Option::None;
    }

    pub fn has_next_commit_position(&self) -> bool {
        self.next_commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_commit_position(&mut self, v: i64) {
        self.next_commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_next_commit_position(&self) -> i64 {
        self.next_commit_position.unwrap_or(0)
    }

    fn get_next_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.next_commit_position
    }

    fn mut_next_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.next_commit_position
    }

    // required int64 next_prepare_position = 5;

    pub fn clear_next_prepare_position(&mut self) {
        self.next_prepare_position = ::std::option::Option::None;
    }

    pub fn has_next_prepare_position(&self) -> bool {
        self.next_prepare_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_prepare_position(&mut self, v: i64) {
        self.next_prepare_position = ::std::option::Option::Some(v);
    }

    pub fn get_next_prepare_position(&self) -> i64 {
        self.next_prepare_position.unwrap_or(0)
    }

    fn get_next_prepare_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.next_prepare_position
    }

    fn mut_next_prepare_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.next_prepare_position
    }

    // optional .EventStore.Client.Messages.ReadAllEventsCompleted.ReadAllResult result = 6;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ReadAllEventsCompleted_ReadAllResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ReadAllEventsCompleted_ReadAllResult {
        self.result.unwrap_or(ReadAllEventsCompleted_ReadAllResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ReadAllEventsCompleted_ReadAllResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ReadAllEventsCompleted_ReadAllResult> {
        &mut self.result
    }

    // optional string error = 7;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::chars::Chars) {
        self.error = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::chars::Chars {
        self.error.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.error
    }
}

impl ::protobuf::Message for ReadAllEventsCompleted {
    fn is_initialized(&self) -> bool {
        if self.commit_position.is_none() {
            return false;
        }
        if self.prepare_position.is_none() {
            return false;
        }
        if self.next_commit_position.is_none() {
            return false;
        }
        if self.next_prepare_position.is_none() {
            return false;
        }
        for v in &self.events {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.commit_position = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.prepare_position = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.next_commit_position = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.next_prepare_position = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 6, &mut self.unknown_fields)?
                },
                7 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.commit_position {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prepare_position {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.next_commit_position {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.next_prepare_position {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(6, v);
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.commit_position {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.prepare_position {
            os.write_int64(2, v)?;
        }
        for v in &self.events {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.next_commit_position {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.next_prepare_position {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.result {
            os.write_enum(6, v.value())?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadAllEventsCompleted {
    fn new() -> ReadAllEventsCompleted {
        ReadAllEventsCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadAllEventsCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "commit_position",
                    ReadAllEventsCompleted::get_commit_position_for_reflect,
                    ReadAllEventsCompleted::mut_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "prepare_position",
                    ReadAllEventsCompleted::get_prepare_position_for_reflect,
                    ReadAllEventsCompleted::mut_prepare_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResolvedEvent>>(
                    "events",
                    ReadAllEventsCompleted::get_events_for_reflect,
                    ReadAllEventsCompleted::mut_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "next_commit_position",
                    ReadAllEventsCompleted::get_next_commit_position_for_reflect,
                    ReadAllEventsCompleted::mut_next_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "next_prepare_position",
                    ReadAllEventsCompleted::get_next_prepare_position_for_reflect,
                    ReadAllEventsCompleted::mut_next_prepare_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReadAllEventsCompleted_ReadAllResult>>(
                    "result",
                    ReadAllEventsCompleted::get_result_for_reflect,
                    ReadAllEventsCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "error",
                    ReadAllEventsCompleted::get_error_for_reflect,
                    ReadAllEventsCompleted::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadAllEventsCompleted>(
                    "ReadAllEventsCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadAllEventsCompleted {
    fn clear(&mut self) {
        self.clear_commit_position();
        self.clear_prepare_position();
        self.clear_events();
        self.clear_next_commit_position();
        self.clear_next_prepare_position();
        self.clear_result();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadAllEventsCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadAllEventsCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReadAllEventsCompleted_ReadAllResult {
    Success = 0,
    NotModified = 1,
    Error = 2,
    AccessDenied = 3,
}

impl ::protobuf::ProtobufEnum for ReadAllEventsCompleted_ReadAllResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReadAllEventsCompleted_ReadAllResult> {
        match value {
            0 => ::std::option::Option::Some(ReadAllEventsCompleted_ReadAllResult::Success),
            1 => ::std::option::Option::Some(ReadAllEventsCompleted_ReadAllResult::NotModified),
            2 => ::std::option::Option::Some(ReadAllEventsCompleted_ReadAllResult::Error),
            3 => ::std::option::Option::Some(ReadAllEventsCompleted_ReadAllResult::AccessDenied),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReadAllEventsCompleted_ReadAllResult] = &[
            ReadAllEventsCompleted_ReadAllResult::Success,
            ReadAllEventsCompleted_ReadAllResult::NotModified,
            ReadAllEventsCompleted_ReadAllResult::Error,
            ReadAllEventsCompleted_ReadAllResult::AccessDenied,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReadAllEventsCompleted_ReadAllResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReadAllEventsCompleted_ReadAllResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReadAllEventsCompleted_ReadAllResult {
}

impl ::protobuf::reflect::ProtobufValue for ReadAllEventsCompleted_ReadAllResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreatePersistentSubscription {
    // message fields
    subscription_group_name: ::std::option::Option<::protobuf::chars::Chars>,
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    resolve_link_tos: ::std::option::Option<bool>,
    start_from: ::std::option::Option<i64>,
    message_timeout_milliseconds: ::std::option::Option<i32>,
    record_statistics: ::std::option::Option<bool>,
    live_buffer_size: ::std::option::Option<i32>,
    read_batch_size: ::std::option::Option<i32>,
    buffer_size: ::std::option::Option<i32>,
    max_retry_count: ::std::option::Option<i32>,
    prefer_round_robin: ::std::option::Option<bool>,
    checkpoint_after_time: ::std::option::Option<i32>,
    checkpoint_max_count: ::std::option::Option<i32>,
    checkpoint_min_count: ::std::option::Option<i32>,
    subscriber_max_count: ::std::option::Option<i32>,
    named_consumer_strategy: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreatePersistentSubscription {}

impl CreatePersistentSubscription {
    pub fn new() -> CreatePersistentSubscription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreatePersistentSubscription {
        static mut instance: ::protobuf::lazy::Lazy<CreatePersistentSubscription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreatePersistentSubscription,
        };
        unsafe {
            instance.get(CreatePersistentSubscription::new)
        }
    }

    // required string subscription_group_name = 1;

    pub fn clear_subscription_group_name(&mut self) {
        self.subscription_group_name = ::std::option::Option::None;
    }

    pub fn has_subscription_group_name(&self) -> bool {
        self.subscription_group_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscription_group_name(&mut self, v: ::protobuf::chars::Chars) {
        self.subscription_group_name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscription_group_name(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.subscription_group_name.is_none() {
            self.subscription_group_name = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.subscription_group_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscription_group_name(&mut self) -> ::protobuf::chars::Chars {
        self.subscription_group_name.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_subscription_group_name(&self) -> &str {
        match self.subscription_group_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_subscription_group_name_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.subscription_group_name
    }

    fn mut_subscription_group_name_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.subscription_group_name
    }

    // required string event_stream_id = 2;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required bool resolve_link_tos = 3;

    pub fn clear_resolve_link_tos(&mut self) {
        self.resolve_link_tos = ::std::option::Option::None;
    }

    pub fn has_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolve_link_tos(&mut self, v: bool) {
        self.resolve_link_tos = ::std::option::Option::Some(v);
    }

    pub fn get_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.unwrap_or(false)
    }

    fn get_resolve_link_tos_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.resolve_link_tos
    }

    fn mut_resolve_link_tos_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.resolve_link_tos
    }

    // required int64 start_from = 4;

    pub fn clear_start_from(&mut self) {
        self.start_from = ::std::option::Option::None;
    }

    pub fn has_start_from(&self) -> bool {
        self.start_from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_from(&mut self, v: i64) {
        self.start_from = ::std::option::Option::Some(v);
    }

    pub fn get_start_from(&self) -> i64 {
        self.start_from.unwrap_or(0)
    }

    fn get_start_from_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.start_from
    }

    fn mut_start_from_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.start_from
    }

    // required int32 message_timeout_milliseconds = 5;

    pub fn clear_message_timeout_milliseconds(&mut self) {
        self.message_timeout_milliseconds = ::std::option::Option::None;
    }

    pub fn has_message_timeout_milliseconds(&self) -> bool {
        self.message_timeout_milliseconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_timeout_milliseconds(&mut self, v: i32) {
        self.message_timeout_milliseconds = ::std::option::Option::Some(v);
    }

    pub fn get_message_timeout_milliseconds(&self) -> i32 {
        self.message_timeout_milliseconds.unwrap_or(0)
    }

    fn get_message_timeout_milliseconds_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.message_timeout_milliseconds
    }

    fn mut_message_timeout_milliseconds_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.message_timeout_milliseconds
    }

    // required bool record_statistics = 6;

    pub fn clear_record_statistics(&mut self) {
        self.record_statistics = ::std::option::Option::None;
    }

    pub fn has_record_statistics(&self) -> bool {
        self.record_statistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_record_statistics(&mut self, v: bool) {
        self.record_statistics = ::std::option::Option::Some(v);
    }

    pub fn get_record_statistics(&self) -> bool {
        self.record_statistics.unwrap_or(false)
    }

    fn get_record_statistics_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.record_statistics
    }

    fn mut_record_statistics_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.record_statistics
    }

    // required int32 live_buffer_size = 7;

    pub fn clear_live_buffer_size(&mut self) {
        self.live_buffer_size = ::std::option::Option::None;
    }

    pub fn has_live_buffer_size(&self) -> bool {
        self.live_buffer_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_live_buffer_size(&mut self, v: i32) {
        self.live_buffer_size = ::std::option::Option::Some(v);
    }

    pub fn get_live_buffer_size(&self) -> i32 {
        self.live_buffer_size.unwrap_or(0)
    }

    fn get_live_buffer_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.live_buffer_size
    }

    fn mut_live_buffer_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.live_buffer_size
    }

    // required int32 read_batch_size = 8;

    pub fn clear_read_batch_size(&mut self) {
        self.read_batch_size = ::std::option::Option::None;
    }

    pub fn has_read_batch_size(&self) -> bool {
        self.read_batch_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_read_batch_size(&mut self, v: i32) {
        self.read_batch_size = ::std::option::Option::Some(v);
    }

    pub fn get_read_batch_size(&self) -> i32 {
        self.read_batch_size.unwrap_or(0)
    }

    fn get_read_batch_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.read_batch_size
    }

    fn mut_read_batch_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.read_batch_size
    }

    // required int32 buffer_size = 9;

    pub fn clear_buffer_size(&mut self) {
        self.buffer_size = ::std::option::Option::None;
    }

    pub fn has_buffer_size(&self) -> bool {
        self.buffer_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer_size(&mut self, v: i32) {
        self.buffer_size = ::std::option::Option::Some(v);
    }

    pub fn get_buffer_size(&self) -> i32 {
        self.buffer_size.unwrap_or(0)
    }

    fn get_buffer_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.buffer_size
    }

    fn mut_buffer_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.buffer_size
    }

    // required int32 max_retry_count = 10;

    pub fn clear_max_retry_count(&mut self) {
        self.max_retry_count = ::std::option::Option::None;
    }

    pub fn has_max_retry_count(&self) -> bool {
        self.max_retry_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_retry_count(&mut self, v: i32) {
        self.max_retry_count = ::std::option::Option::Some(v);
    }

    pub fn get_max_retry_count(&self) -> i32 {
        self.max_retry_count.unwrap_or(0)
    }

    fn get_max_retry_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_retry_count
    }

    fn mut_max_retry_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_retry_count
    }

    // required bool prefer_round_robin = 11;

    pub fn clear_prefer_round_robin(&mut self) {
        self.prefer_round_robin = ::std::option::Option::None;
    }

    pub fn has_prefer_round_robin(&self) -> bool {
        self.prefer_round_robin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefer_round_robin(&mut self, v: bool) {
        self.prefer_round_robin = ::std::option::Option::Some(v);
    }

    pub fn get_prefer_round_robin(&self) -> bool {
        self.prefer_round_robin.unwrap_or(false)
    }

    fn get_prefer_round_robin_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.prefer_round_robin
    }

    fn mut_prefer_round_robin_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.prefer_round_robin
    }

    // required int32 checkpoint_after_time = 12;

    pub fn clear_checkpoint_after_time(&mut self) {
        self.checkpoint_after_time = ::std::option::Option::None;
    }

    pub fn has_checkpoint_after_time(&self) -> bool {
        self.checkpoint_after_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint_after_time(&mut self, v: i32) {
        self.checkpoint_after_time = ::std::option::Option::Some(v);
    }

    pub fn get_checkpoint_after_time(&self) -> i32 {
        self.checkpoint_after_time.unwrap_or(0)
    }

    fn get_checkpoint_after_time_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.checkpoint_after_time
    }

    fn mut_checkpoint_after_time_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.checkpoint_after_time
    }

    // required int32 checkpoint_max_count = 13;

    pub fn clear_checkpoint_max_count(&mut self) {
        self.checkpoint_max_count = ::std::option::Option::None;
    }

    pub fn has_checkpoint_max_count(&self) -> bool {
        self.checkpoint_max_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint_max_count(&mut self, v: i32) {
        self.checkpoint_max_count = ::std::option::Option::Some(v);
    }

    pub fn get_checkpoint_max_count(&self) -> i32 {
        self.checkpoint_max_count.unwrap_or(0)
    }

    fn get_checkpoint_max_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.checkpoint_max_count
    }

    fn mut_checkpoint_max_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.checkpoint_max_count
    }

    // required int32 checkpoint_min_count = 14;

    pub fn clear_checkpoint_min_count(&mut self) {
        self.checkpoint_min_count = ::std::option::Option::None;
    }

    pub fn has_checkpoint_min_count(&self) -> bool {
        self.checkpoint_min_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint_min_count(&mut self, v: i32) {
        self.checkpoint_min_count = ::std::option::Option::Some(v);
    }

    pub fn get_checkpoint_min_count(&self) -> i32 {
        self.checkpoint_min_count.unwrap_or(0)
    }

    fn get_checkpoint_min_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.checkpoint_min_count
    }

    fn mut_checkpoint_min_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.checkpoint_min_count
    }

    // required int32 subscriber_max_count = 15;

    pub fn clear_subscriber_max_count(&mut self) {
        self.subscriber_max_count = ::std::option::Option::None;
    }

    pub fn has_subscriber_max_count(&self) -> bool {
        self.subscriber_max_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscriber_max_count(&mut self, v: i32) {
        self.subscriber_max_count = ::std::option::Option::Some(v);
    }

    pub fn get_subscriber_max_count(&self) -> i32 {
        self.subscriber_max_count.unwrap_or(0)
    }

    fn get_subscriber_max_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.subscriber_max_count
    }

    fn mut_subscriber_max_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.subscriber_max_count
    }

    // optional string named_consumer_strategy = 16;

    pub fn clear_named_consumer_strategy(&mut self) {
        self.named_consumer_strategy = ::std::option::Option::None;
    }

    pub fn has_named_consumer_strategy(&self) -> bool {
        self.named_consumer_strategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_named_consumer_strategy(&mut self, v: ::protobuf::chars::Chars) {
        self.named_consumer_strategy = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_named_consumer_strategy(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.named_consumer_strategy.is_none() {
            self.named_consumer_strategy = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.named_consumer_strategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_named_consumer_strategy(&mut self) -> ::protobuf::chars::Chars {
        self.named_consumer_strategy.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_named_consumer_strategy(&self) -> &str {
        match self.named_consumer_strategy.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_named_consumer_strategy_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.named_consumer_strategy
    }

    fn mut_named_consumer_strategy_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.named_consumer_strategy
    }
}

impl ::protobuf::Message for CreatePersistentSubscription {
    fn is_initialized(&self) -> bool {
        if self.subscription_group_name.is_none() {
            return false;
        }
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.resolve_link_tos.is_none() {
            return false;
        }
        if self.start_from.is_none() {
            return false;
        }
        if self.message_timeout_milliseconds.is_none() {
            return false;
        }
        if self.record_statistics.is_none() {
            return false;
        }
        if self.live_buffer_size.is_none() {
            return false;
        }
        if self.read_batch_size.is_none() {
            return false;
        }
        if self.buffer_size.is_none() {
            return false;
        }
        if self.max_retry_count.is_none() {
            return false;
        }
        if self.prefer_round_robin.is_none() {
            return false;
        }
        if self.checkpoint_after_time.is_none() {
            return false;
        }
        if self.checkpoint_max_count.is_none() {
            return false;
        }
        if self.checkpoint_min_count.is_none() {
            return false;
        }
        if self.subscriber_max_count.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.subscription_group_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.resolve_link_tos = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.start_from = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.message_timeout_milliseconds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.record_statistics = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.live_buffer_size = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.read_batch_size = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.buffer_size = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_retry_count = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.prefer_round_robin = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.checkpoint_after_time = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.checkpoint_max_count = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.checkpoint_min_count = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.subscriber_max_count = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.named_consumer_strategy)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.subscription_group_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.resolve_link_tos {
            my_size += 2;
        }
        if let Some(v) = self.start_from {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.message_timeout_milliseconds {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.record_statistics {
            my_size += 2;
        }
        if let Some(v) = self.live_buffer_size {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.read_batch_size {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.buffer_size {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max_retry_count {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prefer_round_robin {
            my_size += 2;
        }
        if let Some(v) = self.checkpoint_after_time {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.checkpoint_max_count {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.checkpoint_min_count {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.subscriber_max_count {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.named_consumer_strategy.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.subscription_group_name.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.resolve_link_tos {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.start_from {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.message_timeout_milliseconds {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.record_statistics {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.live_buffer_size {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.read_batch_size {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.buffer_size {
            os.write_int32(9, v)?;
        }
        if let Some(v) = self.max_retry_count {
            os.write_int32(10, v)?;
        }
        if let Some(v) = self.prefer_round_robin {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.checkpoint_after_time {
            os.write_int32(12, v)?;
        }
        if let Some(v) = self.checkpoint_max_count {
            os.write_int32(13, v)?;
        }
        if let Some(v) = self.checkpoint_min_count {
            os.write_int32(14, v)?;
        }
        if let Some(v) = self.subscriber_max_count {
            os.write_int32(15, v)?;
        }
        if let Some(ref v) = self.named_consumer_strategy.as_ref() {
            os.write_string(16, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreatePersistentSubscription {
    fn new() -> CreatePersistentSubscription {
        CreatePersistentSubscription::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreatePersistentSubscription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "subscription_group_name",
                    CreatePersistentSubscription::get_subscription_group_name_for_reflect,
                    CreatePersistentSubscription::mut_subscription_group_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    CreatePersistentSubscription::get_event_stream_id_for_reflect,
                    CreatePersistentSubscription::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "resolve_link_tos",
                    CreatePersistentSubscription::get_resolve_link_tos_for_reflect,
                    CreatePersistentSubscription::mut_resolve_link_tos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start_from",
                    CreatePersistentSubscription::get_start_from_for_reflect,
                    CreatePersistentSubscription::mut_start_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "message_timeout_milliseconds",
                    CreatePersistentSubscription::get_message_timeout_milliseconds_for_reflect,
                    CreatePersistentSubscription::mut_message_timeout_milliseconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "record_statistics",
                    CreatePersistentSubscription::get_record_statistics_for_reflect,
                    CreatePersistentSubscription::mut_record_statistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "live_buffer_size",
                    CreatePersistentSubscription::get_live_buffer_size_for_reflect,
                    CreatePersistentSubscription::mut_live_buffer_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "read_batch_size",
                    CreatePersistentSubscription::get_read_batch_size_for_reflect,
                    CreatePersistentSubscription::mut_read_batch_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "buffer_size",
                    CreatePersistentSubscription::get_buffer_size_for_reflect,
                    CreatePersistentSubscription::mut_buffer_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_retry_count",
                    CreatePersistentSubscription::get_max_retry_count_for_reflect,
                    CreatePersistentSubscription::mut_max_retry_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prefer_round_robin",
                    CreatePersistentSubscription::get_prefer_round_robin_for_reflect,
                    CreatePersistentSubscription::mut_prefer_round_robin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "checkpoint_after_time",
                    CreatePersistentSubscription::get_checkpoint_after_time_for_reflect,
                    CreatePersistentSubscription::mut_checkpoint_after_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "checkpoint_max_count",
                    CreatePersistentSubscription::get_checkpoint_max_count_for_reflect,
                    CreatePersistentSubscription::mut_checkpoint_max_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "checkpoint_min_count",
                    CreatePersistentSubscription::get_checkpoint_min_count_for_reflect,
                    CreatePersistentSubscription::mut_checkpoint_min_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "subscriber_max_count",
                    CreatePersistentSubscription::get_subscriber_max_count_for_reflect,
                    CreatePersistentSubscription::mut_subscriber_max_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "named_consumer_strategy",
                    CreatePersistentSubscription::get_named_consumer_strategy_for_reflect,
                    CreatePersistentSubscription::mut_named_consumer_strategy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreatePersistentSubscription>(
                    "CreatePersistentSubscription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreatePersistentSubscription {
    fn clear(&mut self) {
        self.clear_subscription_group_name();
        self.clear_event_stream_id();
        self.clear_resolve_link_tos();
        self.clear_start_from();
        self.clear_message_timeout_milliseconds();
        self.clear_record_statistics();
        self.clear_live_buffer_size();
        self.clear_read_batch_size();
        self.clear_buffer_size();
        self.clear_max_retry_count();
        self.clear_prefer_round_robin();
        self.clear_checkpoint_after_time();
        self.clear_checkpoint_max_count();
        self.clear_checkpoint_min_count();
        self.clear_subscriber_max_count();
        self.clear_named_consumer_strategy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreatePersistentSubscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreatePersistentSubscription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeletePersistentSubscription {
    // message fields
    subscription_group_name: ::std::option::Option<::protobuf::chars::Chars>,
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeletePersistentSubscription {}

impl DeletePersistentSubscription {
    pub fn new() -> DeletePersistentSubscription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeletePersistentSubscription {
        static mut instance: ::protobuf::lazy::Lazy<DeletePersistentSubscription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeletePersistentSubscription,
        };
        unsafe {
            instance.get(DeletePersistentSubscription::new)
        }
    }

    // required string subscription_group_name = 1;

    pub fn clear_subscription_group_name(&mut self) {
        self.subscription_group_name = ::std::option::Option::None;
    }

    pub fn has_subscription_group_name(&self) -> bool {
        self.subscription_group_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscription_group_name(&mut self, v: ::protobuf::chars::Chars) {
        self.subscription_group_name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscription_group_name(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.subscription_group_name.is_none() {
            self.subscription_group_name = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.subscription_group_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscription_group_name(&mut self) -> ::protobuf::chars::Chars {
        self.subscription_group_name.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_subscription_group_name(&self) -> &str {
        match self.subscription_group_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_subscription_group_name_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.subscription_group_name
    }

    fn mut_subscription_group_name_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.subscription_group_name
    }

    // required string event_stream_id = 2;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }
}

impl ::protobuf::Message for DeletePersistentSubscription {
    fn is_initialized(&self) -> bool {
        if self.subscription_group_name.is_none() {
            return false;
        }
        if self.event_stream_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.subscription_group_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.subscription_group_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.subscription_group_name.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeletePersistentSubscription {
    fn new() -> DeletePersistentSubscription {
        DeletePersistentSubscription::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeletePersistentSubscription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "subscription_group_name",
                    DeletePersistentSubscription::get_subscription_group_name_for_reflect,
                    DeletePersistentSubscription::mut_subscription_group_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    DeletePersistentSubscription::get_event_stream_id_for_reflect,
                    DeletePersistentSubscription::mut_event_stream_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeletePersistentSubscription>(
                    "DeletePersistentSubscription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeletePersistentSubscription {
    fn clear(&mut self) {
        self.clear_subscription_group_name();
        self.clear_event_stream_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeletePersistentSubscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeletePersistentSubscription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdatePersistentSubscription {
    // message fields
    subscription_group_name: ::std::option::Option<::protobuf::chars::Chars>,
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    resolve_link_tos: ::std::option::Option<bool>,
    start_from: ::std::option::Option<i64>,
    message_timeout_milliseconds: ::std::option::Option<i32>,
    record_statistics: ::std::option::Option<bool>,
    live_buffer_size: ::std::option::Option<i32>,
    read_batch_size: ::std::option::Option<i32>,
    buffer_size: ::std::option::Option<i32>,
    max_retry_count: ::std::option::Option<i32>,
    prefer_round_robin: ::std::option::Option<bool>,
    checkpoint_after_time: ::std::option::Option<i32>,
    checkpoint_max_count: ::std::option::Option<i32>,
    checkpoint_min_count: ::std::option::Option<i32>,
    subscriber_max_count: ::std::option::Option<i32>,
    named_consumer_strategy: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdatePersistentSubscription {}

impl UpdatePersistentSubscription {
    pub fn new() -> UpdatePersistentSubscription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdatePersistentSubscription {
        static mut instance: ::protobuf::lazy::Lazy<UpdatePersistentSubscription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdatePersistentSubscription,
        };
        unsafe {
            instance.get(UpdatePersistentSubscription::new)
        }
    }

    // required string subscription_group_name = 1;

    pub fn clear_subscription_group_name(&mut self) {
        self.subscription_group_name = ::std::option::Option::None;
    }

    pub fn has_subscription_group_name(&self) -> bool {
        self.subscription_group_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscription_group_name(&mut self, v: ::protobuf::chars::Chars) {
        self.subscription_group_name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscription_group_name(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.subscription_group_name.is_none() {
            self.subscription_group_name = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.subscription_group_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscription_group_name(&mut self) -> ::protobuf::chars::Chars {
        self.subscription_group_name.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_subscription_group_name(&self) -> &str {
        match self.subscription_group_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_subscription_group_name_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.subscription_group_name
    }

    fn mut_subscription_group_name_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.subscription_group_name
    }

    // required string event_stream_id = 2;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required bool resolve_link_tos = 3;

    pub fn clear_resolve_link_tos(&mut self) {
        self.resolve_link_tos = ::std::option::Option::None;
    }

    pub fn has_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolve_link_tos(&mut self, v: bool) {
        self.resolve_link_tos = ::std::option::Option::Some(v);
    }

    pub fn get_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.unwrap_or(false)
    }

    fn get_resolve_link_tos_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.resolve_link_tos
    }

    fn mut_resolve_link_tos_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.resolve_link_tos
    }

    // required int64 start_from = 4;

    pub fn clear_start_from(&mut self) {
        self.start_from = ::std::option::Option::None;
    }

    pub fn has_start_from(&self) -> bool {
        self.start_from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_from(&mut self, v: i64) {
        self.start_from = ::std::option::Option::Some(v);
    }

    pub fn get_start_from(&self) -> i64 {
        self.start_from.unwrap_or(0)
    }

    fn get_start_from_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.start_from
    }

    fn mut_start_from_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.start_from
    }

    // required int32 message_timeout_milliseconds = 5;

    pub fn clear_message_timeout_milliseconds(&mut self) {
        self.message_timeout_milliseconds = ::std::option::Option::None;
    }

    pub fn has_message_timeout_milliseconds(&self) -> bool {
        self.message_timeout_milliseconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_timeout_milliseconds(&mut self, v: i32) {
        self.message_timeout_milliseconds = ::std::option::Option::Some(v);
    }

    pub fn get_message_timeout_milliseconds(&self) -> i32 {
        self.message_timeout_milliseconds.unwrap_or(0)
    }

    fn get_message_timeout_milliseconds_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.message_timeout_milliseconds
    }

    fn mut_message_timeout_milliseconds_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.message_timeout_milliseconds
    }

    // required bool record_statistics = 6;

    pub fn clear_record_statistics(&mut self) {
        self.record_statistics = ::std::option::Option::None;
    }

    pub fn has_record_statistics(&self) -> bool {
        self.record_statistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_record_statistics(&mut self, v: bool) {
        self.record_statistics = ::std::option::Option::Some(v);
    }

    pub fn get_record_statistics(&self) -> bool {
        self.record_statistics.unwrap_or(false)
    }

    fn get_record_statistics_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.record_statistics
    }

    fn mut_record_statistics_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.record_statistics
    }

    // required int32 live_buffer_size = 7;

    pub fn clear_live_buffer_size(&mut self) {
        self.live_buffer_size = ::std::option::Option::None;
    }

    pub fn has_live_buffer_size(&self) -> bool {
        self.live_buffer_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_live_buffer_size(&mut self, v: i32) {
        self.live_buffer_size = ::std::option::Option::Some(v);
    }

    pub fn get_live_buffer_size(&self) -> i32 {
        self.live_buffer_size.unwrap_or(0)
    }

    fn get_live_buffer_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.live_buffer_size
    }

    fn mut_live_buffer_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.live_buffer_size
    }

    // required int32 read_batch_size = 8;

    pub fn clear_read_batch_size(&mut self) {
        self.read_batch_size = ::std::option::Option::None;
    }

    pub fn has_read_batch_size(&self) -> bool {
        self.read_batch_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_read_batch_size(&mut self, v: i32) {
        self.read_batch_size = ::std::option::Option::Some(v);
    }

    pub fn get_read_batch_size(&self) -> i32 {
        self.read_batch_size.unwrap_or(0)
    }

    fn get_read_batch_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.read_batch_size
    }

    fn mut_read_batch_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.read_batch_size
    }

    // required int32 buffer_size = 9;

    pub fn clear_buffer_size(&mut self) {
        self.buffer_size = ::std::option::Option::None;
    }

    pub fn has_buffer_size(&self) -> bool {
        self.buffer_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer_size(&mut self, v: i32) {
        self.buffer_size = ::std::option::Option::Some(v);
    }

    pub fn get_buffer_size(&self) -> i32 {
        self.buffer_size.unwrap_or(0)
    }

    fn get_buffer_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.buffer_size
    }

    fn mut_buffer_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.buffer_size
    }

    // required int32 max_retry_count = 10;

    pub fn clear_max_retry_count(&mut self) {
        self.max_retry_count = ::std::option::Option::None;
    }

    pub fn has_max_retry_count(&self) -> bool {
        self.max_retry_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_retry_count(&mut self, v: i32) {
        self.max_retry_count = ::std::option::Option::Some(v);
    }

    pub fn get_max_retry_count(&self) -> i32 {
        self.max_retry_count.unwrap_or(0)
    }

    fn get_max_retry_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_retry_count
    }

    fn mut_max_retry_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_retry_count
    }

    // required bool prefer_round_robin = 11;

    pub fn clear_prefer_round_robin(&mut self) {
        self.prefer_round_robin = ::std::option::Option::None;
    }

    pub fn has_prefer_round_robin(&self) -> bool {
        self.prefer_round_robin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefer_round_robin(&mut self, v: bool) {
        self.prefer_round_robin = ::std::option::Option::Some(v);
    }

    pub fn get_prefer_round_robin(&self) -> bool {
        self.prefer_round_robin.unwrap_or(false)
    }

    fn get_prefer_round_robin_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.prefer_round_robin
    }

    fn mut_prefer_round_robin_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.prefer_round_robin
    }

    // required int32 checkpoint_after_time = 12;

    pub fn clear_checkpoint_after_time(&mut self) {
        self.checkpoint_after_time = ::std::option::Option::None;
    }

    pub fn has_checkpoint_after_time(&self) -> bool {
        self.checkpoint_after_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint_after_time(&mut self, v: i32) {
        self.checkpoint_after_time = ::std::option::Option::Some(v);
    }

    pub fn get_checkpoint_after_time(&self) -> i32 {
        self.checkpoint_after_time.unwrap_or(0)
    }

    fn get_checkpoint_after_time_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.checkpoint_after_time
    }

    fn mut_checkpoint_after_time_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.checkpoint_after_time
    }

    // required int32 checkpoint_max_count = 13;

    pub fn clear_checkpoint_max_count(&mut self) {
        self.checkpoint_max_count = ::std::option::Option::None;
    }

    pub fn has_checkpoint_max_count(&self) -> bool {
        self.checkpoint_max_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint_max_count(&mut self, v: i32) {
        self.checkpoint_max_count = ::std::option::Option::Some(v);
    }

    pub fn get_checkpoint_max_count(&self) -> i32 {
        self.checkpoint_max_count.unwrap_or(0)
    }

    fn get_checkpoint_max_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.checkpoint_max_count
    }

    fn mut_checkpoint_max_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.checkpoint_max_count
    }

    // required int32 checkpoint_min_count = 14;

    pub fn clear_checkpoint_min_count(&mut self) {
        self.checkpoint_min_count = ::std::option::Option::None;
    }

    pub fn has_checkpoint_min_count(&self) -> bool {
        self.checkpoint_min_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint_min_count(&mut self, v: i32) {
        self.checkpoint_min_count = ::std::option::Option::Some(v);
    }

    pub fn get_checkpoint_min_count(&self) -> i32 {
        self.checkpoint_min_count.unwrap_or(0)
    }

    fn get_checkpoint_min_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.checkpoint_min_count
    }

    fn mut_checkpoint_min_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.checkpoint_min_count
    }

    // required int32 subscriber_max_count = 15;

    pub fn clear_subscriber_max_count(&mut self) {
        self.subscriber_max_count = ::std::option::Option::None;
    }

    pub fn has_subscriber_max_count(&self) -> bool {
        self.subscriber_max_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscriber_max_count(&mut self, v: i32) {
        self.subscriber_max_count = ::std::option::Option::Some(v);
    }

    pub fn get_subscriber_max_count(&self) -> i32 {
        self.subscriber_max_count.unwrap_or(0)
    }

    fn get_subscriber_max_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.subscriber_max_count
    }

    fn mut_subscriber_max_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.subscriber_max_count
    }

    // optional string named_consumer_strategy = 16;

    pub fn clear_named_consumer_strategy(&mut self) {
        self.named_consumer_strategy = ::std::option::Option::None;
    }

    pub fn has_named_consumer_strategy(&self) -> bool {
        self.named_consumer_strategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_named_consumer_strategy(&mut self, v: ::protobuf::chars::Chars) {
        self.named_consumer_strategy = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_named_consumer_strategy(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.named_consumer_strategy.is_none() {
            self.named_consumer_strategy = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.named_consumer_strategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_named_consumer_strategy(&mut self) -> ::protobuf::chars::Chars {
        self.named_consumer_strategy.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_named_consumer_strategy(&self) -> &str {
        match self.named_consumer_strategy.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_named_consumer_strategy_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.named_consumer_strategy
    }

    fn mut_named_consumer_strategy_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.named_consumer_strategy
    }
}

impl ::protobuf::Message for UpdatePersistentSubscription {
    fn is_initialized(&self) -> bool {
        if self.subscription_group_name.is_none() {
            return false;
        }
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.resolve_link_tos.is_none() {
            return false;
        }
        if self.start_from.is_none() {
            return false;
        }
        if self.message_timeout_milliseconds.is_none() {
            return false;
        }
        if self.record_statistics.is_none() {
            return false;
        }
        if self.live_buffer_size.is_none() {
            return false;
        }
        if self.read_batch_size.is_none() {
            return false;
        }
        if self.buffer_size.is_none() {
            return false;
        }
        if self.max_retry_count.is_none() {
            return false;
        }
        if self.prefer_round_robin.is_none() {
            return false;
        }
        if self.checkpoint_after_time.is_none() {
            return false;
        }
        if self.checkpoint_max_count.is_none() {
            return false;
        }
        if self.checkpoint_min_count.is_none() {
            return false;
        }
        if self.subscriber_max_count.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.subscription_group_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.resolve_link_tos = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.start_from = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.message_timeout_milliseconds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.record_statistics = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.live_buffer_size = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.read_batch_size = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.buffer_size = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_retry_count = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.prefer_round_robin = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.checkpoint_after_time = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.checkpoint_max_count = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.checkpoint_min_count = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.subscriber_max_count = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.named_consumer_strategy)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.subscription_group_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.resolve_link_tos {
            my_size += 2;
        }
        if let Some(v) = self.start_from {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.message_timeout_milliseconds {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.record_statistics {
            my_size += 2;
        }
        if let Some(v) = self.live_buffer_size {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.read_batch_size {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.buffer_size {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max_retry_count {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prefer_round_robin {
            my_size += 2;
        }
        if let Some(v) = self.checkpoint_after_time {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.checkpoint_max_count {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.checkpoint_min_count {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.subscriber_max_count {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.named_consumer_strategy.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.subscription_group_name.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.resolve_link_tos {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.start_from {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.message_timeout_milliseconds {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.record_statistics {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.live_buffer_size {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.read_batch_size {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.buffer_size {
            os.write_int32(9, v)?;
        }
        if let Some(v) = self.max_retry_count {
            os.write_int32(10, v)?;
        }
        if let Some(v) = self.prefer_round_robin {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.checkpoint_after_time {
            os.write_int32(12, v)?;
        }
        if let Some(v) = self.checkpoint_max_count {
            os.write_int32(13, v)?;
        }
        if let Some(v) = self.checkpoint_min_count {
            os.write_int32(14, v)?;
        }
        if let Some(v) = self.subscriber_max_count {
            os.write_int32(15, v)?;
        }
        if let Some(ref v) = self.named_consumer_strategy.as_ref() {
            os.write_string(16, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdatePersistentSubscription {
    fn new() -> UpdatePersistentSubscription {
        UpdatePersistentSubscription::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdatePersistentSubscription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "subscription_group_name",
                    UpdatePersistentSubscription::get_subscription_group_name_for_reflect,
                    UpdatePersistentSubscription::mut_subscription_group_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    UpdatePersistentSubscription::get_event_stream_id_for_reflect,
                    UpdatePersistentSubscription::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "resolve_link_tos",
                    UpdatePersistentSubscription::get_resolve_link_tos_for_reflect,
                    UpdatePersistentSubscription::mut_resolve_link_tos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "start_from",
                    UpdatePersistentSubscription::get_start_from_for_reflect,
                    UpdatePersistentSubscription::mut_start_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "message_timeout_milliseconds",
                    UpdatePersistentSubscription::get_message_timeout_milliseconds_for_reflect,
                    UpdatePersistentSubscription::mut_message_timeout_milliseconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "record_statistics",
                    UpdatePersistentSubscription::get_record_statistics_for_reflect,
                    UpdatePersistentSubscription::mut_record_statistics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "live_buffer_size",
                    UpdatePersistentSubscription::get_live_buffer_size_for_reflect,
                    UpdatePersistentSubscription::mut_live_buffer_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "read_batch_size",
                    UpdatePersistentSubscription::get_read_batch_size_for_reflect,
                    UpdatePersistentSubscription::mut_read_batch_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "buffer_size",
                    UpdatePersistentSubscription::get_buffer_size_for_reflect,
                    UpdatePersistentSubscription::mut_buffer_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_retry_count",
                    UpdatePersistentSubscription::get_max_retry_count_for_reflect,
                    UpdatePersistentSubscription::mut_max_retry_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prefer_round_robin",
                    UpdatePersistentSubscription::get_prefer_round_robin_for_reflect,
                    UpdatePersistentSubscription::mut_prefer_round_robin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "checkpoint_after_time",
                    UpdatePersistentSubscription::get_checkpoint_after_time_for_reflect,
                    UpdatePersistentSubscription::mut_checkpoint_after_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "checkpoint_max_count",
                    UpdatePersistentSubscription::get_checkpoint_max_count_for_reflect,
                    UpdatePersistentSubscription::mut_checkpoint_max_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "checkpoint_min_count",
                    UpdatePersistentSubscription::get_checkpoint_min_count_for_reflect,
                    UpdatePersistentSubscription::mut_checkpoint_min_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "subscriber_max_count",
                    UpdatePersistentSubscription::get_subscriber_max_count_for_reflect,
                    UpdatePersistentSubscription::mut_subscriber_max_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "named_consumer_strategy",
                    UpdatePersistentSubscription::get_named_consumer_strategy_for_reflect,
                    UpdatePersistentSubscription::mut_named_consumer_strategy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdatePersistentSubscription>(
                    "UpdatePersistentSubscription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdatePersistentSubscription {
    fn clear(&mut self) {
        self.clear_subscription_group_name();
        self.clear_event_stream_id();
        self.clear_resolve_link_tos();
        self.clear_start_from();
        self.clear_message_timeout_milliseconds();
        self.clear_record_statistics();
        self.clear_live_buffer_size();
        self.clear_read_batch_size();
        self.clear_buffer_size();
        self.clear_max_retry_count();
        self.clear_prefer_round_robin();
        self.clear_checkpoint_after_time();
        self.clear_checkpoint_max_count();
        self.clear_checkpoint_min_count();
        self.clear_subscriber_max_count();
        self.clear_named_consumer_strategy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdatePersistentSubscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdatePersistentSubscription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdatePersistentSubscriptionCompleted {
    // message fields
    result: ::std::option::Option<UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult>,
    reason: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdatePersistentSubscriptionCompleted {}

impl UpdatePersistentSubscriptionCompleted {
    pub fn new() -> UpdatePersistentSubscriptionCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdatePersistentSubscriptionCompleted {
        static mut instance: ::protobuf::lazy::Lazy<UpdatePersistentSubscriptionCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdatePersistentSubscriptionCompleted,
        };
        unsafe {
            instance.get(UpdatePersistentSubscriptionCompleted::new)
        }
    }

    // required .EventStore.Client.Messages.UpdatePersistentSubscriptionCompleted.UpdatePersistentSubscriptionResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult {
        self.result.unwrap_or(UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult> {
        &mut self.result
    }

    // optional string reason = 2;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::protobuf::chars::Chars) {
        self.reason = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.reason.is_none() {
            self.reason = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::protobuf::chars::Chars {
        self.reason.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.reason
    }
}

impl ::protobuf::Message for UpdatePersistentSubscriptionCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.reason)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdatePersistentSubscriptionCompleted {
    fn new() -> UpdatePersistentSubscriptionCompleted {
        UpdatePersistentSubscriptionCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdatePersistentSubscriptionCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult>>(
                    "result",
                    UpdatePersistentSubscriptionCompleted::get_result_for_reflect,
                    UpdatePersistentSubscriptionCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "reason",
                    UpdatePersistentSubscriptionCompleted::get_reason_for_reflect,
                    UpdatePersistentSubscriptionCompleted::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdatePersistentSubscriptionCompleted>(
                    "UpdatePersistentSubscriptionCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdatePersistentSubscriptionCompleted {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdatePersistentSubscriptionCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdatePersistentSubscriptionCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult {
    Success = 0,
    DoesNotExist = 1,
    Fail = 2,
    AccessDenied = 3,
}

impl ::protobuf::ProtobufEnum for UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult> {
        match value {
            0 => ::std::option::Option::Some(UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::Success),
            1 => ::std::option::Option::Some(UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::DoesNotExist),
            2 => ::std::option::Option::Some(UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::Fail),
            3 => ::std::option::Option::Some(UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::AccessDenied),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult] = &[
            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::Success,
            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::DoesNotExist,
            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::Fail,
            UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult::AccessDenied,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult {
}

impl ::protobuf::reflect::ProtobufValue for UpdatePersistentSubscriptionCompleted_UpdatePersistentSubscriptionResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreatePersistentSubscriptionCompleted {
    // message fields
    result: ::std::option::Option<CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult>,
    reason: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreatePersistentSubscriptionCompleted {}

impl CreatePersistentSubscriptionCompleted {
    pub fn new() -> CreatePersistentSubscriptionCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreatePersistentSubscriptionCompleted {
        static mut instance: ::protobuf::lazy::Lazy<CreatePersistentSubscriptionCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreatePersistentSubscriptionCompleted,
        };
        unsafe {
            instance.get(CreatePersistentSubscriptionCompleted::new)
        }
    }

    // required .EventStore.Client.Messages.CreatePersistentSubscriptionCompleted.CreatePersistentSubscriptionResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult {
        self.result.unwrap_or(CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult> {
        &mut self.result
    }

    // optional string reason = 2;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::protobuf::chars::Chars) {
        self.reason = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.reason.is_none() {
            self.reason = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::protobuf::chars::Chars {
        self.reason.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.reason
    }
}

impl ::protobuf::Message for CreatePersistentSubscriptionCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.reason)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreatePersistentSubscriptionCompleted {
    fn new() -> CreatePersistentSubscriptionCompleted {
        CreatePersistentSubscriptionCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreatePersistentSubscriptionCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult>>(
                    "result",
                    CreatePersistentSubscriptionCompleted::get_result_for_reflect,
                    CreatePersistentSubscriptionCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "reason",
                    CreatePersistentSubscriptionCompleted::get_reason_for_reflect,
                    CreatePersistentSubscriptionCompleted::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreatePersistentSubscriptionCompleted>(
                    "CreatePersistentSubscriptionCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreatePersistentSubscriptionCompleted {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreatePersistentSubscriptionCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreatePersistentSubscriptionCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult {
    Success = 0,
    AlreadyExists = 1,
    Fail = 2,
    AccessDenied = 3,
}

impl ::protobuf::ProtobufEnum for CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult> {
        match value {
            0 => ::std::option::Option::Some(CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::Success),
            1 => ::std::option::Option::Some(CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::AlreadyExists),
            2 => ::std::option::Option::Some(CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::Fail),
            3 => ::std::option::Option::Some(CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::AccessDenied),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult] = &[
            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::Success,
            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::AlreadyExists,
            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::Fail,
            CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult::AccessDenied,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult {
}

impl ::protobuf::reflect::ProtobufValue for CreatePersistentSubscriptionCompleted_CreatePersistentSubscriptionResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeletePersistentSubscriptionCompleted {
    // message fields
    result: ::std::option::Option<DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult>,
    reason: ::std::option::Option<::protobuf::chars::Chars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeletePersistentSubscriptionCompleted {}

impl DeletePersistentSubscriptionCompleted {
    pub fn new() -> DeletePersistentSubscriptionCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeletePersistentSubscriptionCompleted {
        static mut instance: ::protobuf::lazy::Lazy<DeletePersistentSubscriptionCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeletePersistentSubscriptionCompleted,
        };
        unsafe {
            instance.get(DeletePersistentSubscriptionCompleted::new)
        }
    }

    // required .EventStore.Client.Messages.DeletePersistentSubscriptionCompleted.DeletePersistentSubscriptionResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult {
        self.result.unwrap_or(DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult> {
        &mut self.result
    }

    // optional string reason = 2;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::protobuf::chars::Chars) {
        self.reason = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.reason.is_none() {
            self.reason = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::protobuf::chars::Chars {
        self.reason.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.reason
    }
}

impl ::protobuf::Message for DeletePersistentSubscriptionCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.reason)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeletePersistentSubscriptionCompleted {
    fn new() -> DeletePersistentSubscriptionCompleted {
        DeletePersistentSubscriptionCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeletePersistentSubscriptionCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult>>(
                    "result",
                    DeletePersistentSubscriptionCompleted::get_result_for_reflect,
                    DeletePersistentSubscriptionCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "reason",
                    DeletePersistentSubscriptionCompleted::get_reason_for_reflect,
                    DeletePersistentSubscriptionCompleted::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeletePersistentSubscriptionCompleted>(
                    "DeletePersistentSubscriptionCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeletePersistentSubscriptionCompleted {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeletePersistentSubscriptionCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeletePersistentSubscriptionCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult {
    Success = 0,
    DoesNotExist = 1,
    Fail = 2,
    AccessDenied = 3,
}

impl ::protobuf::ProtobufEnum for DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult> {
        match value {
            0 => ::std::option::Option::Some(DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::Success),
            1 => ::std::option::Option::Some(DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::DoesNotExist),
            2 => ::std::option::Option::Some(DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::Fail),
            3 => ::std::option::Option::Some(DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::AccessDenied),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult] = &[
            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::Success,
            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::DoesNotExist,
            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::Fail,
            DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult::AccessDenied,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult {
}

impl ::protobuf::reflect::ProtobufValue for DeletePersistentSubscriptionCompleted_DeletePersistentSubscriptionResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectToPersistentSubscription {
    // message fields
    subscription_id: ::std::option::Option<::protobuf::chars::Chars>,
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    allowed_in_flight_messages: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectToPersistentSubscription {}

impl ConnectToPersistentSubscription {
    pub fn new() -> ConnectToPersistentSubscription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectToPersistentSubscription {
        static mut instance: ::protobuf::lazy::Lazy<ConnectToPersistentSubscription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectToPersistentSubscription,
        };
        unsafe {
            instance.get(ConnectToPersistentSubscription::new)
        }
    }

    // required string subscription_id = 1;

    pub fn clear_subscription_id(&mut self) {
        self.subscription_id = ::std::option::Option::None;
    }

    pub fn has_subscription_id(&self) -> bool {
        self.subscription_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscription_id(&mut self, v: ::protobuf::chars::Chars) {
        self.subscription_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscription_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.subscription_id.is_none() {
            self.subscription_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.subscription_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscription_id(&mut self) -> ::protobuf::chars::Chars {
        self.subscription_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_subscription_id(&self) -> &str {
        match self.subscription_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_subscription_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.subscription_id
    }

    fn mut_subscription_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.subscription_id
    }

    // required string event_stream_id = 2;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required int32 allowed_in_flight_messages = 3;

    pub fn clear_allowed_in_flight_messages(&mut self) {
        self.allowed_in_flight_messages = ::std::option::Option::None;
    }

    pub fn has_allowed_in_flight_messages(&self) -> bool {
        self.allowed_in_flight_messages.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed_in_flight_messages(&mut self, v: i32) {
        self.allowed_in_flight_messages = ::std::option::Option::Some(v);
    }

    pub fn get_allowed_in_flight_messages(&self) -> i32 {
        self.allowed_in_flight_messages.unwrap_or(0)
    }

    fn get_allowed_in_flight_messages_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.allowed_in_flight_messages
    }

    fn mut_allowed_in_flight_messages_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.allowed_in_flight_messages
    }
}

impl ::protobuf::Message for ConnectToPersistentSubscription {
    fn is_initialized(&self) -> bool {
        if self.subscription_id.is_none() {
            return false;
        }
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.allowed_in_flight_messages.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.subscription_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.allowed_in_flight_messages = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.subscription_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.allowed_in_flight_messages {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.subscription_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.allowed_in_flight_messages {
            os.write_int32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConnectToPersistentSubscription {
    fn new() -> ConnectToPersistentSubscription {
        ConnectToPersistentSubscription::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectToPersistentSubscription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "subscription_id",
                    ConnectToPersistentSubscription::get_subscription_id_for_reflect,
                    ConnectToPersistentSubscription::mut_subscription_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    ConnectToPersistentSubscription::get_event_stream_id_for_reflect,
                    ConnectToPersistentSubscription::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "allowed_in_flight_messages",
                    ConnectToPersistentSubscription::get_allowed_in_flight_messages_for_reflect,
                    ConnectToPersistentSubscription::mut_allowed_in_flight_messages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConnectToPersistentSubscription>(
                    "ConnectToPersistentSubscription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectToPersistentSubscription {
    fn clear(&mut self) {
        self.clear_subscription_id();
        self.clear_event_stream_id();
        self.clear_allowed_in_flight_messages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectToPersistentSubscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectToPersistentSubscription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PersistentSubscriptionAckEvents {
    // message fields
    subscription_id: ::std::option::Option<::protobuf::chars::Chars>,
    processed_event_ids: ::std::vec::Vec<::bytes::Bytes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PersistentSubscriptionAckEvents {}

impl PersistentSubscriptionAckEvents {
    pub fn new() -> PersistentSubscriptionAckEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PersistentSubscriptionAckEvents {
        static mut instance: ::protobuf::lazy::Lazy<PersistentSubscriptionAckEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PersistentSubscriptionAckEvents,
        };
        unsafe {
            instance.get(PersistentSubscriptionAckEvents::new)
        }
    }

    // required string subscription_id = 1;

    pub fn clear_subscription_id(&mut self) {
        self.subscription_id = ::std::option::Option::None;
    }

    pub fn has_subscription_id(&self) -> bool {
        self.subscription_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscription_id(&mut self, v: ::protobuf::chars::Chars) {
        self.subscription_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscription_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.subscription_id.is_none() {
            self.subscription_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.subscription_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscription_id(&mut self) -> ::protobuf::chars::Chars {
        self.subscription_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_subscription_id(&self) -> &str {
        match self.subscription_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_subscription_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.subscription_id
    }

    fn mut_subscription_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.subscription_id
    }

    // repeated bytes processed_event_ids = 2;

    pub fn clear_processed_event_ids(&mut self) {
        self.processed_event_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_processed_event_ids(&mut self, v: ::std::vec::Vec<::bytes::Bytes>) {
        self.processed_event_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_processed_event_ids(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.processed_event_ids
    }

    // Take field
    pub fn take_processed_event_ids(&mut self) -> ::std::vec::Vec<::bytes::Bytes> {
        ::std::mem::replace(&mut self.processed_event_ids, ::std::vec::Vec::new())
    }

    pub fn get_processed_event_ids(&self) -> &[::bytes::Bytes] {
        &self.processed_event_ids
    }

    fn get_processed_event_ids_for_reflect(&self) -> &::std::vec::Vec<::bytes::Bytes> {
        &self.processed_event_ids
    }

    fn mut_processed_event_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.processed_event_ids
    }
}

impl ::protobuf::Message for PersistentSubscriptionAckEvents {
    fn is_initialized(&self) -> bool {
        if self.subscription_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.subscription_id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_carllerche_bytes_into(wire_type, is, &mut self.processed_event_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.subscription_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.processed_event_ids {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.subscription_id.as_ref() {
            os.write_string(1, v)?;
        }
        for v in &self.processed_event_ids {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PersistentSubscriptionAckEvents {
    fn new() -> PersistentSubscriptionAckEvents {
        PersistentSubscriptionAckEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<PersistentSubscriptionAckEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "subscription_id",
                    PersistentSubscriptionAckEvents::get_subscription_id_for_reflect,
                    PersistentSubscriptionAckEvents::mut_subscription_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "processed_event_ids",
                    PersistentSubscriptionAckEvents::get_processed_event_ids_for_reflect,
                    PersistentSubscriptionAckEvents::mut_processed_event_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PersistentSubscriptionAckEvents>(
                    "PersistentSubscriptionAckEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PersistentSubscriptionAckEvents {
    fn clear(&mut self) {
        self.clear_subscription_id();
        self.clear_processed_event_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PersistentSubscriptionAckEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PersistentSubscriptionAckEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PersistentSubscriptionNakEvents {
    // message fields
    subscription_id: ::std::option::Option<::protobuf::chars::Chars>,
    processed_event_ids: ::std::vec::Vec<::bytes::Bytes>,
    message: ::std::option::Option<::protobuf::chars::Chars>,
    action: ::std::option::Option<PersistentSubscriptionNakEvents_NakAction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PersistentSubscriptionNakEvents {}

impl PersistentSubscriptionNakEvents {
    pub fn new() -> PersistentSubscriptionNakEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PersistentSubscriptionNakEvents {
        static mut instance: ::protobuf::lazy::Lazy<PersistentSubscriptionNakEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PersistentSubscriptionNakEvents,
        };
        unsafe {
            instance.get(PersistentSubscriptionNakEvents::new)
        }
    }

    // required string subscription_id = 1;

    pub fn clear_subscription_id(&mut self) {
        self.subscription_id = ::std::option::Option::None;
    }

    pub fn has_subscription_id(&self) -> bool {
        self.subscription_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscription_id(&mut self, v: ::protobuf::chars::Chars) {
        self.subscription_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscription_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.subscription_id.is_none() {
            self.subscription_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.subscription_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscription_id(&mut self) -> ::protobuf::chars::Chars {
        self.subscription_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_subscription_id(&self) -> &str {
        match self.subscription_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_subscription_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.subscription_id
    }

    fn mut_subscription_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.subscription_id
    }

    // repeated bytes processed_event_ids = 2;

    pub fn clear_processed_event_ids(&mut self) {
        self.processed_event_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_processed_event_ids(&mut self, v: ::std::vec::Vec<::bytes::Bytes>) {
        self.processed_event_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_processed_event_ids(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.processed_event_ids
    }

    // Take field
    pub fn take_processed_event_ids(&mut self) -> ::std::vec::Vec<::bytes::Bytes> {
        ::std::mem::replace(&mut self.processed_event_ids, ::std::vec::Vec::new())
    }

    pub fn get_processed_event_ids(&self) -> &[::bytes::Bytes] {
        &self.processed_event_ids
    }

    fn get_processed_event_ids_for_reflect(&self) -> &::std::vec::Vec<::bytes::Bytes> {
        &self.processed_event_ids
    }

    fn mut_processed_event_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.processed_event_ids
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None;
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::protobuf::chars::Chars) {
        self.message = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::protobuf::chars::Chars {
        self.message.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.message
    }

    // required .EventStore.Client.Messages.PersistentSubscriptionNakEvents.NakAction action = 4;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: PersistentSubscriptionNakEvents_NakAction) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> PersistentSubscriptionNakEvents_NakAction {
        self.action.unwrap_or(PersistentSubscriptionNakEvents_NakAction::Unknown)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<PersistentSubscriptionNakEvents_NakAction> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<PersistentSubscriptionNakEvents_NakAction> {
        &mut self.action
    }
}

impl ::protobuf::Message for PersistentSubscriptionNakEvents {
    fn is_initialized(&self) -> bool {
        if self.subscription_id.is_none() {
            return false;
        }
        if self.action.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.subscription_id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_carllerche_bytes_into(wire_type, is, &mut self.processed_event_ids)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.message)?;
                },
                4 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.action, 4, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.subscription_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.processed_event_ids {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.subscription_id.as_ref() {
            os.write_string(1, v)?;
        }
        for v in &self.processed_event_ids {
            os.write_bytes(2, &v)?;
        };
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(3, v)?;
        }
        if let Some(v) = self.action {
            os.write_enum(4, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PersistentSubscriptionNakEvents {
    fn new() -> PersistentSubscriptionNakEvents {
        PersistentSubscriptionNakEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<PersistentSubscriptionNakEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "subscription_id",
                    PersistentSubscriptionNakEvents::get_subscription_id_for_reflect,
                    PersistentSubscriptionNakEvents::mut_subscription_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "processed_event_ids",
                    PersistentSubscriptionNakEvents::get_processed_event_ids_for_reflect,
                    PersistentSubscriptionNakEvents::mut_processed_event_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "message",
                    PersistentSubscriptionNakEvents::get_message_for_reflect,
                    PersistentSubscriptionNakEvents::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PersistentSubscriptionNakEvents_NakAction>>(
                    "action",
                    PersistentSubscriptionNakEvents::get_action_for_reflect,
                    PersistentSubscriptionNakEvents::mut_action_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PersistentSubscriptionNakEvents>(
                    "PersistentSubscriptionNakEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PersistentSubscriptionNakEvents {
    fn clear(&mut self) {
        self.clear_subscription_id();
        self.clear_processed_event_ids();
        self.clear_message();
        self.clear_action();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PersistentSubscriptionNakEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PersistentSubscriptionNakEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PersistentSubscriptionNakEvents_NakAction {
    Unknown = 0,
    Park = 1,
    Retry = 2,
    Skip = 3,
    Stop = 4,
}

impl ::protobuf::ProtobufEnum for PersistentSubscriptionNakEvents_NakAction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PersistentSubscriptionNakEvents_NakAction> {
        match value {
            0 => ::std::option::Option::Some(PersistentSubscriptionNakEvents_NakAction::Unknown),
            1 => ::std::option::Option::Some(PersistentSubscriptionNakEvents_NakAction::Park),
            2 => ::std::option::Option::Some(PersistentSubscriptionNakEvents_NakAction::Retry),
            3 => ::std::option::Option::Some(PersistentSubscriptionNakEvents_NakAction::Skip),
            4 => ::std::option::Option::Some(PersistentSubscriptionNakEvents_NakAction::Stop),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PersistentSubscriptionNakEvents_NakAction] = &[
            PersistentSubscriptionNakEvents_NakAction::Unknown,
            PersistentSubscriptionNakEvents_NakAction::Park,
            PersistentSubscriptionNakEvents_NakAction::Retry,
            PersistentSubscriptionNakEvents_NakAction::Skip,
            PersistentSubscriptionNakEvents_NakAction::Stop,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<PersistentSubscriptionNakEvents_NakAction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PersistentSubscriptionNakEvents_NakAction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PersistentSubscriptionNakEvents_NakAction {
}

impl ::protobuf::reflect::ProtobufValue for PersistentSubscriptionNakEvents_NakAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PersistentSubscriptionConfirmation {
    // message fields
    last_commit_position: ::std::option::Option<i64>,
    subscription_id: ::std::option::Option<::protobuf::chars::Chars>,
    last_event_number: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PersistentSubscriptionConfirmation {}

impl PersistentSubscriptionConfirmation {
    pub fn new() -> PersistentSubscriptionConfirmation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PersistentSubscriptionConfirmation {
        static mut instance: ::protobuf::lazy::Lazy<PersistentSubscriptionConfirmation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PersistentSubscriptionConfirmation,
        };
        unsafe {
            instance.get(PersistentSubscriptionConfirmation::new)
        }
    }

    // required int64 last_commit_position = 1;

    pub fn clear_last_commit_position(&mut self) {
        self.last_commit_position = ::std::option::Option::None;
    }

    pub fn has_last_commit_position(&self) -> bool {
        self.last_commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_commit_position(&mut self, v: i64) {
        self.last_commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_last_commit_position(&self) -> i64 {
        self.last_commit_position.unwrap_or(0)
    }

    fn get_last_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_commit_position
    }

    fn mut_last_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_commit_position
    }

    // required string subscription_id = 2;

    pub fn clear_subscription_id(&mut self) {
        self.subscription_id = ::std::option::Option::None;
    }

    pub fn has_subscription_id(&self) -> bool {
        self.subscription_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscription_id(&mut self, v: ::protobuf::chars::Chars) {
        self.subscription_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subscription_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.subscription_id.is_none() {
            self.subscription_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.subscription_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_subscription_id(&mut self) -> ::protobuf::chars::Chars {
        self.subscription_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_subscription_id(&self) -> &str {
        match self.subscription_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_subscription_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.subscription_id
    }

    fn mut_subscription_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.subscription_id
    }

    // optional int64 last_event_number = 3;

    pub fn clear_last_event_number(&mut self) {
        self.last_event_number = ::std::option::Option::None;
    }

    pub fn has_last_event_number(&self) -> bool {
        self.last_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_event_number(&mut self, v: i64) {
        self.last_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_last_event_number(&self) -> i64 {
        self.last_event_number.unwrap_or(0)
    }

    fn get_last_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_event_number
    }

    fn mut_last_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_event_number
    }
}

impl ::protobuf::Message for PersistentSubscriptionConfirmation {
    fn is_initialized(&self) -> bool {
        if self.last_commit_position.is_none() {
            return false;
        }
        if self.subscription_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_commit_position = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.subscription_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_event_number = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.last_commit_position {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.subscription_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.last_event_number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.last_commit_position {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.subscription_id.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.last_event_number {
            os.write_int64(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PersistentSubscriptionConfirmation {
    fn new() -> PersistentSubscriptionConfirmation {
        PersistentSubscriptionConfirmation::new()
    }

    fn descriptor_static(_: ::std::option::Option<PersistentSubscriptionConfirmation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_commit_position",
                    PersistentSubscriptionConfirmation::get_last_commit_position_for_reflect,
                    PersistentSubscriptionConfirmation::mut_last_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "subscription_id",
                    PersistentSubscriptionConfirmation::get_subscription_id_for_reflect,
                    PersistentSubscriptionConfirmation::mut_subscription_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_event_number",
                    PersistentSubscriptionConfirmation::get_last_event_number_for_reflect,
                    PersistentSubscriptionConfirmation::mut_last_event_number_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PersistentSubscriptionConfirmation>(
                    "PersistentSubscriptionConfirmation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PersistentSubscriptionConfirmation {
    fn clear(&mut self) {
        self.clear_last_commit_position();
        self.clear_subscription_id();
        self.clear_last_event_number();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PersistentSubscriptionConfirmation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PersistentSubscriptionConfirmation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PersistentSubscriptionStreamEventAppeared {
    // message fields
    event: ::protobuf::SingularPtrField<ResolvedIndexedEvent>,
    retryCount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PersistentSubscriptionStreamEventAppeared {}

impl PersistentSubscriptionStreamEventAppeared {
    pub fn new() -> PersistentSubscriptionStreamEventAppeared {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PersistentSubscriptionStreamEventAppeared {
        static mut instance: ::protobuf::lazy::Lazy<PersistentSubscriptionStreamEventAppeared> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PersistentSubscriptionStreamEventAppeared,
        };
        unsafe {
            instance.get(PersistentSubscriptionStreamEventAppeared::new)
        }
    }

    // required .EventStore.Client.Messages.ResolvedIndexedEvent event = 1;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: ResolvedIndexedEvent) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut ResolvedIndexedEvent {
        if self.event.is_none() {
            self.event.set_default();
        }
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> ResolvedIndexedEvent {
        self.event.take().unwrap_or_else(|| ResolvedIndexedEvent::new())
    }

    pub fn get_event(&self) -> &ResolvedIndexedEvent {
        self.event.as_ref().unwrap_or_else(|| ResolvedIndexedEvent::default_instance())
    }

    fn get_event_for_reflect(&self) -> &::protobuf::SingularPtrField<ResolvedIndexedEvent> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResolvedIndexedEvent> {
        &mut self.event
    }

    // optional int32 retryCount = 2;

    pub fn clear_retryCount(&mut self) {
        self.retryCount = ::std::option::Option::None;
    }

    pub fn has_retryCount(&self) -> bool {
        self.retryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retryCount(&mut self, v: i32) {
        self.retryCount = ::std::option::Option::Some(v);
    }

    pub fn get_retryCount(&self) -> i32 {
        self.retryCount.unwrap_or(0)
    }

    fn get_retryCount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.retryCount
    }

    fn mut_retryCount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.retryCount
    }
}

impl ::protobuf::Message for PersistentSubscriptionStreamEventAppeared {
    fn is_initialized(&self) -> bool {
        if self.event.is_none() {
            return false;
        }
        for v in &self.event {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.retryCount = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.retryCount {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.retryCount {
            os.write_int32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PersistentSubscriptionStreamEventAppeared {
    fn new() -> PersistentSubscriptionStreamEventAppeared {
        PersistentSubscriptionStreamEventAppeared::new()
    }

    fn descriptor_static(_: ::std::option::Option<PersistentSubscriptionStreamEventAppeared>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResolvedIndexedEvent>>(
                    "event",
                    PersistentSubscriptionStreamEventAppeared::get_event_for_reflect,
                    PersistentSubscriptionStreamEventAppeared::mut_event_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "retryCount",
                    PersistentSubscriptionStreamEventAppeared::get_retryCount_for_reflect,
                    PersistentSubscriptionStreamEventAppeared::mut_retryCount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PersistentSubscriptionStreamEventAppeared>(
                    "PersistentSubscriptionStreamEventAppeared",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PersistentSubscriptionStreamEventAppeared {
    fn clear(&mut self) {
        self.clear_event();
        self.clear_retryCount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PersistentSubscriptionStreamEventAppeared {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PersistentSubscriptionStreamEventAppeared {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubscribeToStream {
    // message fields
    event_stream_id: ::std::option::Option<::protobuf::chars::Chars>,
    resolve_link_tos: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubscribeToStream {}

impl SubscribeToStream {
    pub fn new() -> SubscribeToStream {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubscribeToStream {
        static mut instance: ::protobuf::lazy::Lazy<SubscribeToStream> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubscribeToStream,
        };
        unsafe {
            instance.get(SubscribeToStream::new)
        }
    }

    // required string event_stream_id = 1;

    pub fn clear_event_stream_id(&mut self) {
        self.event_stream_id = ::std::option::Option::None;
    }

    pub fn has_event_stream_id(&self) -> bool {
        self.event_stream_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_stream_id(&mut self, v: ::protobuf::chars::Chars) {
        self.event_stream_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_stream_id(&mut self) -> &mut ::protobuf::chars::Chars {
        if self.event_stream_id.is_none() {
            self.event_stream_id = ::std::option::Option::Some(::protobuf::chars::Chars::new());
        }
        self.event_stream_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_stream_id(&mut self) -> ::protobuf::chars::Chars {
        self.event_stream_id.take().unwrap_or_else(|| ::protobuf::chars::Chars::new())
    }

    pub fn get_event_stream_id(&self) -> &str {
        match self.event_stream_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    fn get_event_stream_id_for_reflect(&self) -> &::std::option::Option<::protobuf::chars::Chars> {
        &self.event_stream_id
    }

    fn mut_event_stream_id_for_reflect(&mut self) -> &mut ::std::option::Option<::protobuf::chars::Chars> {
        &mut self.event_stream_id
    }

    // required bool resolve_link_tos = 2;

    pub fn clear_resolve_link_tos(&mut self) {
        self.resolve_link_tos = ::std::option::Option::None;
    }

    pub fn has_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolve_link_tos(&mut self, v: bool) {
        self.resolve_link_tos = ::std::option::Option::Some(v);
    }

    pub fn get_resolve_link_tos(&self) -> bool {
        self.resolve_link_tos.unwrap_or(false)
    }

    fn get_resolve_link_tos_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.resolve_link_tos
    }

    fn mut_resolve_link_tos_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.resolve_link_tos
    }
}

impl ::protobuf::Message for SubscribeToStream {
    fn is_initialized(&self) -> bool {
        if self.event_stream_id.is_none() {
            return false;
        }
        if self.resolve_link_tos.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_carllerche_string_into(wire_type, is, &mut self.event_stream_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.resolve_link_tos = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event_stream_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.resolve_link_tos {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_stream_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.resolve_link_tos {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubscribeToStream {
    fn new() -> SubscribeToStream {
        SubscribeToStream::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubscribeToStream>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "event_stream_id",
                    SubscribeToStream::get_event_stream_id_for_reflect,
                    SubscribeToStream::mut_event_stream_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "resolve_link_tos",
                    SubscribeToStream::get_resolve_link_tos_for_reflect,
                    SubscribeToStream::mut_resolve_link_tos_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubscribeToStream>(
                    "SubscribeToStream",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubscribeToStream {
    fn clear(&mut self) {
        self.clear_event_stream_id();
        self.clear_resolve_link_tos();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscribeToStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscribeToStream {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubscriptionConfirmation {
    // message fields
    last_commit_position: ::std::option::Option<i64>,
    last_event_number: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubscriptionConfirmation {}

impl SubscriptionConfirmation {
    pub fn new() -> SubscriptionConfirmation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubscriptionConfirmation {
        static mut instance: ::protobuf::lazy::Lazy<SubscriptionConfirmation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubscriptionConfirmation,
        };
        unsafe {
            instance.get(SubscriptionConfirmation::new)
        }
    }

    // required int64 last_commit_position = 1;

    pub fn clear_last_commit_position(&mut self) {
        self.last_commit_position = ::std::option::Option::None;
    }

    pub fn has_last_commit_position(&self) -> bool {
        self.last_commit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_commit_position(&mut self, v: i64) {
        self.last_commit_position = ::std::option::Option::Some(v);
    }

    pub fn get_last_commit_position(&self) -> i64 {
        self.last_commit_position.unwrap_or(0)
    }

    fn get_last_commit_position_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_commit_position
    }

    fn mut_last_commit_position_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_commit_position
    }

    // optional int64 last_event_number = 2;

    pub fn clear_last_event_number(&mut self) {
        self.last_event_number = ::std::option::Option::None;
    }

    pub fn has_last_event_number(&self) -> bool {
        self.last_event_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_event_number(&mut self, v: i64) {
        self.last_event_number = ::std::option::Option::Some(v);
    }

    pub fn get_last_event_number(&self) -> i64 {
        self.last_event_number.unwrap_or(0)
    }

    fn get_last_event_number_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.last_event_number
    }

    fn mut_last_event_number_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.last_event_number
    }
}

impl ::protobuf::Message for SubscriptionConfirmation {
    fn is_initialized(&self) -> bool {
        if self.last_commit_position.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_commit_position = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_event_number = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.last_commit_position {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.last_event_number {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.last_commit_position {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.last_event_number {
            os.write_int64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubscriptionConfirmation {
    fn new() -> SubscriptionConfirmation {
        SubscriptionConfirmation::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubscriptionConfirmation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_commit_position",
                    SubscriptionConfirmation::get_last_commit_position_for_reflect,
                    SubscriptionConfirmation::mut_last_commit_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_event_number",
                    SubscriptionConfirmation::get_last_event_number_for_reflect,
                    SubscriptionConfirmation::mut_last_event_number_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubscriptionConfirmation>(
                    "SubscriptionConfirmation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubscriptionConfirmation {
    fn clear(&mut self) {
        self.clear_last_commit_position();
        self.clear_last_event_number();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscriptionConfirmation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscriptionConfirmation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamEventAppeared {
    // message fields
    event: ::protobuf::SingularPtrField<ResolvedEvent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamEventAppeared {}

impl StreamEventAppeared {
    pub fn new() -> StreamEventAppeared {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamEventAppeared {
        static mut instance: ::protobuf::lazy::Lazy<StreamEventAppeared> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamEventAppeared,
        };
        unsafe {
            instance.get(StreamEventAppeared::new)
        }
    }

    // required .EventStore.Client.Messages.ResolvedEvent event = 1;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: ResolvedEvent) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut ResolvedEvent {
        if self.event.is_none() {
            self.event.set_default();
        }
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> ResolvedEvent {
        self.event.take().unwrap_or_else(|| ResolvedEvent::new())
    }

    pub fn get_event(&self) -> &ResolvedEvent {
        self.event.as_ref().unwrap_or_else(|| ResolvedEvent::default_instance())
    }

    fn get_event_for_reflect(&self) -> &::protobuf::SingularPtrField<ResolvedEvent> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResolvedEvent> {
        &mut self.event
    }
}

impl ::protobuf::Message for StreamEventAppeared {
    fn is_initialized(&self) -> bool {
        if self.event.is_none() {
            return false;
        }
        for v in &self.event {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StreamEventAppeared {
    fn new() -> StreamEventAppeared {
        StreamEventAppeared::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamEventAppeared>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResolvedEvent>>(
                    "event",
                    StreamEventAppeared::get_event_for_reflect,
                    StreamEventAppeared::mut_event_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamEventAppeared>(
                    "StreamEventAppeared",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamEventAppeared {
    fn clear(&mut self) {
        self.clear_event();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamEventAppeared {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamEventAppeared {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnsubscribeFromStream {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnsubscribeFromStream {}

impl UnsubscribeFromStream {
    pub fn new() -> UnsubscribeFromStream {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnsubscribeFromStream {
        static mut instance: ::protobuf::lazy::Lazy<UnsubscribeFromStream> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnsubscribeFromStream,
        };
        unsafe {
            instance.get(UnsubscribeFromStream::new)
        }
    }
}

impl ::protobuf::Message for UnsubscribeFromStream {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UnsubscribeFromStream {
    fn new() -> UnsubscribeFromStream {
        UnsubscribeFromStream::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnsubscribeFromStream>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<UnsubscribeFromStream>(
                    "UnsubscribeFromStream",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnsubscribeFromStream {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnsubscribeFromStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnsubscribeFromStream {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubscriptionDropped {
    // message fields
    reason: ::std::option::Option<SubscriptionDropped_SubscriptionDropReason>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubscriptionDropped {}

impl SubscriptionDropped {
    pub fn new() -> SubscriptionDropped {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubscriptionDropped {
        static mut instance: ::protobuf::lazy::Lazy<SubscriptionDropped> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubscriptionDropped,
        };
        unsafe {
            instance.get(SubscriptionDropped::new)
        }
    }

    // optional .EventStore.Client.Messages.SubscriptionDropped.SubscriptionDropReason reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: SubscriptionDropped_SubscriptionDropReason) {
        self.reason = ::std::option::Option::Some(v);
    }

    pub fn get_reason(&self) -> SubscriptionDropped_SubscriptionDropReason {
        self.reason.unwrap_or(SubscriptionDropped_SubscriptionDropReason::Unsubscribed)
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<SubscriptionDropped_SubscriptionDropReason> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<SubscriptionDropped_SubscriptionDropReason> {
        &mut self.reason
    }
}

impl ::protobuf::Message for SubscriptionDropped {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.reason, 1, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reason {
            os.write_enum(1, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubscriptionDropped {
    fn new() -> SubscriptionDropped {
        SubscriptionDropped::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubscriptionDropped>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SubscriptionDropped_SubscriptionDropReason>>(
                    "reason",
                    SubscriptionDropped::get_reason_for_reflect,
                    SubscriptionDropped::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubscriptionDropped>(
                    "SubscriptionDropped",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubscriptionDropped {
    fn clear(&mut self) {
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscriptionDropped {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscriptionDropped {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SubscriptionDropped_SubscriptionDropReason {
    Unsubscribed = 0,
    AccessDenied = 1,
    NotFound = 2,
    PersistentSubscriptionDeleted = 3,
    SubscriberMaxCountReached = 4,
}

impl ::protobuf::ProtobufEnum for SubscriptionDropped_SubscriptionDropReason {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SubscriptionDropped_SubscriptionDropReason> {
        match value {
            0 => ::std::option::Option::Some(SubscriptionDropped_SubscriptionDropReason::Unsubscribed),
            1 => ::std::option::Option::Some(SubscriptionDropped_SubscriptionDropReason::AccessDenied),
            2 => ::std::option::Option::Some(SubscriptionDropped_SubscriptionDropReason::NotFound),
            3 => ::std::option::Option::Some(SubscriptionDropped_SubscriptionDropReason::PersistentSubscriptionDeleted),
            4 => ::std::option::Option::Some(SubscriptionDropped_SubscriptionDropReason::SubscriberMaxCountReached),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SubscriptionDropped_SubscriptionDropReason] = &[
            SubscriptionDropped_SubscriptionDropReason::Unsubscribed,
            SubscriptionDropped_SubscriptionDropReason::AccessDenied,
            SubscriptionDropped_SubscriptionDropReason::NotFound,
            SubscriptionDropped_SubscriptionDropReason::PersistentSubscriptionDeleted,
            SubscriptionDropped_SubscriptionDropReason::SubscriberMaxCountReached,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<SubscriptionDropped_SubscriptionDropReason>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SubscriptionDropped_SubscriptionDropReason", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SubscriptionDropped_SubscriptionDropReason {
}

impl ::protobuf::reflect::ProtobufValue for SubscriptionDropped_SubscriptionDropReason {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NotHandled {
    // message fields
    reason: ::std::option::Option<NotHandled_NotHandledReason>,
    additional_info: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NotHandled {}

impl NotHandled {
    pub fn new() -> NotHandled {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NotHandled {
        static mut instance: ::protobuf::lazy::Lazy<NotHandled> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NotHandled,
        };
        unsafe {
            instance.get(NotHandled::new)
        }
    }

    // required .EventStore.Client.Messages.NotHandled.NotHandledReason reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: NotHandled_NotHandledReason) {
        self.reason = ::std::option::Option::Some(v);
    }

    pub fn get_reason(&self) -> NotHandled_NotHandledReason {
        self.reason.unwrap_or(NotHandled_NotHandledReason::NotReady)
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<NotHandled_NotHandledReason> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<NotHandled_NotHandledReason> {
        &mut self.reason
    }

    // optional bytes additional_info = 2;

    pub fn clear_additional_info(&mut self) {
        self.additional_info.clear();
    }

    pub fn has_additional_info(&self) -> bool {
        self.additional_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_additional_info(&mut self, v: ::std::vec::Vec<u8>) {
        self.additional_info = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_additional_info(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.additional_info.is_none() {
            self.additional_info.set_default();
        }
        self.additional_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_additional_info(&mut self) -> ::std::vec::Vec<u8> {
        self.additional_info.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_additional_info(&self) -> &[u8] {
        match self.additional_info.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_additional_info_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.additional_info
    }

    fn mut_additional_info_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.additional_info
    }
}

impl ::protobuf::Message for NotHandled {
    fn is_initialized(&self) -> bool {
        if self.reason.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.reason, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.additional_info)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.additional_info.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reason {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.additional_info.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NotHandled {
    fn new() -> NotHandled {
        NotHandled::new()
    }

    fn descriptor_static(_: ::std::option::Option<NotHandled>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NotHandled_NotHandledReason>>(
                    "reason",
                    NotHandled::get_reason_for_reflect,
                    NotHandled::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "additional_info",
                    NotHandled::get_additional_info_for_reflect,
                    NotHandled::mut_additional_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NotHandled>(
                    "NotHandled",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NotHandled {
    fn clear(&mut self) {
        self.clear_reason();
        self.clear_additional_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NotHandled {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NotHandled {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NotHandled_MasterInfo {
    // message fields
    external_tcp_address: ::protobuf::SingularField<::std::string::String>,
    external_tcp_port: ::std::option::Option<i32>,
    external_http_address: ::protobuf::SingularField<::std::string::String>,
    external_http_port: ::std::option::Option<i32>,
    external_secure_tcp_address: ::protobuf::SingularField<::std::string::String>,
    external_secure_tcp_port: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NotHandled_MasterInfo {}

impl NotHandled_MasterInfo {
    pub fn new() -> NotHandled_MasterInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NotHandled_MasterInfo {
        static mut instance: ::protobuf::lazy::Lazy<NotHandled_MasterInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NotHandled_MasterInfo,
        };
        unsafe {
            instance.get(NotHandled_MasterInfo::new)
        }
    }

    // required string external_tcp_address = 1;

    pub fn clear_external_tcp_address(&mut self) {
        self.external_tcp_address.clear();
    }

    pub fn has_external_tcp_address(&self) -> bool {
        self.external_tcp_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_external_tcp_address(&mut self, v: ::std::string::String) {
        self.external_tcp_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_external_tcp_address(&mut self) -> &mut ::std::string::String {
        if self.external_tcp_address.is_none() {
            self.external_tcp_address.set_default();
        }
        self.external_tcp_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_external_tcp_address(&mut self) -> ::std::string::String {
        self.external_tcp_address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_external_tcp_address(&self) -> &str {
        match self.external_tcp_address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_external_tcp_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.external_tcp_address
    }

    fn mut_external_tcp_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.external_tcp_address
    }

    // required int32 external_tcp_port = 2;

    pub fn clear_external_tcp_port(&mut self) {
        self.external_tcp_port = ::std::option::Option::None;
    }

    pub fn has_external_tcp_port(&self) -> bool {
        self.external_tcp_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_external_tcp_port(&mut self, v: i32) {
        self.external_tcp_port = ::std::option::Option::Some(v);
    }

    pub fn get_external_tcp_port(&self) -> i32 {
        self.external_tcp_port.unwrap_or(0)
    }

    fn get_external_tcp_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.external_tcp_port
    }

    fn mut_external_tcp_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.external_tcp_port
    }

    // required string external_http_address = 3;

    pub fn clear_external_http_address(&mut self) {
        self.external_http_address.clear();
    }

    pub fn has_external_http_address(&self) -> bool {
        self.external_http_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_external_http_address(&mut self, v: ::std::string::String) {
        self.external_http_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_external_http_address(&mut self) -> &mut ::std::string::String {
        if self.external_http_address.is_none() {
            self.external_http_address.set_default();
        }
        self.external_http_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_external_http_address(&mut self) -> ::std::string::String {
        self.external_http_address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_external_http_address(&self) -> &str {
        match self.external_http_address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_external_http_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.external_http_address
    }

    fn mut_external_http_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.external_http_address
    }

    // required int32 external_http_port = 4;

    pub fn clear_external_http_port(&mut self) {
        self.external_http_port = ::std::option::Option::None;
    }

    pub fn has_external_http_port(&self) -> bool {
        self.external_http_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_external_http_port(&mut self, v: i32) {
        self.external_http_port = ::std::option::Option::Some(v);
    }

    pub fn get_external_http_port(&self) -> i32 {
        self.external_http_port.unwrap_or(0)
    }

    fn get_external_http_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.external_http_port
    }

    fn mut_external_http_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.external_http_port
    }

    // optional string external_secure_tcp_address = 5;

    pub fn clear_external_secure_tcp_address(&mut self) {
        self.external_secure_tcp_address.clear();
    }

    pub fn has_external_secure_tcp_address(&self) -> bool {
        self.external_secure_tcp_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_external_secure_tcp_address(&mut self, v: ::std::string::String) {
        self.external_secure_tcp_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_external_secure_tcp_address(&mut self) -> &mut ::std::string::String {
        if self.external_secure_tcp_address.is_none() {
            self.external_secure_tcp_address.set_default();
        }
        self.external_secure_tcp_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_external_secure_tcp_address(&mut self) -> ::std::string::String {
        self.external_secure_tcp_address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_external_secure_tcp_address(&self) -> &str {
        match self.external_secure_tcp_address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_external_secure_tcp_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.external_secure_tcp_address
    }

    fn mut_external_secure_tcp_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.external_secure_tcp_address
    }

    // optional int32 external_secure_tcp_port = 6;

    pub fn clear_external_secure_tcp_port(&mut self) {
        self.external_secure_tcp_port = ::std::option::Option::None;
    }

    pub fn has_external_secure_tcp_port(&self) -> bool {
        self.external_secure_tcp_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_external_secure_tcp_port(&mut self, v: i32) {
        self.external_secure_tcp_port = ::std::option::Option::Some(v);
    }

    pub fn get_external_secure_tcp_port(&self) -> i32 {
        self.external_secure_tcp_port.unwrap_or(0)
    }

    fn get_external_secure_tcp_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.external_secure_tcp_port
    }

    fn mut_external_secure_tcp_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.external_secure_tcp_port
    }
}

impl ::protobuf::Message for NotHandled_MasterInfo {
    fn is_initialized(&self) -> bool {
        if self.external_tcp_address.is_none() {
            return false;
        }
        if self.external_tcp_port.is_none() {
            return false;
        }
        if self.external_http_address.is_none() {
            return false;
        }
        if self.external_http_port.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.external_tcp_address)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.external_tcp_port = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.external_http_address)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.external_http_port = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.external_secure_tcp_address)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.external_secure_tcp_port = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.external_tcp_address.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.external_tcp_port {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.external_http_address.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.external_http_port {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.external_secure_tcp_address.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.external_secure_tcp_port {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.external_tcp_address.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.external_tcp_port {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.external_http_address.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.external_http_port {
            os.write_int32(4, v)?;
        }
        if let Some(ref v) = self.external_secure_tcp_address.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.external_secure_tcp_port {
            os.write_int32(6, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NotHandled_MasterInfo {
    fn new() -> NotHandled_MasterInfo {
        NotHandled_MasterInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<NotHandled_MasterInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "external_tcp_address",
                    NotHandled_MasterInfo::get_external_tcp_address_for_reflect,
                    NotHandled_MasterInfo::mut_external_tcp_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "external_tcp_port",
                    NotHandled_MasterInfo::get_external_tcp_port_for_reflect,
                    NotHandled_MasterInfo::mut_external_tcp_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "external_http_address",
                    NotHandled_MasterInfo::get_external_http_address_for_reflect,
                    NotHandled_MasterInfo::mut_external_http_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "external_http_port",
                    NotHandled_MasterInfo::get_external_http_port_for_reflect,
                    NotHandled_MasterInfo::mut_external_http_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "external_secure_tcp_address",
                    NotHandled_MasterInfo::get_external_secure_tcp_address_for_reflect,
                    NotHandled_MasterInfo::mut_external_secure_tcp_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "external_secure_tcp_port",
                    NotHandled_MasterInfo::get_external_secure_tcp_port_for_reflect,
                    NotHandled_MasterInfo::mut_external_secure_tcp_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NotHandled_MasterInfo>(
                    "NotHandled_MasterInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NotHandled_MasterInfo {
    fn clear(&mut self) {
        self.clear_external_tcp_address();
        self.clear_external_tcp_port();
        self.clear_external_http_address();
        self.clear_external_http_port();
        self.clear_external_secure_tcp_address();
        self.clear_external_secure_tcp_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NotHandled_MasterInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NotHandled_MasterInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NotHandled_NotHandledReason {
    NotReady = 0,
    TooBusy = 1,
    NotMaster = 2,
}

impl ::protobuf::ProtobufEnum for NotHandled_NotHandledReason {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NotHandled_NotHandledReason> {
        match value {
            0 => ::std::option::Option::Some(NotHandled_NotHandledReason::NotReady),
            1 => ::std::option::Option::Some(NotHandled_NotHandledReason::TooBusy),
            2 => ::std::option::Option::Some(NotHandled_NotHandledReason::NotMaster),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NotHandled_NotHandledReason] = &[
            NotHandled_NotHandledReason::NotReady,
            NotHandled_NotHandledReason::TooBusy,
            NotHandled_NotHandledReason::NotMaster,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NotHandled_NotHandledReason>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NotHandled_NotHandledReason", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NotHandled_NotHandledReason {
}

impl ::protobuf::reflect::ProtobufValue for NotHandled_NotHandledReason {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScavengeDatabase {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScavengeDatabase {}

impl ScavengeDatabase {
    pub fn new() -> ScavengeDatabase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScavengeDatabase {
        static mut instance: ::protobuf::lazy::Lazy<ScavengeDatabase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScavengeDatabase,
        };
        unsafe {
            instance.get(ScavengeDatabase::new)
        }
    }
}

impl ::protobuf::Message for ScavengeDatabase {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScavengeDatabase {
    fn new() -> ScavengeDatabase {
        ScavengeDatabase::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScavengeDatabase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ScavengeDatabase>(
                    "ScavengeDatabase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScavengeDatabase {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScavengeDatabase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScavengeDatabase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScavengeDatabaseCompleted {
    // message fields
    result: ::std::option::Option<ScavengeDatabaseCompleted_ScavengeResult>,
    error: ::protobuf::SingularField<::std::string::String>,
    total_time_ms: ::std::option::Option<i32>,
    total_space_saved: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScavengeDatabaseCompleted {}

impl ScavengeDatabaseCompleted {
    pub fn new() -> ScavengeDatabaseCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScavengeDatabaseCompleted {
        static mut instance: ::protobuf::lazy::Lazy<ScavengeDatabaseCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScavengeDatabaseCompleted,
        };
        unsafe {
            instance.get(ScavengeDatabaseCompleted::new)
        }
    }

    // required .EventStore.Client.Messages.ScavengeDatabaseCompleted.ScavengeResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ScavengeDatabaseCompleted_ScavengeResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ScavengeDatabaseCompleted_ScavengeResult {
        self.result.unwrap_or(ScavengeDatabaseCompleted_ScavengeResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ScavengeDatabaseCompleted_ScavengeResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ScavengeDatabaseCompleted_ScavengeResult> {
        &mut self.result
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }

    // required int32 total_time_ms = 3;

    pub fn clear_total_time_ms(&mut self) {
        self.total_time_ms = ::std::option::Option::None;
    }

    pub fn has_total_time_ms(&self) -> bool {
        self.total_time_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_time_ms(&mut self, v: i32) {
        self.total_time_ms = ::std::option::Option::Some(v);
    }

    pub fn get_total_time_ms(&self) -> i32 {
        self.total_time_ms.unwrap_or(0)
    }

    fn get_total_time_ms_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.total_time_ms
    }

    fn mut_total_time_ms_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.total_time_ms
    }

    // required int64 total_space_saved = 4;

    pub fn clear_total_space_saved(&mut self) {
        self.total_space_saved = ::std::option::Option::None;
    }

    pub fn has_total_space_saved(&self) -> bool {
        self.total_space_saved.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_space_saved(&mut self, v: i64) {
        self.total_space_saved = ::std::option::Option::Some(v);
    }

    pub fn get_total_space_saved(&self) -> i64 {
        self.total_space_saved.unwrap_or(0)
    }

    fn get_total_space_saved_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.total_space_saved
    }

    fn mut_total_space_saved_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.total_space_saved
    }
}

impl ::protobuf::Message for ScavengeDatabaseCompleted {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
            return false;
        }
        if self.total_time_ms.is_none() {
            return false;
        }
        if self.total_space_saved.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.total_time_ms = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_space_saved = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.total_time_ms {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.total_space_saved {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.total_time_ms {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.total_space_saved {
            os.write_int64(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScavengeDatabaseCompleted {
    fn new() -> ScavengeDatabaseCompleted {
        ScavengeDatabaseCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScavengeDatabaseCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ScavengeDatabaseCompleted_ScavengeResult>>(
                    "result",
                    ScavengeDatabaseCompleted::get_result_for_reflect,
                    ScavengeDatabaseCompleted::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ScavengeDatabaseCompleted::get_error_for_reflect,
                    ScavengeDatabaseCompleted::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "total_time_ms",
                    ScavengeDatabaseCompleted::get_total_time_ms_for_reflect,
                    ScavengeDatabaseCompleted::mut_total_time_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_space_saved",
                    ScavengeDatabaseCompleted::get_total_space_saved_for_reflect,
                    ScavengeDatabaseCompleted::mut_total_space_saved_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScavengeDatabaseCompleted>(
                    "ScavengeDatabaseCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScavengeDatabaseCompleted {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_error();
        self.clear_total_time_ms();
        self.clear_total_space_saved();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScavengeDatabaseCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScavengeDatabaseCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ScavengeDatabaseCompleted_ScavengeResult {
    Success = 0,
    InProgress = 1,
    Failed = 2,
}

impl ::protobuf::ProtobufEnum for ScavengeDatabaseCompleted_ScavengeResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ScavengeDatabaseCompleted_ScavengeResult> {
        match value {
            0 => ::std::option::Option::Some(ScavengeDatabaseCompleted_ScavengeResult::Success),
            1 => ::std::option::Option::Some(ScavengeDatabaseCompleted_ScavengeResult::InProgress),
            2 => ::std::option::Option::Some(ScavengeDatabaseCompleted_ScavengeResult::Failed),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ScavengeDatabaseCompleted_ScavengeResult] = &[
            ScavengeDatabaseCompleted_ScavengeResult::Success,
            ScavengeDatabaseCompleted_ScavengeResult::InProgress,
            ScavengeDatabaseCompleted_ScavengeResult::Failed,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ScavengeDatabaseCompleted_ScavengeResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ScavengeDatabaseCompleted_ScavengeResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ScavengeDatabaseCompleted_ScavengeResult {
}

impl ::protobuf::reflect::ProtobufValue for ScavengeDatabaseCompleted_ScavengeResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IdentifyClient {
    // message fields
    version: ::std::option::Option<i32>,
    connection_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IdentifyClient {}

impl IdentifyClient {
    pub fn new() -> IdentifyClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IdentifyClient {
        static mut instance: ::protobuf::lazy::Lazy<IdentifyClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IdentifyClient,
        };
        unsafe {
            instance.get(IdentifyClient::new)
        }
    }

    // required int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> i32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
    }

    // optional string connection_name = 2;

    pub fn clear_connection_name(&mut self) {
        self.connection_name.clear();
    }

    pub fn has_connection_name(&self) -> bool {
        self.connection_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_name(&mut self, v: ::std::string::String) {
        self.connection_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connection_name(&mut self) -> &mut ::std::string::String {
        if self.connection_name.is_none() {
            self.connection_name.set_default();
        }
        self.connection_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_connection_name(&mut self) -> ::std::string::String {
        self.connection_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_connection_name(&self) -> &str {
        match self.connection_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_connection_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.connection_name
    }

    fn mut_connection_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.connection_name
    }
}

impl ::protobuf::Message for IdentifyClient {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.connection_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.connection_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.connection_name.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IdentifyClient {
    fn new() -> IdentifyClient {
        IdentifyClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<IdentifyClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    IdentifyClient::get_version_for_reflect,
                    IdentifyClient::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "connection_name",
                    IdentifyClient::get_connection_name_for_reflect,
                    IdentifyClient::mut_connection_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IdentifyClient>(
                    "IdentifyClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IdentifyClient {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_connection_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IdentifyClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IdentifyClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientIdentified {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientIdentified {}

impl ClientIdentified {
    pub fn new() -> ClientIdentified {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientIdentified {
        static mut instance: ::protobuf::lazy::Lazy<ClientIdentified> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientIdentified,
        };
        unsafe {
            instance.get(ClientIdentified::new)
        }
    }
}

impl ::protobuf::Message for ClientIdentified {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientIdentified {
    fn new() -> ClientIdentified {
        ClientIdentified::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientIdentified>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ClientIdentified>(
                    "ClientIdentified",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientIdentified {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientIdentified {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientIdentified {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OperationResult {
    Success = 0,
    PrepareTimeout = 1,
    CommitTimeout = 2,
    ForwardTimeout = 3,
    WrongExpectedVersion = 4,
    StreamDeleted = 5,
    InvalidTransaction = 6,
    AccessDenied = 7,
}

impl ::protobuf::ProtobufEnum for OperationResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OperationResult> {
        match value {
            0 => ::std::option::Option::Some(OperationResult::Success),
            1 => ::std::option::Option::Some(OperationResult::PrepareTimeout),
            2 => ::std::option::Option::Some(OperationResult::CommitTimeout),
            3 => ::std::option::Option::Some(OperationResult::ForwardTimeout),
            4 => ::std::option::Option::Some(OperationResult::WrongExpectedVersion),
            5 => ::std::option::Option::Some(OperationResult::StreamDeleted),
            6 => ::std::option::Option::Some(OperationResult::InvalidTransaction),
            7 => ::std::option::Option::Some(OperationResult::AccessDenied),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OperationResult] = &[
            OperationResult::Success,
            OperationResult::PrepareTimeout,
            OperationResult::CommitTimeout,
            OperationResult::ForwardTimeout,
            OperationResult::WrongExpectedVersion,
            OperationResult::StreamDeleted,
            OperationResult::InvalidTransaction,
            OperationResult::AccessDenied,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OperationResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OperationResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OperationResult {
}

impl ::protobuf::reflect::ProtobufValue for OperationResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0emessages.proto\x12\x1aEventStore.Client.Messages\x1a\x0frustproto.\
    proto\"\xde\x01\n\x08NewEvent\x12\x19\n\x08event_id\x18\x01\x20\x02(\x0c\
    R\x07eventId\x12\x1d\n\nevent_type\x18\x02\x20\x02(\tR\teventType\x12*\n\
    \x11data_content_type\x18\x03\x20\x02(\x05R\x0fdataContentType\x122\n\
    \x15metadata_content_type\x18\x04\x20\x02(\x05R\x13metadataContentType\
    \x12\x12\n\x04data\x18\x05\x20\x02(\x0cR\x04data\x12\x1a\n\x08metadata\
    \x18\x06\x20\x01(\x0cR\x08metadata:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\
    \"\xeb\x02\n\x0bEventRecord\x12&\n\x0fevent_stream_id\x18\x01\x20\x02(\t\
    R\reventStreamId\x12!\n\x0cevent_number\x18\x02\x20\x02(\x03R\x0beventNu\
    mber\x12\x19\n\x08event_id\x18\x03\x20\x02(\x0cR\x07eventId\x12\x1d\n\ne\
    vent_type\x18\x04\x20\x02(\tR\teventType\x12*\n\x11data_content_type\x18\
    \x05\x20\x02(\x05R\x0fdataContentType\x122\n\x15metadata_content_type\
    \x18\x06\x20\x02(\x05R\x13metadataContentType\x12\x12\n\x04data\x18\x07\
    \x20\x02(\x0cR\x04data\x12\x1a\n\x08metadata\x18\x08\x20\x01(\x0cR\x08me\
    tadata\x12\x18\n\x07created\x18\t\x20\x01(\x03R\x07created\x12#\n\rcreat\
    ed_epoch\x18\n\x20\x01(\x03R\x0ccreatedEpoch:\x08\x98\xa7\x08\x01\xa0\
    \xa7\x08\x01\"\x92\x01\n\x14ResolvedIndexedEvent\x12=\n\x05event\x18\x01\
    \x20\x02(\x0b2'.EventStore.Client.Messages.EventRecordR\x05event\x12;\n\
    \x04link\x18\x02\x20\x01(\x0b2'.EventStore.Client.Messages.EventRecordR\
    \x04link\"\xdf\x01\n\rResolvedEvent\x12=\n\x05event\x18\x01\x20\x02(\x0b\
    2'.EventStore.Client.Messages.EventRecordR\x05event\x12;\n\x04link\x18\
    \x02\x20\x01(\x0b2'.EventStore.Client.Messages.EventRecordR\x04link\x12'\
    \n\x0fcommit_position\x18\x03\x20\x02(\x03R\x0ecommitPosition\x12)\n\x10\
    prepare_position\x18\x04\x20\x02(\x03R\x0fpreparePosition\"\xcf\x01\n\
    \x0bWriteEvents\x12&\n\x0fevent_stream_id\x18\x01\x20\x02(\tR\reventStre\
    amId\x12)\n\x10expected_version\x18\x02\x20\x02(\x03R\x0fexpectedVersion\
    \x12<\n\x06events\x18\x03\x20\x03(\x0b2$.EventStore.Client.Messages.NewE\
    ventR\x06events\x12%\n\x0erequire_master\x18\x04\x20\x02(\x08R\rrequireM\
    aster:\x08\x98\xa7\x08\x01\xa0\xa7\x08\x01\"\xd6\x02\n\x14WriteEventsCom\
    pleted\x12C\n\x06result\x18\x01\x20\x02(\x0e2+.EventStore.Client.Message\
    s.OperationResultR\x06result\x12\x18\n\x07message\x18\x02\x20\x01(\tR\
    \x07message\x12,\n\x12first_event_number\x18\x03\x20\x02(\x03R\x10firstE\
    ventNumber\x12*\n\x11last_event_number\x18\x04\x20\x02(\x03R\x0flastEven\
    tNumber\x12)\n\x10prepare_position\x18\x05\x20\x01(\x03R\x0fpreparePosit\
    ion\x12'\n\x0fcommit_position\x18\x06\x20\x01(\x03R\x0ecommitPosition\
    \x12'\n\x0fcurrent_version\x18\x07\x20\x01(\x03R\x0ecurrentVersion:\x08\
    \x98\xa7\x08\x01\xa0\xa7\x08\x01\"\xb3\x01\n\x0cDeleteStream\x12&\n\x0fe\
    vent_stream_id\x18\x01\x20\x02(\tR\reventStreamId\x12)\n\x10expected_ver\
    sion\x18\x02\x20\x02(\x03R\x0fexpectedVersion\x12%\n\x0erequire_master\
    \x18\x03\x20\x02(\x08R\rrequireMaster\x12\x1f\n\x0bhard_delete\x18\x04\
    \x20\x01(\x08R\nhardDelete:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\xd4\
    \x01\n\x15DeleteStreamCompleted\x12C\n\x06result\x18\x01\x20\x02(\x0e2+.\
    EventStore.Client.Messages.OperationResultR\x06result\x12\x18\n\x07messa\
    ge\x18\x02\x20\x01(\tR\x07message\x12)\n\x10prepare_position\x18\x03\x20\
    \x01(\x03R\x0fpreparePosition\x12'\n\x0fcommit_position\x18\x04\x20\x01(\
    \x03R\x0ecommitPosition:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\x96\x01\n\
    \x10TransactionStart\x12&\n\x0fevent_stream_id\x18\x01\x20\x02(\tR\reven\
    tStreamId\x12)\n\x10expected_version\x18\x02\x20\x02(\x03R\x0fexpectedVe\
    rsion\x12%\n\x0erequire_master\x18\x03\x20\x02(\x08R\rrequireMaster:\x08\
    \x98\xa7\x08\x01\xa0\xa7\x08\x01\"\xab\x01\n\x19TransactionStartComplete\
    d\x12%\n\x0etransaction_id\x18\x01\x20\x02(\x03R\rtransactionId\x12C\n\
    \x06result\x18\x02\x20\x02(\x0e2+.EventStore.Client.Messages.OperationRe\
    sultR\x06result\x12\x18\n\x07message\x18\x03\x20\x01(\tR\x07message:\x08\
    \x98\xa7\x08\x01\xa0\xa7\x08\x01\"\x9e\x01\n\x10TransactionWrite\x12%\n\
    \x0etransaction_id\x18\x01\x20\x02(\x03R\rtransactionId\x12<\n\x06events\
    \x18\x02\x20\x03(\x0b2$.EventStore.Client.Messages.NewEventR\x06events\
    \x12%\n\x0erequire_master\x18\x03\x20\x02(\x08R\rrequireMaster\"\xab\x01\
    \n\x19TransactionWriteCompleted\x12%\n\x0etransaction_id\x18\x01\x20\x02\
    (\x03R\rtransactionId\x12C\n\x06result\x18\x02\x20\x02(\x0e2+.EventStore\
    .Client.Messages.OperationResultR\x06result\x12\x18\n\x07message\x18\x03\
    \x20\x01(\tR\x07message:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"a\n\x11Tra\
    nsactionCommit\x12%\n\x0etransaction_id\x18\x01\x20\x02(\x03R\rtransacti\
    onId\x12%\n\x0erequire_master\x18\x02\x20\x02(\x08R\rrequireMaster\"\xda\
    \x02\n\x1aTransactionCommitCompleted\x12%\n\x0etransaction_id\x18\x01\
    \x20\x02(\x03R\rtransactionId\x12C\n\x06result\x18\x02\x20\x02(\x0e2+.Ev\
    entStore.Client.Messages.OperationResultR\x06result\x12\x18\n\x07message\
    \x18\x03\x20\x01(\tR\x07message\x12,\n\x12first_event_number\x18\x04\x20\
    \x02(\x03R\x10firstEventNumber\x12*\n\x11last_event_number\x18\x05\x20\
    \x02(\x03R\x0flastEventNumber\x12)\n\x10prepare_position\x18\x06\x20\x01\
    (\x03R\x0fpreparePosition\x12'\n\x0fcommit_position\x18\x07\x20\x01(\x03\
    R\x0ecommitPosition:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\xb1\x01\n\tRe\
    adEvent\x12&\n\x0fevent_stream_id\x18\x01\x20\x02(\tR\reventStreamId\x12\
    !\n\x0cevent_number\x18\x02\x20\x02(\x03R\x0beventNumber\x12(\n\x10resol\
    ve_link_tos\x18\x03\x20\x02(\x08R\x0eresolveLinkTos\x12%\n\x0erequire_ma\
    ster\x18\x04\x20\x02(\x08R\rrequireMaster:\x08\x98\xa7\x08\x01\xa0\xa7\
    \x08\x01\"\xc0\x02\n\x12ReadEventCompleted\x12V\n\x06result\x18\x01\x20\
    \x02(\x0e2>.EventStore.Client.Messages.ReadEventCompleted.ReadEventResul\
    tR\x06result\x12F\n\x05event\x18\x02\x20\x02(\x0b20.EventStore.Client.Me\
    ssages.ResolvedIndexedEventR\x05event\x12\x14\n\x05error\x18\x03\x20\x01\
    (\tR\x05error\"j\n\x0fReadEventResult\x12\x0b\n\x07Success\x10\0\x12\x0c\
    \n\x08NotFound\x10\x01\x12\x0c\n\x08NoStream\x10\x02\x12\x11\n\rStreamDe\
    leted\x10\x03\x12\t\n\x05Error\x10\x04\x12\x10\n\x0cAccessDenied\x10\x05\
    :\x08\x98\xa7\x08\x01\xa0\xa7\x08\x01\"\xde\x01\n\x10ReadStreamEvents\
    \x12&\n\x0fevent_stream_id\x18\x01\x20\x02(\tR\reventStreamId\x12*\n\x11\
    from_event_number\x18\x02\x20\x02(\x03R\x0ffromEventNumber\x12\x1b\n\tma\
    x_count\x18\x03\x20\x02(\x05R\x08maxCount\x12(\n\x10resolve_link_tos\x18\
    \x04\x20\x02(\x08R\x0eresolveLinkTos\x12%\n\x0erequire_master\x18\x05\
    \x20\x02(\x08R\rrequireMaster:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\x88\
    \x04\n\x19ReadStreamEventsCompleted\x12H\n\x06events\x18\x01\x20\x03(\
    \x0b20.EventStore.Client.Messages.ResolvedIndexedEventR\x06events\x12^\n\
    \x06result\x18\x02\x20\x02(\x0e2F.EventStore.Client.Messages.ReadStreamE\
    ventsCompleted.ReadStreamResultR\x06result\x12*\n\x11next_event_number\
    \x18\x03\x20\x02(\x03R\x0fnextEventNumber\x12*\n\x11last_event_number\
    \x18\x04\x20\x02(\x03R\x0flastEventNumber\x12'\n\x10is_end_of_stream\x18\
    \x05\x20\x02(\x08R\risEndOfStream\x120\n\x14last_commit_position\x18\x06\
    \x20\x02(\x03R\x12lastCommitPosition\x12\x14\n\x05error\x18\x07\x20\x01(\
    \tR\x05error\"n\n\x10ReadStreamResult\x12\x0b\n\x07Success\x10\0\x12\x0c\
    \n\x08NoStream\x10\x01\x12\x11\n\rStreamDeleted\x10\x02\x12\x0f\n\x0bNot\
    Modified\x10\x03\x12\t\n\x05Error\x10\x04\x12\x10\n\x0cAccessDenied\x10\
    \x05:\x08\x98\xa7\x08\x01\xa0\xa7\x08\x01\"\xd1\x01\n\rReadAllEvents\x12\
    '\n\x0fcommit_position\x18\x01\x20\x02(\x03R\x0ecommitPosition\x12)\n\
    \x10prepare_position\x18\x02\x20\x02(\x03R\x0fpreparePosition\x12\x1b\n\
    \tmax_count\x18\x03\x20\x02(\x05R\x08maxCount\x12(\n\x10resolve_link_tos\
    \x18\x04\x20\x02(\x08R\x0eresolveLinkTos\x12%\n\x0erequire_master\x18\
    \x05\x20\x02(\x08R\rrequireMaster\"\xe4\x03\n\x16ReadAllEventsCompleted\
    \x12'\n\x0fcommit_position\x18\x01\x20\x02(\x03R\x0ecommitPosition\x12)\
    \n\x10prepare_position\x18\x02\x20\x02(\x03R\x0fpreparePosition\x12A\n\
    \x06events\x18\x03\x20\x03(\x0b2).EventStore.Client.Messages.ResolvedEve\
    ntR\x06events\x120\n\x14next_commit_position\x18\x04\x20\x02(\x03R\x12ne\
    xtCommitPosition\x122\n\x15next_prepare_position\x18\x05\x20\x02(\x03R\
    \x13nextPreparePosition\x12a\n\x06result\x18\x06\x20\x01(\x0e2@.EventSto\
    re.Client.Messages.ReadAllEventsCompleted.ReadAllResult:\x07SuccessR\x06\
    result\x12\x14\n\x05error\x18\x07\x20\x01(\tR\x05error\"J\n\rReadAllResu\
    lt\x12\x0b\n\x07Success\x10\0\x12\x0f\n\x0bNotModified\x10\x01\x12\t\n\
    \x05Error\x10\x02\x12\x10\n\x0cAccessDenied\x10\x03:\x08\xa0\xa7\x08\x01\
    \x98\xa7\x08\x01\"\x8b\x06\n\x1cCreatePersistentSubscription\x126\n\x17s\
    ubscription_group_name\x18\x01\x20\x02(\tR\x15subscriptionGroupName\x12&\
    \n\x0fevent_stream_id\x18\x02\x20\x02(\tR\reventStreamId\x12(\n\x10resol\
    ve_link_tos\x18\x03\x20\x02(\x08R\x0eresolveLinkTos\x12\x1d\n\nstart_fro\
    m\x18\x04\x20\x02(\x03R\tstartFrom\x12@\n\x1cmessage_timeout_millisecond\
    s\x18\x05\x20\x02(\x05R\x1amessageTimeoutMilliseconds\x12+\n\x11record_s\
    tatistics\x18\x06\x20\x02(\x08R\x10recordStatistics\x12(\n\x10live_buffe\
    r_size\x18\x07\x20\x02(\x05R\x0eliveBufferSize\x12&\n\x0fread_batch_size\
    \x18\x08\x20\x02(\x05R\rreadBatchSize\x12\x1f\n\x0bbuffer_size\x18\t\x20\
    \x02(\x05R\nbufferSize\x12&\n\x0fmax_retry_count\x18\n\x20\x02(\x05R\rma\
    xRetryCount\x12,\n\x12prefer_round_robin\x18\x0b\x20\x02(\x08R\x10prefer\
    RoundRobin\x122\n\x15checkpoint_after_time\x18\x0c\x20\x02(\x05R\x13chec\
    kpointAfterTime\x120\n\x14checkpoint_max_count\x18\r\x20\x02(\x05R\x12ch\
    eckpointMaxCount\x120\n\x14checkpoint_min_count\x18\x0e\x20\x02(\x05R\
    \x12checkpointMinCount\x120\n\x14subscriber_max_count\x18\x0f\x20\x02(\
    \x05R\x12subscriberMaxCount\x126\n\x17named_consumer_strategy\x18\x10\
    \x20\x01(\tR\x15namedConsumerStrategy:\x08\x98\xa7\x08\x01\xa0\xa7\x08\
    \x01\"\x88\x01\n\x1cDeletePersistentSubscription\x126\n\x17subscription_\
    group_name\x18\x01\x20\x02(\tR\x15subscriptionGroupName\x12&\n\x0fevent_\
    stream_id\x18\x02\x20\x02(\tR\reventStreamId:\x08\x98\xa7\x08\x01\xa0\
    \xa7\x08\x01\"\x8b\x06\n\x1cUpdatePersistentSubscription\x126\n\x17subsc\
    ription_group_name\x18\x01\x20\x02(\tR\x15subscriptionGroupName\x12&\n\
    \x0fevent_stream_id\x18\x02\x20\x02(\tR\reventStreamId\x12(\n\x10resolve\
    _link_tos\x18\x03\x20\x02(\x08R\x0eresolveLinkTos\x12\x1d\n\nstart_from\
    \x18\x04\x20\x02(\x03R\tstartFrom\x12@\n\x1cmessage_timeout_milliseconds\
    \x18\x05\x20\x02(\x05R\x1amessageTimeoutMilliseconds\x12+\n\x11record_st\
    atistics\x18\x06\x20\x02(\x08R\x10recordStatistics\x12(\n\x10live_buffer\
    _size\x18\x07\x20\x02(\x05R\x0eliveBufferSize\x12&\n\x0fread_batch_size\
    \x18\x08\x20\x02(\x05R\rreadBatchSize\x12\x1f\n\x0bbuffer_size\x18\t\x20\
    \x02(\x05R\nbufferSize\x12&\n\x0fmax_retry_count\x18\n\x20\x02(\x05R\rma\
    xRetryCount\x12,\n\x12prefer_round_robin\x18\x0b\x20\x02(\x08R\x10prefer\
    RoundRobin\x122\n\x15checkpoint_after_time\x18\x0c\x20\x02(\x05R\x13chec\
    kpointAfterTime\x120\n\x14checkpoint_max_count\x18\r\x20\x02(\x05R\x12ch\
    eckpointMaxCount\x120\n\x14checkpoint_min_count\x18\x0e\x20\x02(\x05R\
    \x12checkpointMinCount\x120\n\x14subscriber_max_count\x18\x0f\x20\x02(\
    \x05R\x12subscriberMaxCount\x126\n\x17named_consumer_strategy\x18\x10\
    \x20\x01(\tR\x15namedConsumerStrategy:\x08\xa0\xa7\x08\x01\x98\xa7\x08\
    \x01\"\xb2\x02\n%UpdatePersistentSubscriptionCompleted\x12\x85\x01\n\x06\
    result\x18\x01\x20\x02(\x0e2d.EventStore.Client.Messages.UpdatePersisten\
    tSubscriptionCompleted.UpdatePersistentSubscriptionResult:\x07SuccessR\
    \x06result\x12\x16\n\x06reason\x18\x02\x20\x01(\tR\x06reason\"_\n\"Updat\
    ePersistentSubscriptionResult\x12\x0b\n\x07Success\x10\0\x12\x10\n\x0cDo\
    esNotExist\x10\x01\x12\x08\n\x04Fail\x10\x02\x12\x10\n\x0cAccessDenied\
    \x10\x03:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\xb3\x02\n%CreatePersiste\
    ntSubscriptionCompleted\x12\x85\x01\n\x06result\x18\x01\x20\x02(\x0e2d.E\
    ventStore.Client.Messages.CreatePersistentSubscriptionCompleted.CreatePe\
    rsistentSubscriptionResult:\x07SuccessR\x06result\x12\x16\n\x06reason\
    \x18\x02\x20\x01(\tR\x06reason\"`\n\"CreatePersistentSubscriptionResult\
    \x12\x0b\n\x07Success\x10\0\x12\x11\n\rAlreadyExists\x10\x01\x12\x08\n\
    \x04Fail\x10\x02\x12\x10\n\x0cAccessDenied\x10\x03:\x08\xa0\xa7\x08\x01\
    \x98\xa7\x08\x01\"\xb2\x02\n%DeletePersistentSubscriptionCompleted\x12\
    \x85\x01\n\x06result\x18\x01\x20\x02(\x0e2d.EventStore.Client.Messages.D\
    eletePersistentSubscriptionCompleted.DeletePersistentSubscriptionResult:\
    \x07SuccessR\x06result\x12\x16\n\x06reason\x18\x02\x20\x01(\tR\x06reason\
    \"_\n\"DeletePersistentSubscriptionResult\x12\x0b\n\x07Success\x10\0\x12\
    \x10\n\x0cDoesNotExist\x10\x01\x12\x08\n\x04Fail\x10\x02\x12\x10\n\x0cAc\
    cessDenied\x10\x03:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\xb9\x01\n\x1fC\
    onnectToPersistentSubscription\x12'\n\x0fsubscription_id\x18\x01\x20\x02\
    (\tR\x0esubscriptionId\x12&\n\x0fevent_stream_id\x18\x02\x20\x02(\tR\rev\
    entStreamId\x12;\n\x1aallowed_in_flight_messages\x18\x03\x20\x02(\x05R\
    \x17allowedInFlightMessages:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\x84\
    \x01\n\x1fPersistentSubscriptionAckEvents\x12'\n\x0fsubscription_id\x18\
    \x01\x20\x02(\tR\x0esubscriptionId\x12.\n\x13processed_event_ids\x18\x02\
    \x20\x03(\x0cR\x11processedEventIds:\x08\x98\xa7\x08\x01\xa0\xa7\x08\x01\
    \"\xc9\x02\n\x1fPersistentSubscriptionNakEvents\x12'\n\x0fsubscription_i\
    d\x18\x01\x20\x02(\tR\x0esubscriptionId\x12.\n\x13processed_event_ids\
    \x18\x02\x20\x03(\x0cR\x11processedEventIds\x12\x18\n\x07message\x18\x03\
    \x20\x01(\tR\x07message\x12f\n\x06action\x18\x04\x20\x02(\x0e2E.EventSto\
    re.Client.Messages.PersistentSubscriptionNakEvents.NakAction:\x07Unknown\
    R\x06action\"A\n\tNakAction\x12\x0b\n\x07Unknown\x10\0\x12\x08\n\x04Park\
    \x10\x01\x12\t\n\x05Retry\x10\x02\x12\x08\n\x04Skip\x10\x03\x12\x08\n\
    \x04Stop\x10\x04:\x08\xa0\xa7\x08\x01\x98\xa7\x08\x01\"\xb5\x01\n\"Persi\
    stentSubscriptionConfirmation\x120\n\x14last_commit_position\x18\x01\x20\
    \x02(\x03R\x12lastCommitPosition\x12'\n\x0fsubscription_id\x18\x02\x20\
    \x02(\tR\x0esubscriptionId\x12*\n\x11last_event_number\x18\x03\x20\x01(\
    \x03R\x0flastEventNumber:\x08\x98\xa7\x08\x01\xa0\xa7\x08\x01\"\x93\x01\
    \n)PersistentSubscriptionStreamEventAppeared\x12F\n\x05event\x18\x01\x20\
    \x02(\x0b20.EventStore.Client.Messages.ResolvedIndexedEventR\x05event\
    \x12\x1e\n\nretryCount\x18\x02\x20\x01(\x05R\nretryCount\"o\n\x11Subscri\
    beToStream\x12&\n\x0fevent_stream_id\x18\x01\x20\x02(\tR\reventStreamId\
    \x12(\n\x10resolve_link_tos\x18\x02\x20\x02(\x08R\x0eresolveLinkTos:\x08\
    \xa0\xa7\x08\x01\x98\xa7\x08\x01\"x\n\x18SubscriptionConfirmation\x120\n\
    \x14last_commit_position\x18\x01\x20\x02(\x03R\x12lastCommitPosition\x12\
    *\n\x11last_event_number\x18\x02\x20\x01(\x03R\x0flastEventNumber\"V\n\
    \x13StreamEventAppeared\x12?\n\x05event\x18\x01\x20\x02(\x0b2).EventStor\
    e.Client.Messages.ResolvedEventR\x05event\"\x17\n\x15UnsubscribeFromStre\
    am\"\x92\x02\n\x13SubscriptionDropped\x12l\n\x06reason\x18\x01\x20\x01(\
    \x0e2F.EventStore.Client.Messages.SubscriptionDropped.SubscriptionDropRe\
    ason:\x0cUnsubscribedR\x06reason\"\x8c\x01\n\x16SubscriptionDropReason\
    \x12\x10\n\x0cUnsubscribed\x10\0\x12\x10\n\x0cAccessDenied\x10\x01\x12\
    \x0c\n\x08NotFound\x10\x02\x12!\n\x1dPersistentSubscriptionDeleted\x10\
    \x03\x12\x1d\n\x19SubscriberMaxCountReached\x10\x04\"\x8b\x04\n\nNotHand\
    led\x12O\n\x06reason\x18\x01\x20\x02(\x0e27.EventStore.Client.Messages.N\
    otHandled.NotHandledReasonR\x06reason\x12'\n\x0fadditional_info\x18\x02\
    \x20\x01(\x0cR\x0eadditionalInfo\x1a\xc4\x02\n\nMasterInfo\x120\n\x14ext\
    ernal_tcp_address\x18\x01\x20\x02(\tR\x12externalTcpAddress\x12*\n\x11ex\
    ternal_tcp_port\x18\x02\x20\x02(\x05R\x0fexternalTcpPort\x122\n\x15exter\
    nal_http_address\x18\x03\x20\x02(\tR\x13externalHttpAddress\x12,\n\x12ex\
    ternal_http_port\x18\x04\x20\x02(\x05R\x10externalHttpPort\x12=\n\x1bext\
    ernal_secure_tcp_address\x18\x05\x20\x01(\tR\x18externalSecureTcpAddress\
    \x127\n\x18external_secure_tcp_port\x18\x06\x20\x01(\x05R\x15externalSec\
    ureTcpPort\"<\n\x10NotHandledReason\x12\x0c\n\x08NotReady\x10\0\x12\x0b\
    \n\x07TooBusy\x10\x01\x12\r\n\tNotMaster\x10\x02\"\x12\n\x10ScavengeData\
    base\"\x9a\x02\n\x19ScavengeDatabaseCompleted\x12\\\n\x06result\x18\x01\
    \x20\x02(\x0e2D.EventStore.Client.Messages.ScavengeDatabaseCompleted.Sca\
    vengeResultR\x06result\x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05error\
    \x12\"\n\rtotal_time_ms\x18\x03\x20\x02(\x05R\x0btotalTimeMs\x12*\n\x11t\
    otal_space_saved\x18\x04\x20\x02(\x03R\x0ftotalSpaceSaved\"9\n\x0eScaven\
    geResult\x12\x0b\n\x07Success\x10\0\x12\x0e\n\nInProgress\x10\x01\x12\n\
    \n\x06Failed\x10\x02\"S\n\x0eIdentifyClient\x12\x18\n\x07version\x18\x01\
    \x20\x02(\x05R\x07version\x12'\n\x0fconnection_name\x18\x02\x20\x01(\tR\
    \x0econnectionName\"\x12\n\x10ClientIdentified*\xb0\x01\n\x0fOperationRe\
    sult\x12\x0b\n\x07Success\x10\0\x12\x12\n\x0ePrepareTimeout\x10\x01\x12\
    \x11\n\rCommitTimeout\x10\x02\x12\x12\n\x0eForwardTimeout\x10\x03\x12\
    \x18\n\x14WrongExpectedVersion\x10\x04\x12\x11\n\rStreamDeleted\x10\x05\
    \x12\x16\n\x12InvalidTransaction\x10\x06\x12\x10\n\x0cAccessDenied\x10\
    \x07\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
