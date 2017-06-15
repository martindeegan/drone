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
pub struct ControllerInput {
    // message fields
    pub id: i32,
    pub time: i32,
    orientation: ::protobuf::SingularPtrField<super::vector3::Vector3>,
    pub vertical_velocity: f32,
    pub yaw_velocity: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ControllerInput {}

impl ControllerInput {
    pub fn new() -> ControllerInput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ControllerInput {
        static mut instance: ::protobuf::lazy::Lazy<ControllerInput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ControllerInput,
        };
        unsafe {
            instance.get(ControllerInput::new)
        }
    }

    // int32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // int32 time = 2;

    pub fn clear_time(&mut self) {
        self.time = 0;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i32) {
        self.time = v;
    }

    pub fn get_time(&self) -> i32 {
        self.time
    }

    fn get_time_for_reflect(&self) -> &i32 {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut i32 {
        &mut self.time
    }

    // .Vector3 orientation = 3;

    pub fn clear_orientation(&mut self) {
        self.orientation.clear();
    }

    pub fn has_orientation(&self) -> bool {
        self.orientation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_orientation(&mut self, v: super::vector3::Vector3) {
        self.orientation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_orientation(&mut self) -> &mut super::vector3::Vector3 {
        if self.orientation.is_none() {
            self.orientation.set_default();
        };
        self.orientation.as_mut().unwrap()
    }

    // Take field
    pub fn take_orientation(&mut self) -> super::vector3::Vector3 {
        self.orientation.take().unwrap_or_else(|| super::vector3::Vector3::new())
    }

    pub fn get_orientation(&self) -> &super::vector3::Vector3 {
        self.orientation.as_ref().unwrap_or_else(|| super::vector3::Vector3::default_instance())
    }

    fn get_orientation_for_reflect(&self) -> &::protobuf::SingularPtrField<super::vector3::Vector3> {
        &self.orientation
    }

    fn mut_orientation_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::vector3::Vector3> {
        &mut self.orientation
    }

    // float vertical_velocity = 4;

    pub fn clear_vertical_velocity(&mut self) {
        self.vertical_velocity = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vertical_velocity(&mut self, v: f32) {
        self.vertical_velocity = v;
    }

    pub fn get_vertical_velocity(&self) -> f32 {
        self.vertical_velocity
    }

    fn get_vertical_velocity_for_reflect(&self) -> &f32 {
        &self.vertical_velocity
    }

    fn mut_vertical_velocity_for_reflect(&mut self) -> &mut f32 {
        &mut self.vertical_velocity
    }

    // float yaw_velocity = 5;

    pub fn clear_yaw_velocity(&mut self) {
        self.yaw_velocity = 0.;
    }

    // Param is passed by value, moved
    pub fn set_yaw_velocity(&mut self, v: f32) {
        self.yaw_velocity = v;
    }

    pub fn get_yaw_velocity(&self) -> f32 {
        self.yaw_velocity
    }

    fn get_yaw_velocity_for_reflect(&self) -> &f32 {
        &self.yaw_velocity
    }

    fn mut_yaw_velocity_for_reflect(&mut self) -> &mut f32 {
        &mut self.yaw_velocity
    }
}

impl ::protobuf::Message for ControllerInput {
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
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.time = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.orientation)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.vertical_velocity = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.yaw_velocity = tmp;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.time != 0 {
            my_size += ::protobuf::rt::value_size(2, self.time, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.orientation.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.vertical_velocity != 0. {
            my_size += 5;
        };
        if self.yaw_velocity != 0. {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        };
        if self.time != 0 {
            os.write_int32(2, self.time)?;
        };
        if let Some(v) = self.orientation.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.vertical_velocity != 0. {
            os.write_float(4, self.vertical_velocity)?;
        };
        if self.yaw_velocity != 0. {
            os.write_float(5, self.yaw_velocity)?;
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

impl ::protobuf::MessageStatic for ControllerInput {
    fn new() -> ControllerInput {
        ControllerInput::new()
    }

    fn descriptor_static(_: ::std::option::Option<ControllerInput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    ControllerInput::get_id_for_reflect,
                    ControllerInput::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "time",
                    ControllerInput::get_time_for_reflect,
                    ControllerInput::mut_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::vector3::Vector3>>(
                    "orientation",
                    ControllerInput::get_orientation_for_reflect,
                    ControllerInput::mut_orientation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "vertical_velocity",
                    ControllerInput::get_vertical_velocity_for_reflect,
                    ControllerInput::mut_vertical_velocity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "yaw_velocity",
                    ControllerInput::get_yaw_velocity_for_reflect,
                    ControllerInput::mut_yaw_velocity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ControllerInput>(
                    "ControllerInput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ControllerInput {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_time();
        self.clear_orientation();
        self.clear_vertical_velocity();
        self.clear_yaw_velocity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ControllerInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ControllerInput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xb1, 0x01, 0x0a, 0x0f, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x2a, 0x0a, 0x0b, 0x6f,
    0x72, 0x69, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x08, 0x2e, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x33, 0x52, 0x0b, 0x6f, 0x72, 0x69, 0x65,
    0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x0a, 0x11, 0x76, 0x65, 0x72, 0x74, 0x69,
    0x63, 0x61, 0x6c, 0x5f, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x10, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x56, 0x65, 0x6c, 0x6f,
    0x63, 0x69, 0x74, 0x79, 0x12, 0x21, 0x0a, 0x0c, 0x79, 0x61, 0x77, 0x5f, 0x76, 0x65, 0x6c, 0x6f,
    0x63, 0x69, 0x74, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0b, 0x79, 0x61, 0x77, 0x56,
    0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x4a, 0x93, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x0a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04,
    0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x05, 0x04, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x05, 0x0a, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x05, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06,
    0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x06, 0x04, 0x05,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x07, 0x04, 0x06, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x07, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x07, 0x0c, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x04, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x08, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x0a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x08, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x09, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x09,
    0x04, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x09, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x0a, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x09, 0x19, 0x1a, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
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
