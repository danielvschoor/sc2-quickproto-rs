// Automatically generated rust module for 'common.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Race {
    NoRace = 0,
    Terran = 1,
    Zerg = 2,
    Protoss = 3,
    Random = 4,
}

impl Default for Race {
    fn default() -> Self {
        Race::NoRace
    }
}

impl From<i32> for Race {
    fn from(i: i32) -> Self {
        match i {
            0 => Race::NoRace,
            1 => Race::Terran,
            2 => Race::Zerg,
            3 => Race::Protoss,
            4 => Race::Random,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Race {
    fn from(s: &'a str) -> Self {
        match s {
            "NoRace" => Race::NoRace,
            "Terran" => Race::Terran,
            "Zerg" => Race::Zerg,
            "Protoss" => Race::Protoss,
            "Random" => Race::Random,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AvailableAbility {
    pub ability_id: Option<i32>,
    pub requires_point: Option<bool>,
}

impl<'a> MessageRead<'a> for AvailableAbility {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_int32(bytes)?),
                Ok(16) => msg.requires_point = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AvailableAbility {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.requires_point.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.requires_point { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImageData<'a> {
    pub bits_per_pixel: Option<i32>,
    pub size: Option<Size2DI>,
    pub data: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ImageData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.bits_per_pixel = Some(r.read_int32(bytes)?),
                Ok(18) => msg.size = Some(r.read_message::<Size2DI>(bytes)?),
                Ok(26) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ImageData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.bits_per_pixel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.bits_per_pixel { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.data { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PointI {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl<'a> MessageRead<'a> for PointI {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.x = Some(r.read_int32(bytes)?),
                Ok(16) => msg.y = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PointI {
    fn get_size(&self) -> usize {
        0
        + self.x.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.y.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.x { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.y { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RectangleI {
    pub p0: Option<PointI>,
    pub p1: Option<PointI>,
}

impl<'a> MessageRead<'a> for RectangleI {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.p0 = Some(r.read_message::<PointI>(bytes)?),
                Ok(18) => msg.p1 = Some(r.read_message::<PointI>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RectangleI {
    fn get_size(&self) -> usize {
        0
        + self.p0.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.p1.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.p0 { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.p1 { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Point2D {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl<'a> MessageRead<'a> for Point2D {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = Some(r.read_float(bytes)?),
                Ok(21) => msg.y = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Point2D {
    fn get_size(&self) -> usize {
        0
        + self.x.as_ref().map_or(0, |_| 1 + 4)
        + self.y.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.x { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.y { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Point {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
}

impl<'a> MessageRead<'a> for Point {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = Some(r.read_float(bytes)?),
                Ok(21) => msg.y = Some(r.read_float(bytes)?),
                Ok(29) => msg.z = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Point {
    fn get_size(&self) -> usize {
        0
        + self.x.as_ref().map_or(0, |_| 1 + 4)
        + self.y.as_ref().map_or(0, |_| 1 + 4)
        + self.z.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.x { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.y { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.z { w.write_with_tag(29, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Size2DI {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl<'a> MessageRead<'a> for Size2DI {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.x = Some(r.read_int32(bytes)?),
                Ok(16) => msg.y = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Size2DI {
    fn get_size(&self) -> usize {
        0
        + self.x.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.y.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.x { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.y { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

