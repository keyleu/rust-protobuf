// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.12.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `protos/MsgIssue.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:MsgIssue)
pub struct MsgIssue {
    // message fields
    // @@protoc_insertion_point(field:MsgIssue.issuer)
    pub issuer: ::std::string::String,
    // @@protoc_insertion_point(field:MsgIssue.symbol)
    pub symbol: ::std::string::String,
    // @@protoc_insertion_point(field:MsgIssue.subunit)
    pub subunit: ::std::string::String,
    // @@protoc_insertion_point(field:MsgIssue.precision)
    pub precision: u32,
    // @@protoc_insertion_point(field:MsgIssue.initial_amount)
    pub initial_amount: ::std::string::String,
    // @@protoc_insertion_point(field:MsgIssue.description)
    pub description: ::std::string::String,
    // @@protoc_insertion_point(field:MsgIssue.features)
    pub features: ::std::vec::Vec<::protobuf::EnumOrUnknown<Feature>>,
    // @@protoc_insertion_point(field:MsgIssue.burn_rate)
    pub burn_rate: ::std::string::String,
    // @@protoc_insertion_point(field:MsgIssue.send_commission_rate)
    pub send_commission_rate: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:MsgIssue.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgIssue {
    fn default() -> &'a MsgIssue {
        <MsgIssue as ::protobuf::Message>::default_instance()
    }
}

