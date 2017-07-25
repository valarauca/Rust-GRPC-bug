// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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
pub struct LogMessage {
    // message fields
    pub loglevel: LogLevel,
    pub timestamp: ::std::string::String,
    pub logline: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogMessage {}

impl LogMessage {
    pub fn new() -> LogMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogMessage {
        static mut instance: ::protobuf::lazy::Lazy<LogMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogMessage,
        };
        unsafe {
            instance.get(LogMessage::new)
        }
    }

    // .gateway.LogLevel loglevel = 1;

    pub fn clear_loglevel(&mut self) {
        self.loglevel = LogLevel::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_loglevel(&mut self, v: LogLevel) {
        self.loglevel = v;
    }

    pub fn get_loglevel(&self) -> LogLevel {
        self.loglevel
    }

    fn get_loglevel_for_reflect(&self) -> &LogLevel {
        &self.loglevel
    }

    fn mut_loglevel_for_reflect(&mut self) -> &mut LogLevel {
        &mut self.loglevel
    }

    // string timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: ::std::string::String) {
        self.timestamp = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestamp(&mut self) -> &mut ::std::string::String {
        &mut self.timestamp
    }

    // Take field
    pub fn take_timestamp(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.timestamp, ::std::string::String::new())
    }

    pub fn get_timestamp(&self) -> &str {
        &self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &::std::string::String {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.timestamp
    }

    // string logline = 3;

    pub fn clear_logline(&mut self) {
        self.logline.clear();
    }

    // Param is passed by value, moved
    pub fn set_logline(&mut self, v: ::std::string::String) {
        self.logline = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logline(&mut self) -> &mut ::std::string::String {
        &mut self.logline
    }

    // Take field
    pub fn take_logline(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.logline, ::std::string::String::new())
    }

    pub fn get_logline(&self) -> &str {
        &self.logline
    }

    fn get_logline_for_reflect(&self) -> &::std::string::String {
        &self.logline
    }

    fn mut_logline_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.logline
    }
}

