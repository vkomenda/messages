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
pub struct Message {
    // message fields
    pub message_type: Message_MessageType,
    pub root_hash: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // .message_stream_example.protocol.Message.MessageType message_type = 1;

    pub fn clear_message_type(&mut self) {
        self.message_type = Message_MessageType::VALUE;
    }

    // Param is passed by value, moved
    pub fn set_message_type(&mut self, v: Message_MessageType) {
        self.message_type = v;
    }

    pub fn get_message_type(&self) -> Message_MessageType {
        self.message_type
    }

    fn get_message_type_for_reflect(&self) -> &Message_MessageType {
        &self.message_type
    }

    fn mut_message_type_for_reflect(&mut self) -> &mut Message_MessageType {
        &mut self.message_type
    }

    // bytes root_hash = 2;

    pub fn clear_root_hash(&mut self) {
        self.root_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_root_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.root_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.root_hash
    }

    // Take field
    pub fn take_root_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.root_hash, ::std::vec::Vec::new())
    }

    pub fn get_root_hash(&self) -> &[u8] {
        &self.root_hash
    }

    fn get_root_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.root_hash
    }

    fn mut_root_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.root_hash
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for Message {
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
                    }
                    let tmp = is.read_enum()?;
                    self.message_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.root_hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
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
        if self.message_type != Message_MessageType::VALUE {
            my_size += ::protobuf::rt::enum_size(1, self.message_type);
        }
        if !self.root_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.root_hash);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.message_type != Message_MessageType::VALUE {
            os.write_enum(1, self.message_type.value())?;
        }
        if !self.root_hash.is_empty() {
            os.write_bytes(2, &self.root_hash)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Message_MessageType>>(
                    "message_type",
                    Message::get_message_type_for_reflect,
                    Message::mut_message_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "root_hash",
                    Message::get_root_hash_for_reflect,
                    Message::mut_root_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Message::get_value_for_reflect,
                    Message::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_message_type();
        self.clear_root_hash();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_MessageType {
    VALUE = 0,
    ECHO = 1,
    READY = 2,
}

impl ::protobuf::ProtobufEnum for Message_MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_MessageType> {
        match value {
            0 => ::std::option::Option::Some(Message_MessageType::VALUE),
            1 => ::std::option::Option::Some(Message_MessageType::ECHO),
            2 => ::std::option::Option::Some(Message_MessageType::READY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Message_MessageType] = &[
            Message_MessageType::VALUE,
            Message_MessageType::ECHO,
            Message_MessageType::READY,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Message_MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_MessageType {
}

impl ::std::default::Default for Message_MessageType {
    fn default() -> Self {
        Message_MessageType::VALUE
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\x12\x1fmessage_stream_example.protocol\"\xc4\x01\n\x07\
    Message\x12W\n\x0cmessage_type\x18\x01\x20\x01(\x0e24.message_stream_exa\
    mple.protocol.Message.MessageTypeR\x0bmessageType\x12\x1b\n\troot_hash\
    \x18\x02\x20\x01(\x0cR\x08rootHash\x12\x14\n\x05value\x18\x03\x20\x01(\
    \x0cR\x05value\"-\n\x0bMessageType\x12\t\n\x05VALUE\x10\0\x12\x08\n\x04E\
    CHO\x10\x01\x12\t\n\x05READY\x10\x02b\x06proto3\
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
