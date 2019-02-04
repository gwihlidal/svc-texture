// automatically generated by the FlatBuffers compiler, do not modify


#![allow(dead_code)]
#![allow(unused_imports)]
extern crate flatbuffers;

pub mod service {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
pub mod texture {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
pub mod schema {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextureType {
  Tex1dArray = 0,
  Tex2d = 1,
  Tex2dArray = 2,
  Tex3d = 3,
  Cube = 4,
  CubeArray = 5,

}

const ENUM_MIN_TEXTURE_TYPE: i8 = 0;
const ENUM_MAX_TEXTURE_TYPE: i8 = 5;

impl<'a> flatbuffers::Follow<'a> for TextureType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for TextureType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = i8::to_le(self as i8);
    let p = &n as *const i8 as *const TextureType;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = i8::from_le(self as i8);
    let p = &n as *const i8 as *const TextureType;
    unsafe { *p }
  }
}

impl flatbuffers::Push for TextureType {
    type Output = TextureType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<TextureType>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_TEXTURE_TYPE:[TextureType; 6] = [
  TextureType::Tex1dArray,
  TextureType::Tex2d,
  TextureType::Tex2dArray,
  TextureType::Tex3d,
  TextureType::Cube,
  TextureType::CubeArray
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_TEXTURE_TYPE:[&'static str; 6] = [
    "Tex1dArray",
    "Tex2d",
    "Tex2dArray",
    "Tex3d",
    "Cube",
    "CubeArray"
];

pub fn enum_name_texture_type(e: TextureType) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_TEXTURE_TYPE[index]
}

#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextureFormat {
  UNKNOWN = 0,
  R4G4_UNORM = 1,
  R4G4B4A4_UNORM = 2,
  R5G6B5_UNORM = 3,
  R5G5B5A1_UNORM = 4,
  R8_UNORM = 5,
  R8_SNORM = 6,
  R8_SRGB = 7,
  R8_UINT = 8,
  R8_SINT = 9,
  R8G8_UNORM = 10,
  R8G8_SNORM = 11,
  R8G8_SRGB = 12,
  R8G8_UINT = 13,
  R8G8_SINT = 14,
  R8G8B8_UNORM = 15,
  R8G8B8_SRGB = 16,
  R8G8B8A8_UNORM = 17,
  R8G8B8A8_SNORM = 18,
  R8G8B8A8_SRGB = 19,
  R8G8B8A8_UINT = 20,
  R8G8B8A8_SINT = 21,
  B8G8R8A8_UNORM = 22,
  B8G8R8A8_SRGB = 23,
  R11G11B10_FLOAT = 24,
  R10G10B10A2_UNORM = 25,
  R10G10B10A2_UINT = 26,
  R9G9B9E5_FLOAT = 27,
  R16_FLOAT = 28,
  R16_UNORM = 29,
  R16_SNORM = 30,
  R16_UINT = 31,
  R16_SINT = 32,
  R16G16_FLOAT = 33,
  R16G16_UNORM = 34,
  R16G16_SNORM = 35,
  R16G16_UINT = 36,
  R16G16_SINT = 37,
  R16G16B16A16_FLOAT = 38,
  R16G16B16A16_UNORM = 39,
  R16G16B16A16_SNORM = 40,
  R16G16B16A16_UINT = 41,
  R16G16B16A16_SINT = 42,
  R32_FLOAT = 43,
  R32_UINT = 44,
  R32_SINT = 45,
  R32G32_FLOAT = 46,
  R32G32_UINT = 47,
  R32G32_SINT = 48,
  R32G32B32_FLOAT = 49,
  R32G32B32_UINT = 50,
  R32G32B32_SINT = 51,
  R32G32B32A32_FLOAT = 52,
  R32G32B32A32_UINT = 53,
  R32G32B32A32_SINT = 54,
  BC1_UNORM = 55,
  BC1_SRGB = 56,
  BC1A_UNORM = 57,
  BC1A_SRGB = 58,
  BC2_UNORM = 59,
  BC2_SRGB = 60,
  BC3_UNORM = 61,
  BC3_SRGB = 62,
  BC4_UNORM = 63,
  BC4_SNORM = 64,
  BC5_UNORM = 65,
  BC5_SNORM = 66,
  BC6U_FLOAT = 67,
  BC6S_FLOAT = 68,
  BC7_UNORM = 69,
  BC7_SRGB = 70,
  ASTC_4x4_UNORM = 71,
  ASTC_4x4_SRGB = 72,
  ASTC_5x4_UNORM = 73,
  ASTC_5x4_SRGB = 74,
  ASTC_5x5_UNORM = 75,
  ASTC_5x5_SRGB = 76,
  ASTC_6x5_UNORM = 77,
  ASTC_6x5_SRGB = 78,
  ASTC_6x6_UNORM = 79,
  ASTC_6x6_SRGB = 80,
  ASTC_8x5_UNORM = 81,
  ASTC_8x5_SRGB = 82,
  ASTC_8x6_SRGB = 83,
  ASTC_8x6_UNORM = 84,
  ASTC_8x8_UNORM = 85,
  ASTC_8x8_SRGB = 86,
  ASTC_10x5_UNORM = 87,
  ASTC_10x5_SRGB = 88,
  ASTC_10x6_UNORM = 89,
  ASTC_10x6_SRGB = 90,
  ASTC_10x8_UNORM = 91,
  ASTC_10x8_SRGB = 92,
  ASTC_10x10_UNORM = 93,
  ASTC_10x10_SRGB = 94,
  ASTC_12x10_UNORM = 95,
  ASTC_12x10_SRGB = 96,
  ASTC_12x12_UNORM = 97,
  ASTC_12x12_SRGB = 98,
  D24_UNORM_S8_UINT = 99,
  D32_FLOAT_S8_UINT = 100,
  D16_UNORM = 101,
  D32_FLOAT = 102,

}

const ENUM_MIN_TEXTURE_FORMAT: i8 = 0;
const ENUM_MAX_TEXTURE_FORMAT: i8 = 102;

impl<'a> flatbuffers::Follow<'a> for TextureFormat {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for TextureFormat {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = i8::to_le(self as i8);
    let p = &n as *const i8 as *const TextureFormat;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = i8::from_le(self as i8);
    let p = &n as *const i8 as *const TextureFormat;
    unsafe { *p }
  }
}

impl flatbuffers::Push for TextureFormat {
    type Output = TextureFormat;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<TextureFormat>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_TEXTURE_FORMAT:[TextureFormat; 103] = [
  TextureFormat::UNKNOWN,
  TextureFormat::R4G4_UNORM,
  TextureFormat::R4G4B4A4_UNORM,
  TextureFormat::R5G6B5_UNORM,
  TextureFormat::R5G5B5A1_UNORM,
  TextureFormat::R8_UNORM,
  TextureFormat::R8_SNORM,
  TextureFormat::R8_SRGB,
  TextureFormat::R8_UINT,
  TextureFormat::R8_SINT,
  TextureFormat::R8G8_UNORM,
  TextureFormat::R8G8_SNORM,
  TextureFormat::R8G8_SRGB,
  TextureFormat::R8G8_UINT,
  TextureFormat::R8G8_SINT,
  TextureFormat::R8G8B8_UNORM,
  TextureFormat::R8G8B8_SRGB,
  TextureFormat::R8G8B8A8_UNORM,
  TextureFormat::R8G8B8A8_SNORM,
  TextureFormat::R8G8B8A8_SRGB,
  TextureFormat::R8G8B8A8_UINT,
  TextureFormat::R8G8B8A8_SINT,
  TextureFormat::B8G8R8A8_UNORM,
  TextureFormat::B8G8R8A8_SRGB,
  TextureFormat::R11G11B10_FLOAT,
  TextureFormat::R10G10B10A2_UNORM,
  TextureFormat::R10G10B10A2_UINT,
  TextureFormat::R9G9B9E5_FLOAT,
  TextureFormat::R16_FLOAT,
  TextureFormat::R16_UNORM,
  TextureFormat::R16_SNORM,
  TextureFormat::R16_UINT,
  TextureFormat::R16_SINT,
  TextureFormat::R16G16_FLOAT,
  TextureFormat::R16G16_UNORM,
  TextureFormat::R16G16_SNORM,
  TextureFormat::R16G16_UINT,
  TextureFormat::R16G16_SINT,
  TextureFormat::R16G16B16A16_FLOAT,
  TextureFormat::R16G16B16A16_UNORM,
  TextureFormat::R16G16B16A16_SNORM,
  TextureFormat::R16G16B16A16_UINT,
  TextureFormat::R16G16B16A16_SINT,
  TextureFormat::R32_FLOAT,
  TextureFormat::R32_UINT,
  TextureFormat::R32_SINT,
  TextureFormat::R32G32_FLOAT,
  TextureFormat::R32G32_UINT,
  TextureFormat::R32G32_SINT,
  TextureFormat::R32G32B32_FLOAT,
  TextureFormat::R32G32B32_UINT,
  TextureFormat::R32G32B32_SINT,
  TextureFormat::R32G32B32A32_FLOAT,
  TextureFormat::R32G32B32A32_UINT,
  TextureFormat::R32G32B32A32_SINT,
  TextureFormat::BC1_UNORM,
  TextureFormat::BC1_SRGB,
  TextureFormat::BC1A_UNORM,
  TextureFormat::BC1A_SRGB,
  TextureFormat::BC2_UNORM,
  TextureFormat::BC2_SRGB,
  TextureFormat::BC3_UNORM,
  TextureFormat::BC3_SRGB,
  TextureFormat::BC4_UNORM,
  TextureFormat::BC4_SNORM,
  TextureFormat::BC5_UNORM,
  TextureFormat::BC5_SNORM,
  TextureFormat::BC6U_FLOAT,
  TextureFormat::BC6S_FLOAT,
  TextureFormat::BC7_UNORM,
  TextureFormat::BC7_SRGB,
  TextureFormat::ASTC_4x4_UNORM,
  TextureFormat::ASTC_4x4_SRGB,
  TextureFormat::ASTC_5x4_UNORM,
  TextureFormat::ASTC_5x4_SRGB,
  TextureFormat::ASTC_5x5_UNORM,
  TextureFormat::ASTC_5x5_SRGB,
  TextureFormat::ASTC_6x5_UNORM,
  TextureFormat::ASTC_6x5_SRGB,
  TextureFormat::ASTC_6x6_UNORM,
  TextureFormat::ASTC_6x6_SRGB,
  TextureFormat::ASTC_8x5_UNORM,
  TextureFormat::ASTC_8x5_SRGB,
  TextureFormat::ASTC_8x6_SRGB,
  TextureFormat::ASTC_8x6_UNORM,
  TextureFormat::ASTC_8x8_UNORM,
  TextureFormat::ASTC_8x8_SRGB,
  TextureFormat::ASTC_10x5_UNORM,
  TextureFormat::ASTC_10x5_SRGB,
  TextureFormat::ASTC_10x6_UNORM,
  TextureFormat::ASTC_10x6_SRGB,
  TextureFormat::ASTC_10x8_UNORM,
  TextureFormat::ASTC_10x8_SRGB,
  TextureFormat::ASTC_10x10_UNORM,
  TextureFormat::ASTC_10x10_SRGB,
  TextureFormat::ASTC_12x10_UNORM,
  TextureFormat::ASTC_12x10_SRGB,
  TextureFormat::ASTC_12x12_UNORM,
  TextureFormat::ASTC_12x12_SRGB,
  TextureFormat::D24_UNORM_S8_UINT,
  TextureFormat::D32_FLOAT_S8_UINT,
  TextureFormat::D16_UNORM,
  TextureFormat::D32_FLOAT
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_TEXTURE_FORMAT:[&'static str; 103] = [
    "UNKNOWN",
    "R4G4_UNORM",
    "R4G4B4A4_UNORM",
    "R5G6B5_UNORM",
    "R5G5B5A1_UNORM",
    "R8_UNORM",
    "R8_SNORM",
    "R8_SRGB",
    "R8_UINT",
    "R8_SINT",
    "R8G8_UNORM",
    "R8G8_SNORM",
    "R8G8_SRGB",
    "R8G8_UINT",
    "R8G8_SINT",
    "R8G8B8_UNORM",
    "R8G8B8_SRGB",
    "R8G8B8A8_UNORM",
    "R8G8B8A8_SNORM",
    "R8G8B8A8_SRGB",
    "R8G8B8A8_UINT",
    "R8G8B8A8_SINT",
    "B8G8R8A8_UNORM",
    "B8G8R8A8_SRGB",
    "R11G11B10_FLOAT",
    "R10G10B10A2_UNORM",
    "R10G10B10A2_UINT",
    "R9G9B9E5_FLOAT",
    "R16_FLOAT",
    "R16_UNORM",
    "R16_SNORM",
    "R16_UINT",
    "R16_SINT",
    "R16G16_FLOAT",
    "R16G16_UNORM",
    "R16G16_SNORM",
    "R16G16_UINT",
    "R16G16_SINT",
    "R16G16B16A16_FLOAT",
    "R16G16B16A16_UNORM",
    "R16G16B16A16_SNORM",
    "R16G16B16A16_UINT",
    "R16G16B16A16_SINT",
    "R32_FLOAT",
    "R32_UINT",
    "R32_SINT",
    "R32G32_FLOAT",
    "R32G32_UINT",
    "R32G32_SINT",
    "R32G32B32_FLOAT",
    "R32G32B32_UINT",
    "R32G32B32_SINT",
    "R32G32B32A32_FLOAT",
    "R32G32B32A32_UINT",
    "R32G32B32A32_SINT",
    "BC1_UNORM",
    "BC1_SRGB",
    "BC1A_UNORM",
    "BC1A_SRGB",
    "BC2_UNORM",
    "BC2_SRGB",
    "BC3_UNORM",
    "BC3_SRGB",
    "BC4_UNORM",
    "BC4_SNORM",
    "BC5_UNORM",
    "BC5_SNORM",
    "BC6U_FLOAT",
    "BC6S_FLOAT",
    "BC7_UNORM",
    "BC7_SRGB",
    "ASTC_4x4_UNORM",
    "ASTC_4x4_SRGB",
    "ASTC_5x4_UNORM",
    "ASTC_5x4_SRGB",
    "ASTC_5x5_UNORM",
    "ASTC_5x5_SRGB",
    "ASTC_6x5_UNORM",
    "ASTC_6x5_SRGB",
    "ASTC_6x6_UNORM",
    "ASTC_6x6_SRGB",
    "ASTC_8x5_UNORM",
    "ASTC_8x5_SRGB",
    "ASTC_8x6_SRGB",
    "ASTC_8x6_UNORM",
    "ASTC_8x8_UNORM",
    "ASTC_8x8_SRGB",
    "ASTC_10x5_UNORM",
    "ASTC_10x5_SRGB",
    "ASTC_10x6_UNORM",
    "ASTC_10x6_SRGB",
    "ASTC_10x8_UNORM",
    "ASTC_10x8_SRGB",
    "ASTC_10x10_UNORM",
    "ASTC_10x10_SRGB",
    "ASTC_12x10_UNORM",
    "ASTC_12x10_SRGB",
    "ASTC_12x12_UNORM",
    "ASTC_12x12_SRGB",
    "D24_UNORM_S8_UINT",
    "D32_FLOAT_S8_UINT",
    "D16_UNORM",
    "D32_FLOAT"
];

pub fn enum_name_texture_format(e: TextureFormat) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_TEXTURE_FORMAT[index]
}

pub enum TextureDataOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TextureData<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TextureData<'a> {
    type Inner = TextureData<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TextureData<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TextureData {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TextureDataArgs<'args>) -> flatbuffers::WIPOffset<TextureData<'bldr>> {
      let mut builder = TextureDataBuilder::new(_fbb);
      if let Some(x) = args.data { builder.add_data(x); }
      builder.add_slice_pitch(args.slice_pitch);
      builder.add_row_pitch(args.row_pitch);
      builder.finish()
    }

    pub const VT_ROW_PITCH: flatbuffers::VOffsetT = 4;
    pub const VT_SLICE_PITCH: flatbuffers::VOffsetT = 6;
    pub const VT_DATA: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn row_pitch(&self) -> u32 {
    self._tab.get::<u32>(TextureData::VT_ROW_PITCH, Some(0)).unwrap()
  }
  #[inline]
  pub fn slice_pitch(&self) -> u32 {
    self._tab.get::<u32>(TextureData::VT_SLICE_PITCH, Some(0)).unwrap()
  }
  #[inline]
  pub fn data(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(TextureData::VT_DATA, None).map(|v| v.safe_slice())
  }
}

pub struct TextureDataArgs<'a> {
    pub row_pitch: u32,
    pub slice_pitch: u32,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
}
impl<'a> Default for TextureDataArgs<'a> {
    #[inline]
    fn default() -> Self {
        TextureDataArgs {
            row_pitch: 0,
            slice_pitch: 0,
            data: None,
        }
    }
}
pub struct TextureDataBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TextureDataBuilder<'a, 'b> {
  #[inline]
  pub fn add_row_pitch(&mut self, row_pitch: u32) {
    self.fbb_.push_slot::<u32>(TextureData::VT_ROW_PITCH, row_pitch, 0);
  }
  #[inline]
  pub fn add_slice_pitch(&mut self, slice_pitch: u32) {
    self.fbb_.push_slot::<u32>(TextureData::VT_SLICE_PITCH, slice_pitch, 0);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(TextureData::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TextureDataBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TextureDataBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TextureData<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum TextureDescOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TextureDesc<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TextureDesc<'a> {
    type Inner = TextureDesc<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TextureDesc<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TextureDesc {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TextureDescArgs) -> flatbuffers::WIPOffset<TextureDesc<'bldr>> {
      let mut builder = TextureDescBuilder::new(_fbb);
      builder.add_elements(args.elements);
      builder.add_levels(args.levels);
      builder.add_depth(args.depth);
      builder.add_height(args.height);
      builder.add_width(args.width);
      builder.add_format(args.format);
      builder.add_type_(args.type_);
      builder.finish()
    }

    pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
    pub const VT_FORMAT: flatbuffers::VOffsetT = 6;
    pub const VT_WIDTH: flatbuffers::VOffsetT = 8;
    pub const VT_HEIGHT: flatbuffers::VOffsetT = 10;
    pub const VT_DEPTH: flatbuffers::VOffsetT = 12;
    pub const VT_LEVELS: flatbuffers::VOffsetT = 14;
    pub const VT_ELEMENTS: flatbuffers::VOffsetT = 16;

  #[inline]
  pub fn type_(&self) -> TextureType {
    self._tab.get::<TextureType>(TextureDesc::VT_TYPE_, Some(TextureType::Tex1dArray)).unwrap()
  }
  #[inline]
  pub fn format(&self) -> TextureFormat {
    self._tab.get::<TextureFormat>(TextureDesc::VT_FORMAT, Some(TextureFormat::UNKNOWN)).unwrap()
  }
  #[inline]
  pub fn width(&self) -> u32 {
    self._tab.get::<u32>(TextureDesc::VT_WIDTH, Some(0)).unwrap()
  }
  #[inline]
  pub fn height(&self) -> u32 {
    self._tab.get::<u32>(TextureDesc::VT_HEIGHT, Some(0)).unwrap()
  }
  #[inline]
  pub fn depth(&self) -> u32 {
    self._tab.get::<u32>(TextureDesc::VT_DEPTH, Some(0)).unwrap()
  }
  #[inline]
  pub fn levels(&self) -> u32 {
    self._tab.get::<u32>(TextureDesc::VT_LEVELS, Some(0)).unwrap()
  }
  #[inline]
  pub fn elements(&self) -> u32 {
    self._tab.get::<u32>(TextureDesc::VT_ELEMENTS, Some(0)).unwrap()
  }
}

pub struct TextureDescArgs {
    pub type_: TextureType,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub levels: u32,
    pub elements: u32,
}
impl<'a> Default for TextureDescArgs {
    #[inline]
    fn default() -> Self {
        TextureDescArgs {
            type_: TextureType::Tex1dArray,
            format: TextureFormat::UNKNOWN,
            width: 0,
            height: 0,
            depth: 0,
            levels: 0,
            elements: 0,
        }
    }
}
pub struct TextureDescBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TextureDescBuilder<'a, 'b> {
  #[inline]
  pub fn add_type_(&mut self, type_: TextureType) {
    self.fbb_.push_slot::<TextureType>(TextureDesc::VT_TYPE_, type_, TextureType::Tex1dArray);
  }
  #[inline]
  pub fn add_format(&mut self, format: TextureFormat) {
    self.fbb_.push_slot::<TextureFormat>(TextureDesc::VT_FORMAT, format, TextureFormat::UNKNOWN);
  }
  #[inline]
  pub fn add_width(&mut self, width: u32) {
    self.fbb_.push_slot::<u32>(TextureDesc::VT_WIDTH, width, 0);
  }
  #[inline]
  pub fn add_height(&mut self, height: u32) {
    self.fbb_.push_slot::<u32>(TextureDesc::VT_HEIGHT, height, 0);
  }
  #[inline]
  pub fn add_depth(&mut self, depth: u32) {
    self.fbb_.push_slot::<u32>(TextureDesc::VT_DEPTH, depth, 0);
  }
  #[inline]
  pub fn add_levels(&mut self, levels: u32) {
    self.fbb_.push_slot::<u32>(TextureDesc::VT_LEVELS, levels, 0);
  }
  #[inline]
  pub fn add_elements(&mut self, elements: u32) {
    self.fbb_.push_slot::<u32>(TextureDesc::VT_ELEMENTS, elements, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TextureDescBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TextureDescBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TextureDesc<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum TextureOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Texture<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Texture<'a> {
    type Inner = Texture<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Texture<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Texture {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TextureArgs<'args>) -> flatbuffers::WIPOffset<Texture<'bldr>> {
      let mut builder = TextureBuilder::new(_fbb);
      if let Some(x) = args.data { builder.add_data(x); }
      if let Some(x) = args.desc { builder.add_desc(x); }
      if let Some(x) = args.identity { builder.add_identity(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      builder.finish()
    }

    pub const VT_NAME: flatbuffers::VOffsetT = 4;
    pub const VT_IDENTITY: flatbuffers::VOffsetT = 6;
    pub const VT_DESC: flatbuffers::VOffsetT = 8;
    pub const VT_DATA: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Texture::VT_NAME, None)
  }
  #[inline]
  pub fn identity(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Texture::VT_IDENTITY, None)
  }
  #[inline]
  pub fn desc(&self) -> Option<TextureDesc<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<TextureDesc<'a>>>(Texture::VT_DESC, None)
  }
  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<TextureData<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<TextureData<'a>>>>>(Texture::VT_DATA, None)
  }
}

pub struct TextureArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub identity: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub desc: Option<flatbuffers::WIPOffset<TextureDesc<'a >>>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<TextureData<'a >>>>>,
}
impl<'a> Default for TextureArgs<'a> {
    #[inline]
    fn default() -> Self {
        TextureArgs {
            name: None,
            identity: None,
            desc: None,
            data: None,
        }
    }
}
pub struct TextureBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TextureBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Texture::VT_NAME, name);
  }
  #[inline]
  pub fn add_identity(&mut self, identity: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Texture::VT_IDENTITY, identity);
  }
  #[inline]
  pub fn add_desc(&mut self, desc: flatbuffers::WIPOffset<TextureDesc<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<TextureDesc>>(Texture::VT_DESC, desc);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<TextureData<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Texture::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TextureBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TextureBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Texture<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum ManifestOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Manifest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Manifest<'a> {
    type Inner = Manifest<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Manifest<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Manifest {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ManifestArgs<'args>) -> flatbuffers::WIPOffset<Manifest<'bldr>> {
      let mut builder = ManifestBuilder::new(_fbb);
      if let Some(x) = args.textures { builder.add_textures(x); }
      builder.finish()
    }

    pub const VT_TEXTURES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn textures(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Texture<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Texture<'a>>>>>(Manifest::VT_TEXTURES, None)
  }
}

pub struct ManifestArgs<'a> {
    pub textures: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Texture<'a >>>>>,
}
impl<'a> Default for ManifestArgs<'a> {
    #[inline]
    fn default() -> Self {
        ManifestArgs {
            textures: None,
        }
    }
}
pub struct ManifestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ManifestBuilder<'a, 'b> {
  #[inline]
  pub fn add_textures(&mut self, textures: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Texture<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Manifest::VT_TEXTURES, textures);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ManifestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ManifestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Manifest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_manifest<'a>(buf: &'a [u8]) -> Manifest<'a> {
  flatbuffers::get_root::<Manifest<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_manifest<'a>(buf: &'a [u8]) -> Manifest<'a> {
  flatbuffers::get_size_prefixed_root::<Manifest<'a>>(buf)
}

#[inline]
pub fn finish_manifest_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Manifest<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_manifest_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Manifest<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod schema
}  // pub mod texture
}  // pub mod service