impl ::protobuf::Message for LogMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.loglevel = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.timestamp)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.logline)?;
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
        if self.loglevel != LogLevel::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.loglevel);
        };
        if !self.timestamp.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.timestamp);
        };
        if !self.logline.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.logline);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.loglevel != LogLevel::UNKNOWN {
            os.write_enum(1, self.loglevel.value())?;
        };
        if !self.timestamp.is_empty() {
            os.write_string(2, &self.timestamp)?;
        };
        if !self.logline.is_empty() {
            os.write_string(3, &self.logline)?;
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

impl ::protobuf::MessageStatic for LogMessage {
    fn new() -> LogMessage {
        LogMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<LogLevel>>(
                    "loglevel",
                    LogMessage::get_loglevel_for_reflect,
                    LogMessage::mut_loglevel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "timestamp",
                    LogMessage::get_timestamp_for_reflect,
                    LogMessage::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "logline",
                    LogMessage::get_logline_for_reflect,
                    LogMessage::mut_logline_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogMessage>(
                    "LogMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogMessage {
    fn clear(&mut self) {
        self.clear_loglevel();
        self.clear_timestamp();
        self.clear_logline();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetLogLevel {
    // message fields
    pub level: LogLevel,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetLogLevel {}

impl SetLogLevel {
    pub fn new() -> SetLogLevel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetLogLevel {
        static mut instance: ::protobuf::lazy::Lazy<SetLogLevel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetLogLevel,
        };
        unsafe {
            instance.get(SetLogLevel::new)
        }
    }

    // .gateway.LogLevel level = 1;

    pub fn clear_level(&mut self) {
        self.level = LogLevel::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: LogLevel) {
        self.level = v;
    }

    pub fn get_level(&self) -> LogLevel {
        self.level
    }

    fn get_level_for_reflect(&self) -> &LogLevel {
        &self.level
    }

    fn mut_level_for_reflect(&mut self) -> &mut LogLevel {
        &mut self.level
    }
}

impl ::protobuf::Message for SetLogLevel {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.level = tmp;
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
        if self.level != LogLevel::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.level);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.level != LogLevel::UNKNOWN {
            os.write_enum(1, self.level.value())?;
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

impl ::protobuf::MessageStatic for SetLogLevel {
    fn new() -> SetLogLevel {
        SetLogLevel::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetLogLevel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<LogLevel>>(
                    "level",
                    SetLogLevel::get_level_for_reflect,
                    SetLogLevel::mut_level_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetLogLevel>(
                    "SetLogLevel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetLogLevel {
    fn clear(&mut self) {
        self.clear_level();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetLogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetLogLevel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum LogLevel {
    UNKNOWN = 0,
    CRIT = 1,
    ERROR = 2,
    WARN = 3,
    NOTE = 4,
    INFO = 5,
    DEBUG = 6,
}

impl ::protobuf::ProtobufEnum for LogLevel {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LogLevel> {
        match value {
            0 => ::std::option::Option::Some(LogLevel::UNKNOWN),
            1 => ::std::option::Option::Some(LogLevel::CRIT),
            2 => ::std::option::Option::Some(LogLevel::ERROR),
            3 => ::std::option::Option::Some(LogLevel::WARN),
            4 => ::std::option::Option::Some(LogLevel::NOTE),
            5 => ::std::option::Option::Some(LogLevel::INFO),
            6 => ::std::option::Option::Some(LogLevel::DEBUG),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [LogLevel] = &[
            LogLevel::UNKNOWN,
            LogLevel::CRIT,
            LogLevel::ERROR,
            LogLevel::WARN,
            LogLevel::NOTE,
            LogLevel::INFO,
            LogLevel::DEBUG,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<LogLevel>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("LogLevel", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for LogLevel {
}

impl ::std::default::Default for LogLevel {
    fn default() -> Self {
        LogLevel::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for LogLevel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x07, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x22, 0x73, 0x0a, 0x0a, 0x4c, 0x6f, 0x67, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2d, 0x0a, 0x08, 0x6c, 0x6f, 0x67, 0x6c, 0x65, 0x76,
    0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x74, 0x65, 0x77,
    0x61, 0x79, 0x2e, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x52, 0x08, 0x6c, 0x6f, 0x67,
    0x6c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x12, 0x18, 0x0a, 0x07, 0x6c, 0x6f, 0x67, 0x6c, 0x69, 0x6e, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6c, 0x6f, 0x67, 0x6c, 0x69, 0x6e, 0x65, 0x22, 0x36, 0x0a,
    0x0b, 0x53, 0x65, 0x74, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x27, 0x0a, 0x05,
    0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x67, 0x61,
    0x74, 0x65, 0x77, 0x61, 0x79, 0x2e, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x52, 0x05,
    0x6c, 0x65, 0x76, 0x65, 0x6c, 0x2a, 0x55, 0x0a, 0x08, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65,
    0x6c, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x08,
    0x0a, 0x04, 0x43, 0x52, 0x49, 0x54, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x57, 0x41, 0x52, 0x4e, 0x10, 0x03, 0x12, 0x08, 0x0a,
    0x04, 0x4e, 0x4f, 0x54, 0x45, 0x10, 0x04, 0x12, 0x08, 0x0a, 0x04, 0x49, 0x4e, 0x46, 0x4f, 0x10,
    0x05, 0x12, 0x09, 0x0a, 0x05, 0x44, 0x45, 0x42, 0x55, 0x47, 0x10, 0x06, 0x32, 0x47, 0x0a, 0x07,
    0x47, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x12, 0x3c, 0x0a, 0x0d, 0x52, 0x65, 0x6d, 0x6f, 0x74,
    0x65, 0x4c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x12, 0x14, 0x2e, 0x67, 0x61, 0x74, 0x65, 0x77,
    0x61, 0x79, 0x2e, 0x53, 0x65, 0x74, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x1a, 0x13,
    0x2e, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x2e, 0x4c, 0x6f, 0x67, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x30, 0x01, 0x42, 0x25, 0x0a, 0x1a, 0x69, 0x6f, 0x2e, 0x73, 0x68, 0x69, 0x66,
    0x74, 0x6c, 0x65, 0x66, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x67, 0x61, 0x74, 0x65,
    0x77, 0x61, 0x79, 0x42, 0x07, 0x47, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x4a, 0xd7, 0x07, 0x0a,
    0x06, 0x12, 0x04, 0x01, 0x00, 0x20, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x03, 0x08, 0x0f, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x05, 0x00, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x05,
    0x00, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x05, 0x07, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x07, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x05, 0x16, 0x32, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12,
    0x03, 0x06, 0x00, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x06,
    0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07,
    0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x07,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x06, 0x1e, 0x27, 0x0a,
    0x2c, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x11, 0x01, 0x1a, 0x20, 0x20, 0x45, 0x6e,
    0x75, 0x6d, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x69, 0x6e,
    0x67, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x09, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0a, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0a, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a,
    0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x0b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x06, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x09, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x0c, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x02,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x06, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x09, 0x0a, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x0e, 0x09, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x0f, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x02,
    0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0f, 0x09, 0x0a, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x06, 0x02, 0x12, 0x03, 0x10, 0x0a, 0x0b, 0x0a, 0x23, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x14, 0x00, 0x18, 0x01, 0x1a, 0x17, 0x2f, 0x20, 0x41, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65,
    0x20, 0x6c, 0x6f, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x15, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x15, 0x02, 0x14, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x16, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x16, 0x02, 0x15, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x16, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x16, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x17,
    0x02, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x17, 0x02, 0x16,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x09, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x1a, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x1a, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1b, 0x02, 0x1a, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12,
    0x04, 0x1e, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x3e, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x06, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1f, 0x15, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x1f, 0x2b, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1f, 0x32, 0x3c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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