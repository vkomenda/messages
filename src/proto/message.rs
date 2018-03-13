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
    pub list: ::protobuf::SingularPtrField<List>,
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

    // .Message.MessageType message_type = 1;

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

    // .List list = 4;

    pub fn clear_list(&mut self) {
        self.list.clear();
    }

    pub fn has_list(&self) -> bool {
        self.list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list(&mut self, v: List) {
        self.list = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list(&mut self) -> &mut List {
        if self.list.is_none() {
            self.list.set_default();
        }
        self.list.as_mut().unwrap()
    }

    // Take field
    pub fn take_list(&mut self) -> List {
        self.list.take().unwrap_or_else(|| List::new())
    }

    pub fn get_list(&self) -> &List {
        self.list.as_ref().unwrap_or_else(|| List::default_instance())
    }

    fn get_list_for_reflect(&self) -> &::protobuf::SingularPtrField<List> {
        &self.list
    }

    fn mut_list_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<List> {
        &mut self.list
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        for v in &self.list {
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
                    let tmp = is.read_enum()?;
                    self.message_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.root_hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list)?;
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
        if let Some(ref v) = self.list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(ref v) = self.list.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<List>>(
                    "list",
                    Message::get_list_for_reflect,
                    Message::mut_list_for_reflect,
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
        self.clear_list();
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

#[derive(PartialEq,Clone,Default)]
pub struct List {
    // message oneof groups
    list: ::std::option::Option<List_oneof_list>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for List {}

#[derive(Clone,PartialEq)]
pub enum List_oneof_list {
    nil(ListNil),
    cons(ListCons),
}

impl List {
    pub fn new() -> List {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static List {
        static mut instance: ::protobuf::lazy::Lazy<List> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const List,
        };
        unsafe {
            instance.get(List::new)
        }
    }

    // .ListNil nil = 1;

    pub fn clear_nil(&mut self) {
        self.list = ::std::option::Option::None;
    }

    pub fn has_nil(&self) -> bool {
        match self.list {
            ::std::option::Option::Some(List_oneof_list::nil(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_nil(&mut self, v: ListNil) {
        self.list = ::std::option::Option::Some(List_oneof_list::nil(v))
    }

    // Mutable pointer to the field.
    pub fn mut_nil(&mut self) -> &mut ListNil {
        if let ::std::option::Option::Some(List_oneof_list::nil(_)) = self.list {
        } else {
            self.list = ::std::option::Option::Some(List_oneof_list::nil(ListNil::new()));
        }
        match self.list {
            ::std::option::Option::Some(List_oneof_list::nil(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_nil(&mut self) -> ListNil {
        if self.has_nil() {
            match self.list.take() {
                ::std::option::Option::Some(List_oneof_list::nil(v)) => v,
                _ => panic!(),
            }
        } else {
            ListNil::new()
        }
    }

    pub fn get_nil(&self) -> &ListNil {
        match self.list {
            ::std::option::Option::Some(List_oneof_list::nil(ref v)) => v,
            _ => ListNil::default_instance(),
        }
    }

    // .ListCons cons = 2;

    pub fn clear_cons(&mut self) {
        self.list = ::std::option::Option::None;
    }

    pub fn has_cons(&self) -> bool {
        match self.list {
            ::std::option::Option::Some(List_oneof_list::cons(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cons(&mut self, v: ListCons) {
        self.list = ::std::option::Option::Some(List_oneof_list::cons(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cons(&mut self) -> &mut ListCons {
        if let ::std::option::Option::Some(List_oneof_list::cons(_)) = self.list {
        } else {
            self.list = ::std::option::Option::Some(List_oneof_list::cons(ListCons::new()));
        }
        match self.list {
            ::std::option::Option::Some(List_oneof_list::cons(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cons(&mut self) -> ListCons {
        if self.has_cons() {
            match self.list.take() {
                ::std::option::Option::Some(List_oneof_list::cons(v)) => v,
                _ => panic!(),
            }
        } else {
            ListCons::new()
        }
    }

    pub fn get_cons(&self) -> &ListCons {
        match self.list {
            ::std::option::Option::Some(List_oneof_list::cons(ref v)) => v,
            _ => ListCons::default_instance(),
        }
    }
}

impl ::protobuf::Message for List {
    fn is_initialized(&self) -> bool {
        if let Some(List_oneof_list::nil(ref v)) = self.list {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(List_oneof_list::cons(ref v)) = self.list {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.list = ::std::option::Option::Some(List_oneof_list::nil(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.list = ::std::option::Option::Some(List_oneof_list::cons(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.list {
            match v {
                &List_oneof_list::nil(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &List_oneof_list::cons(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.list {
            match v {
                &List_oneof_list::nil(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &List_oneof_list::cons(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for List {
    fn new() -> List {
        List::new()
    }

    fn descriptor_static(_: ::std::option::Option<List>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ListNil>(
                    "nil",
                    List::has_nil,
                    List::get_nil,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ListCons>(
                    "cons",
                    List::has_cons,
                    List::get_cons,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<List>(
                    "List",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for List {
    fn clear(&mut self) {
        self.clear_nil();
        self.clear_cons();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for List {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for List {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListCons {
    // message fields
    pub head: u32,
    pub tail: ::protobuf::SingularPtrField<List>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListCons {}

impl ListCons {
    pub fn new() -> ListCons {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListCons {
        static mut instance: ::protobuf::lazy::Lazy<ListCons> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListCons,
        };
        unsafe {
            instance.get(ListCons::new)
        }
    }

    // uint32 head = 1;

    pub fn clear_head(&mut self) {
        self.head = 0;
    }

    // Param is passed by value, moved
    pub fn set_head(&mut self, v: u32) {
        self.head = v;
    }

    pub fn get_head(&self) -> u32 {
        self.head
    }

    fn get_head_for_reflect(&self) -> &u32 {
        &self.head
    }

    fn mut_head_for_reflect(&mut self) -> &mut u32 {
        &mut self.head
    }

    // .List tail = 2;

    pub fn clear_tail(&mut self) {
        self.tail.clear();
    }

    pub fn has_tail(&self) -> bool {
        self.tail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tail(&mut self, v: List) {
        self.tail = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tail(&mut self) -> &mut List {
        if self.tail.is_none() {
            self.tail.set_default();
        }
        self.tail.as_mut().unwrap()
    }

    // Take field
    pub fn take_tail(&mut self) -> List {
        self.tail.take().unwrap_or_else(|| List::new())
    }

    pub fn get_tail(&self) -> &List {
        self.tail.as_ref().unwrap_or_else(|| List::default_instance())
    }

    fn get_tail_for_reflect(&self) -> &::protobuf::SingularPtrField<List> {
        &self.tail
    }

    fn mut_tail_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<List> {
        &mut self.tail
    }
}

impl ::protobuf::Message for ListCons {
    fn is_initialized(&self) -> bool {
        for v in &self.tail {
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
                    let tmp = is.read_uint32()?;
                    self.head = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tail)?;
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
        if self.head != 0 {
            my_size += ::protobuf::rt::value_size(1, self.head, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.tail.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.head != 0 {
            os.write_uint32(1, self.head)?;
        }
        if let Some(ref v) = self.tail.as_ref() {
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

impl ::protobuf::MessageStatic for ListCons {
    fn new() -> ListCons {
        ListCons::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListCons>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "head",
                    ListCons::get_head_for_reflect,
                    ListCons::mut_head_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<List>>(
                    "tail",
                    ListCons::get_tail_for_reflect,
                    ListCons::mut_tail_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListCons>(
                    "ListCons",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListCons {
    fn clear(&mut self) {
        self.clear_head();
        self.clear_tail();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListCons {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListCons {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListNil {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListNil {}

impl ListNil {
    pub fn new() -> ListNil {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListNil {
        static mut instance: ::protobuf::lazy::Lazy<ListNil> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListNil,
        };
        unsafe {
            instance.get(ListNil::new)
        }
    }
}

impl ::protobuf::Message for ListNil {
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

impl ::protobuf::MessageStatic for ListNil {
    fn new() -> ListNil {
        ListNil::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListNil>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListNil>(
                    "ListNil",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListNil {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListNil {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListNil {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\"\xbf\x01\n\x07Message\x127\n\x0cmessage_type\x18\x01\
    \x20\x01(\x0e2\x14.Message.MessageTypeR\x0bmessageType\x12\x1b\n\troot_h\
    ash\x18\x02\x20\x01(\x0cR\x08rootHash\x12\x14\n\x05value\x18\x03\x20\x01\
    (\x0cR\x05value\x12\x19\n\x04list\x18\x04\x20\x01(\x0b2\x05.ListR\x04lis\
    t\"-\n\x0bMessageType\x12\t\n\x05VALUE\x10\0\x12\x08\n\x04ECHO\x10\x01\
    \x12\t\n\x05READY\x10\x02\"M\n\x04List\x12\x1c\n\x03nil\x18\x01\x20\x01(\
    \x0b2\x08.ListNilH\0R\x03nil\x12\x1f\n\x04cons\x18\x02\x20\x01(\x0b2\t.L\
    istConsH\0R\x04consB\x06\n\x04list\"9\n\x08ListCons\x12\x12\n\x04head\
    \x18\x01\x20\x01(\rR\x04head\x12\x19\n\x04tail\x18\x02\x20\x01(\x0b2\x05\
    .ListR\x04tail\"\t\n\x07ListNilb\x06proto3\
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