impl MsgIssue {
    pub fn new() -> MsgIssue {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "issuer",
            |m: &MsgIssue| { &m.issuer },
            |m: &mut MsgIssue| { &mut m.issuer },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "symbol",
            |m: &MsgIssue| { &m.symbol },
            |m: &mut MsgIssue| { &mut m.symbol },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "subunit",
            |m: &MsgIssue| { &m.subunit },
            |m: &mut MsgIssue| { &mut m.subunit },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "precision",
            |m: &MsgIssue| { &m.precision },
            |m: &mut MsgIssue| { &mut m.precision },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "initial_amount",
            |m: &MsgIssue| { &m.initial_amount },
            |m: &mut MsgIssue| { &mut m.initial_amount },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "description",
            |m: &MsgIssue| { &m.description },
            |m: &mut MsgIssue| { &mut m.description },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "features",
            |m: &MsgIssue| { &m.features },
            |m: &mut MsgIssue| { &mut m.features },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "burn_rate",
            |m: &MsgIssue| { &m.burn_rate },
            |m: &mut MsgIssue| { &mut m.burn_rate },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "send_commission_rate",
            |m: &MsgIssue| { &m.send_commission_rate },
            |m: &mut MsgIssue| { &mut m.send_commission_rate },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgIssue>(
            "MsgIssue",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgIssue {
    const NAME: &'static str = "MsgIssue";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.issuer = is.read_string()?;
                },
                18 => {
                    self.symbol = is.read_string()?;
                },
                26 => {
                    self.subunit = is.read_string()?;
                },
                32 => {
                    self.precision = is.read_uint32()?;
                },
                42 => {
                    self.initial_amount = is.read_string()?;
                },
                50 => {
                    self.description = is.read_string()?;
                },
                56 => {
                    self.features.push(is.read_enum_or_unknown()?);
                },
                58 => {
                    ::protobuf::rt::read_repeated_packed_enum_or_unknown_into(is, &mut self.features)?
                },
                66 => {
                    self.burn_rate = is.read_string()?;
                },
                74 => {
                    self.send_commission_rate = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.issuer.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.issuer);
        }
        if !self.symbol.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.symbol);
        }
        if !self.subunit.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.subunit);
        }
        if self.precision != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.precision);
        }
        if !self.initial_amount.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.initial_amount);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.description);
        }
        for value in &self.features {
            my_size += ::protobuf::rt::int32_size(7, value.value());
        };
        if !self.burn_rate.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.burn_rate);
        }
        if !self.send_commission_rate.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.send_commission_rate);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.issuer.is_empty() {
            os.write_string(1, &self.issuer)?;
        }
        if !self.symbol.is_empty() {
            os.write_string(2, &self.symbol)?;
        }
        if !self.subunit.is_empty() {
            os.write_string(3, &self.subunit)?;
        }
        if self.precision != 0 {
            os.write_uint32(4, self.precision)?;
        }
        if !self.initial_amount.is_empty() {
            os.write_string(5, &self.initial_amount)?;
        }
        if !self.description.is_empty() {
            os.write_string(6, &self.description)?;
        }
        for v in &self.features {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(v))?;
        };
        if !self.burn_rate.is_empty() {
            os.write_string(8, &self.burn_rate)?;
        }
        if !self.send_commission_rate.is_empty() {
            os.write_string(9, &self.send_commission_rate)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgIssue {
        MsgIssue::new()
    }

    fn clear(&mut self) {
        self.issuer.clear();
        self.symbol.clear();
        self.subunit.clear();
        self.precision = 0;
        self.initial_amount.clear();
        self.description.clear();
        self.features.clear();
        self.burn_rate.clear();
        self.send_commission_rate.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgIssue {
        static instance: MsgIssue = MsgIssue {
            issuer: ::std::string::String::new(),
            symbol: ::std::string::String::new(),
            subunit: ::std::string::String::new(),
            precision: 0,
            initial_amount: ::std::string::String::new(),
            description: ::std::string::String::new(),
            features: ::std::vec::Vec::new(),
            burn_rate: ::std::string::String::new(),
            send_commission_rate: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgIssue {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgIssue").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgIssue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgIssue {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:Feature)
pub enum Feature {
    // @@protoc_insertion_point(enum_value:Feature.minting)
    minting = 0,
    // @@protoc_insertion_point(enum_value:Feature.burning)
    burning = 1,
    // @@protoc_insertion_point(enum_value:Feature.freezing)
    freezing = 2,
    // @@protoc_insertion_point(enum_value:Feature.whitelisting)
    whitelisting = 3,
    // @@protoc_insertion_point(enum_value:Feature.ibc)
    ibc = 4,
}

impl ::protobuf::Enum for Feature {
    const NAME: &'static str = "Feature";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Feature> {
        match value {
            0 => ::std::option::Option::Some(Feature::minting),
            1 => ::std::option::Option::Some(Feature::burning),
            2 => ::std::option::Option::Some(Feature::freezing),
            3 => ::std::option::Option::Some(Feature::whitelisting),
            4 => ::std::option::Option::Some(Feature::ibc),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [Feature] = &[
        Feature::minting,
        Feature::burning,
        Feature::freezing,
        Feature::whitelisting,
        Feature::ibc,
    ];
}

impl ::protobuf::EnumFull for Feature {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("Feature").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for Feature {
    fn default() -> Self {
        Feature::minting
    }
}

impl Feature {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Feature>("Feature")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15protos/MsgIssue.proto\"\xb0\x02\n\x08MsgIssue\x12\x16\n\x06issuer\
    \x18\x01\x20\x01(\tR\x06issuer\x12\x16\n\x06symbol\x18\x02\x20\x01(\tR\
    \x06symbol\x12\x18\n\x07subunit\x18\x03\x20\x01(\tR\x07subunit\x12\x1c\n\
    \tprecision\x18\x04\x20\x01(\rR\tprecision\x12%\n\x0einitial_amount\x18\
    \x05\x20\x01(\tR\rinitialAmount\x12\x20\n\x0bdescription\x18\x06\x20\x01\
    (\tR\x0bdescription\x12$\n\x08features\x18\x07\x20\x03(\x0e2\x08.Feature\
    R\x08features\x12\x1b\n\tburn_rate\x18\x08\x20\x01(\tR\x08burnRate\x120\
    \n\x14send_commission_rate\x18\t\x20\x01(\tR\x12sendCommissionRate*L\n\
    \x07Feature\x12\x0b\n\x07minting\x10\0\x12\x0b\n\x07burning\x10\x01\x12\
    \x0c\n\x08freezing\x10\x02\x12\x10\n\x0cwhitelisting\x10\x03\x12\x07\n\
    \x03ibc\x10\x04b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MsgIssue::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(Feature::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}