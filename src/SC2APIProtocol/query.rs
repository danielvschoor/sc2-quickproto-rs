// Automatically generated rust module for 'query.proto' file

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
use crate::SC2APIProtocol::common::*;
use crate::SC2APIProtocol::raw::*;
use crate::SC2APIProtocol::data::*;
use crate::SC2APIProtocol::spatial::*;
use crate::SC2APIProtocol::ui::*;
use crate::SC2APIProtocol::score::*;
use crate::SC2APIProtocol::debug::*;

use crate::SC2APIProtocol::error::*;
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestQuery {
    pub pathing: Vec<RequestQueryPathing>,
    pub abilities: Vec<RequestQueryAvailableAbilities>,
    pub placements: Vec<RequestQueryBuildingPlacement>,
    pub ignore_resource_requirements: Option<bool>,
}

impl<'a> MessageRead<'a> for RequestQuery {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pathing.push(r.read_message::<RequestQueryPathing>(bytes)?),
                Ok(18) => msg.abilities.push(r.read_message::<RequestQueryAvailableAbilities>(bytes)?),
                Ok(26) => msg.placements.push(r.read_message::<RequestQueryBuildingPlacement>(bytes)?),
                Ok(32) => msg.ignore_resource_requirements = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestQuery {
    fn get_size(&self) -> usize {
        0
        + self.pathing.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.abilities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.placements.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.ignore_resource_requirements.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.pathing { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.abilities { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.placements { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ignore_resource_requirements { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseQuery {
    pub pathing: Vec<ResponseQueryPathing>,
    pub abilities: Vec<ResponseQueryAvailableAbilities>,
    pub placements: Vec<ResponseQueryBuildingPlacement>,
}

impl<'a> MessageRead<'a> for ResponseQuery {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pathing.push(r.read_message::<ResponseQueryPathing>(bytes)?),
                Ok(18) => msg.abilities.push(r.read_message::<ResponseQueryAvailableAbilities>(bytes)?),
                Ok(26) => msg.placements.push(r.read_message::<ResponseQueryBuildingPlacement>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponseQuery {
    fn get_size(&self) -> usize {
        0
        + self.pathing.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.abilities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.placements.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.pathing { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.abilities { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.placements { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestQueryPathing {
    pub end_pos: Option<Point2D>,
    pub start: mod_RequestQueryPathing::OneOfstart,
}

impl<'a> MessageRead<'a> for RequestQueryPathing {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.end_pos = Some(r.read_message::<Point2D>(bytes)?),
                Ok(10) => msg.start = mod_RequestQueryPathing::OneOfstart::start_pos(r.read_message::<Point2D>(bytes)?),
                Ok(16) => msg.start = mod_RequestQueryPathing::OneOfstart::unit_tag(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestQueryPathing {
    fn get_size(&self) -> usize {
        0
        + self.end_pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + match self.start {
            mod_RequestQueryPathing::OneOfstart::start_pos(ref m) => 1 + sizeof_len((m).get_size()),
            mod_RequestQueryPathing::OneOfstart::unit_tag(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_RequestQueryPathing::OneOfstart::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.end_pos { w.write_with_tag(26, |w| w.write_message(s))?; }
        match self.start {            mod_RequestQueryPathing::OneOfstart::start_pos(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_RequestQueryPathing::OneOfstart::unit_tag(ref m) => { w.write_with_tag(16, |w| w.write_uint64(*m))? },
            mod_RequestQueryPathing::OneOfstart::None => {},
    }        Ok(())
    }
}

pub mod mod_RequestQueryPathing {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfstart {
    start_pos(Point2D),
    unit_tag(u64),
    None,
}

impl Default for OneOfstart {
    fn default() -> Self {
        OneOfstart::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseQueryPathing {
    pub distance: Option<f32>,
}

impl<'a> MessageRead<'a> for ResponseQueryPathing {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.distance = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponseQueryPathing {
    fn get_size(&self) -> usize {
        0
        + self.distance.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.distance { w.write_with_tag(13, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestQueryAvailableAbilities {
    pub unit_tag: Option<u64>,
}

impl<'a> MessageRead<'a> for RequestQueryAvailableAbilities {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_tag = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestQueryAvailableAbilities {
    fn get_size(&self) -> usize {
        0
        + self.unit_tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_tag { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseQueryAvailableAbilities {
    pub abilities: Vec<AvailableAbility>,
    pub unit_tag: Option<u64>,
    pub unit_type_id: Option<u32>,
}

impl<'a> MessageRead<'a> for ResponseQueryAvailableAbilities {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.abilities.push(r.read_message::<AvailableAbility>(bytes)?),
                Ok(16) => msg.unit_tag = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.unit_type_id = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponseQueryAvailableAbilities {
    fn get_size(&self) -> usize {
        0
        + self.abilities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.unit_tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unit_type_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.abilities { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_tag { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.unit_type_id { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestQueryBuildingPlacement {
    pub ability_id: Option<i32>,
    pub target_pos: Option<Point2D>,
    pub placing_unit_tag: Option<u64>,
}

impl<'a> MessageRead<'a> for RequestQueryBuildingPlacement {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_int32(bytes)?),
                Ok(18) => msg.target_pos = Some(r.read_message::<Point2D>(bytes)?),
                Ok(24) => msg.placing_unit_tag = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestQueryBuildingPlacement {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.target_pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.placing_unit_tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.target_pos { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.placing_unit_tag { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseQueryBuildingPlacement {
    pub result: Option<ActionResult>,
}

impl<'a> MessageRead<'a> for ResponseQueryBuildingPlacement {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponseQueryBuildingPlacement {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

