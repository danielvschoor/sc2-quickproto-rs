// Automatically generated rust module for 'raw.proto' file

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

use crate::SC2APIProtocol::data::*;
use crate::SC2APIProtocol::spatial::*;
use crate::SC2APIProtocol::ui::*;
use crate::SC2APIProtocol::score::*;
use crate::SC2APIProtocol::debug::*;
use crate::SC2APIProtocol::query::*;
use crate::SC2APIProtocol::error::*;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DisplayType {
    Visible = 1,
    Snapshot = 2,
    Hidden = 3,
    Placeholder = 4,
}

impl Default for DisplayType {
    fn default() -> Self {
        DisplayType::Visible
    }
}

impl From<i32> for DisplayType {
    fn from(i: i32) -> Self {
        match i {
            1 => DisplayType::Visible,
            2 => DisplayType::Snapshot,
            3 => DisplayType::Hidden,
            4 => DisplayType::Placeholder,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DisplayType {
    fn from(s: &'a str) -> Self {
        match s {
            "Visible" => DisplayType::Visible,
            "Snapshot" => DisplayType::Snapshot,
            "Hidden" => DisplayType::Hidden,
            "Placeholder" => DisplayType::Placeholder,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alliance {
    Self_pb = 1,
    Ally = 2,
    Neutral = 3,
    Enemy = 4,
}

impl Default for Alliance {
    fn default() -> Self {
        Alliance::Self_pb
    }
}

impl From<i32> for Alliance {
    fn from(i: i32) -> Self {
        match i {
            1 => Alliance::Self_pb,
            2 => Alliance::Ally,
            3 => Alliance::Neutral,
            4 => Alliance::Enemy,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Alliance {
    fn from(s: &'a str) -> Self {
        match s {
            "Self_pb" => Alliance::Self_pb,
            "Ally" => Alliance::Ally,
            "Neutral" => Alliance::Neutral,
            "Enemy" => Alliance::Enemy,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CloakState {
    CloakedUnknown = 0,
    Cloaked = 1,
    CloakedDetected = 2,
    NotCloaked = 3,
    CloakedAllied = 4,
}

impl Default for CloakState {
    fn default() -> Self {
        CloakState::CloakedUnknown
    }
}

impl From<i32> for CloakState {
    fn from(i: i32) -> Self {
        match i {
            0 => CloakState::CloakedUnknown,
            1 => CloakState::Cloaked,
            2 => CloakState::CloakedDetected,
            3 => CloakState::NotCloaked,
            4 => CloakState::CloakedAllied,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CloakState {
    fn from(s: &'a str) -> Self {
        match s {
            "CloakedUnknown" => CloakState::CloakedUnknown,
            "Cloaked" => CloakState::Cloaked,
            "CloakedDetected" => CloakState::CloakedDetected,
            "NotCloaked" => CloakState::NotCloaked,
            "CloakedAllied" => CloakState::CloakedAllied,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StartRaw<'a> {
    pub map_size: Option<Size2DI>,
    pub pathing_grid: Option<ImageData<'a>>,
    pub terrain_height: Option<ImageData<'a>>,
    pub placement_grid: Option<ImageData<'a>>,
    pub playable_area: Option<common::RectangleI>,
    pub start_locations: Vec<Point2D>,
}

impl<'a> MessageRead<'a> for StartRaw<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.map_size = Some(r.read_message::<Size2DI>(bytes)?),
                Ok(18) => msg.pathing_grid = Some(r.read_message::<ImageData>(bytes)?),
                Ok(26) => msg.terrain_height = Some(r.read_message::<ImageData>(bytes)?),
                Ok(34) => msg.placement_grid = Some(r.read_message::<ImageData>(bytes)?),
                Ok(42) => msg.playable_area = Some(r.read_message::<common::RectangleI>(bytes)?),
                Ok(50) => msg.start_locations.push(r.read_message::<Point2D>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StartRaw<'a> {
    fn get_size(&self) -> usize {
        0
        + self.map_size.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.pathing_grid.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.terrain_height.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.placement_grid.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.playable_area.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.start_locations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.map_size { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pathing_grid { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.terrain_height { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.placement_grid { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.playable_area { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.start_locations { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObservationRaw<'a> {
    pub player: Option<PlayerRaw>,
    pub units: Vec<Unit>,
    pub map_state: Option<MapState<'a>>,
    pub event: Option<Event>,
    pub effects: Vec<Effect>,
    pub radar: Vec<RadarRing>,
}

impl<'a> MessageRead<'a> for ObservationRaw<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.player = Some(r.read_message::<PlayerRaw>(bytes)?),
                Ok(18) => msg.units.push(r.read_message::<Unit>(bytes)?),
                Ok(26) => msg.map_state = Some(r.read_message::<MapState>(bytes)?),
                Ok(34) => msg.event = Some(r.read_message::<Event>(bytes)?),
                Ok(42) => msg.effects.push(r.read_message::<Effect>(bytes)?),
                Ok(50) => msg.radar.push(r.read_message::<RadarRing>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObservationRaw<'a> {
    fn get_size(&self) -> usize {
        0
        + self.player.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.units.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.map_state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.event.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.effects.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.radar.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.units { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.map_state { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.event { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.effects { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.radar { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RadarRing {
    pub pos: Option<Point>,
    pub radius: Option<f32>,
}

impl<'a> MessageRead<'a> for RadarRing {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pos = Some(r.read_message::<Point>(bytes)?),
                Ok(21) => msg.radius = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RadarRing {
    fn get_size(&self) -> usize {
        0
        + self.pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.radius.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.pos { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.radius { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PowerSource {
    pub pos: Option<Point>,
    pub radius: Option<f32>,
    pub tag: Option<u64>,
}

impl<'a> MessageRead<'a> for PowerSource {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pos = Some(r.read_message::<Point>(bytes)?),
                Ok(21) => msg.radius = Some(r.read_float(bytes)?),
                Ok(24) => msg.tag = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PowerSource {
    fn get_size(&self) -> usize {
        0
        + self.pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.radius.as_ref().map_or(0, |_| 1 + 4)
        + self.tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.pos { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.radius { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.tag { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerRaw {
    pub power_sources: Vec<PowerSource>,
    pub camera: Option<Point>,
    pub upgrade_ids: Vec<u32>,
}

impl<'a> MessageRead<'a> for PlayerRaw {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.power_sources.push(r.read_message::<PowerSource>(bytes)?),
                Ok(18) => msg.camera = Some(r.read_message::<Point>(bytes)?),
                Ok(24) => msg.upgrade_ids.push(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerRaw {
    fn get_size(&self) -> usize {
        0
        + self.power_sources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.camera.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.upgrade_ids.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.power_sources { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.camera { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.upgrade_ids { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitOrder {
    pub ability_id: Option<u32>,
    pub progress: Option<f32>,
    pub target: mod_UnitOrder::OneOftarget,
}

impl<'a> MessageRead<'a> for UnitOrder {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_uint32(bytes)?),
                Ok(37) => msg.progress = Some(r.read_float(bytes)?),
                Ok(18) => msg.target = mod_UnitOrder::OneOftarget::target_world_space_pos(r.read_message::<Point>(bytes)?),
                Ok(24) => msg.target = mod_UnitOrder::OneOftarget::target_unit_tag(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UnitOrder {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.progress.as_ref().map_or(0, |_| 1 + 4)
        + match self.target {
            mod_UnitOrder::OneOftarget::target_world_space_pos(ref m) => 1 + sizeof_len((m).get_size()),
            mod_UnitOrder::OneOftarget::target_unit_tag(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_UnitOrder::OneOftarget::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.progress { w.write_with_tag(37, |w| w.write_float(*s))?; }
        match self.target {            mod_UnitOrder::OneOftarget::target_world_space_pos(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_UnitOrder::OneOftarget::target_unit_tag(ref m) => { w.write_with_tag(24, |w| w.write_uint64(*m))? },
            mod_UnitOrder::OneOftarget::None => {},
    }        Ok(())
    }
}

pub mod mod_UnitOrder {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOftarget {
    target_world_space_pos(Point),
    target_unit_tag(u64),
    None,
}

impl Default for OneOftarget {
    fn default() -> Self {
        OneOftarget::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PassengerUnit {
    pub tag: Option<u64>,
    pub health: Option<f32>,
    pub health_max: Option<f32>,
    pub shield: Option<f32>,
    pub shield_max: Option<f32>,
    pub energy: Option<f32>,
    pub energy_max: Option<f32>,
    pub unit_type: Option<u32>,
}

impl<'a> MessageRead<'a> for PassengerUnit {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.tag = Some(r.read_uint64(bytes)?),
                Ok(21) => msg.health = Some(r.read_float(bytes)?),
                Ok(29) => msg.health_max = Some(r.read_float(bytes)?),
                Ok(37) => msg.shield = Some(r.read_float(bytes)?),
                Ok(61) => msg.shield_max = Some(r.read_float(bytes)?),
                Ok(45) => msg.energy = Some(r.read_float(bytes)?),
                Ok(69) => msg.energy_max = Some(r.read_float(bytes)?),
                Ok(48) => msg.unit_type = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PassengerUnit {
    fn get_size(&self) -> usize {
        0
        + self.tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.health.as_ref().map_or(0, |_| 1 + 4)
        + self.health_max.as_ref().map_or(0, |_| 1 + 4)
        + self.shield.as_ref().map_or(0, |_| 1 + 4)
        + self.shield_max.as_ref().map_or(0, |_| 1 + 4)
        + self.energy.as_ref().map_or(0, |_| 1 + 4)
        + self.energy_max.as_ref().map_or(0, |_| 1 + 4)
        + self.unit_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.tag { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.health { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.health_max { w.write_with_tag(29, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.shield { w.write_with_tag(37, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.shield_max { w.write_with_tag(61, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.energy { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.energy_max { w.write_with_tag(69, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.unit_type { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RallyTarget {
    pub point: Option<Point>,
    pub tag: Option<u64>,
}

impl<'a> MessageRead<'a> for RallyTarget {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.point = Some(r.read_message::<Point>(bytes)?),
                Ok(16) => msg.tag = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RallyTarget {
    fn get_size(&self) -> usize {
        0
        + self.point.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.point { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tag { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Unit {
    pub display_type: Option<DisplayType>,
    pub alliance: Option<Alliance>,
    pub tag: Option<u64>,
    pub unit_type: Option<u32>,
    pub owner: Option<i32>,
    pub pos: Option<Point>,
    pub facing: Option<f32>,
    pub radius: Option<f32>,
    pub build_progress: Option<f32>,
    pub cloak: Option<CloakState>,
    pub buff_ids: Vec<u32>,
    pub detect_range: Option<f32>,
    pub radar_range: Option<f32>,
    pub is_selected: Option<bool>,
    pub is_on_screen: Option<bool>,
    pub is_blip: Option<bool>,
    pub is_powered: Option<bool>,
    pub is_active: Option<bool>,
    pub attack_upgrade_level: Option<i32>,
    pub armor_upgrade_level: Option<i32>,
    pub shield_upgrade_level: Option<i32>,
    pub health: Option<f32>,
    pub health_max: Option<f32>,
    pub shield: Option<f32>,
    pub shield_max: Option<f32>,
    pub energy: Option<f32>,
    pub energy_max: Option<f32>,
    pub mineral_contents: Option<i32>,
    pub vespene_contents: Option<i32>,
    pub is_flying: Option<bool>,
    pub is_burrowed: Option<bool>,
    pub is_hallucination: Option<bool>,
    pub orders: Vec<UnitOrder>,
    pub add_on_tag: Option<u64>,
    pub passengers: Vec<PassengerUnit>,
    pub cargo_space_taken: Option<i32>,
    pub cargo_space_max: Option<i32>,
    pub assigned_harvesters: Option<i32>,
    pub ideal_harvesters: Option<i32>,
    pub weapon_cooldown: Option<f32>,
    pub engaged_target_tag: Option<u64>,
    pub buff_duration_remain: Option<i32>,
    pub buff_duration_max: Option<i32>,
    pub rally_targets: Vec<RallyTarget>,
}

impl<'a> MessageRead<'a> for Unit {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.display_type = Some(r.read_enum(bytes)?),
                Ok(16) => msg.alliance = Some(r.read_enum(bytes)?),
                Ok(24) => msg.tag = Some(r.read_uint64(bytes)?),
                Ok(32) => msg.unit_type = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.owner = Some(r.read_int32(bytes)?),
                Ok(50) => msg.pos = Some(r.read_message::<Point>(bytes)?),
                Ok(61) => msg.facing = Some(r.read_float(bytes)?),
                Ok(69) => msg.radius = Some(r.read_float(bytes)?),
                Ok(77) => msg.build_progress = Some(r.read_float(bytes)?),
                Ok(80) => msg.cloak = Some(r.read_enum(bytes)?),
                Ok(216) => msg.buff_ids.push(r.read_uint32(bytes)?),
                Ok(253) => msg.detect_range = Some(r.read_float(bytes)?),
                Ok(261) => msg.radar_range = Some(r.read_float(bytes)?),
                Ok(88) => msg.is_selected = Some(r.read_bool(bytes)?),
                Ok(96) => msg.is_on_screen = Some(r.read_bool(bytes)?),
                Ok(104) => msg.is_blip = Some(r.read_bool(bytes)?),
                Ok(280) => msg.is_powered = Some(r.read_bool(bytes)?),
                Ok(312) => msg.is_active = Some(r.read_bool(bytes)?),
                Ok(320) => msg.attack_upgrade_level = Some(r.read_int32(bytes)?),
                Ok(328) => msg.armor_upgrade_level = Some(r.read_int32(bytes)?),
                Ok(336) => msg.shield_upgrade_level = Some(r.read_int32(bytes)?),
                Ok(117) => msg.health = Some(r.read_float(bytes)?),
                Ok(125) => msg.health_max = Some(r.read_float(bytes)?),
                Ok(133) => msg.shield = Some(r.read_float(bytes)?),
                Ok(293) => msg.shield_max = Some(r.read_float(bytes)?),
                Ok(141) => msg.energy = Some(r.read_float(bytes)?),
                Ok(301) => msg.energy_max = Some(r.read_float(bytes)?),
                Ok(144) => msg.mineral_contents = Some(r.read_int32(bytes)?),
                Ok(152) => msg.vespene_contents = Some(r.read_int32(bytes)?),
                Ok(160) => msg.is_flying = Some(r.read_bool(bytes)?),
                Ok(168) => msg.is_burrowed = Some(r.read_bool(bytes)?),
                Ok(304) => msg.is_hallucination = Some(r.read_bool(bytes)?),
                Ok(178) => msg.orders.push(r.read_message::<UnitOrder>(bytes)?),
                Ok(184) => msg.add_on_tag = Some(r.read_uint64(bytes)?),
                Ok(194) => msg.passengers.push(r.read_message::<PassengerUnit>(bytes)?),
                Ok(200) => msg.cargo_space_taken = Some(r.read_int32(bytes)?),
                Ok(208) => msg.cargo_space_max = Some(r.read_int32(bytes)?),
                Ok(224) => msg.assigned_harvesters = Some(r.read_int32(bytes)?),
                Ok(232) => msg.ideal_harvesters = Some(r.read_int32(bytes)?),
                Ok(245) => msg.weapon_cooldown = Some(r.read_float(bytes)?),
                Ok(272) => msg.engaged_target_tag = Some(r.read_uint64(bytes)?),
                Ok(344) => msg.buff_duration_remain = Some(r.read_int32(bytes)?),
                Ok(352) => msg.buff_duration_max = Some(r.read_int32(bytes)?),
                Ok(362) => msg.rally_targets.push(r.read_message::<RallyTarget>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Unit {
    fn get_size(&self) -> usize {
        0
        + self.display_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.alliance.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unit_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.owner.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.facing.as_ref().map_or(0, |_| 1 + 4)
        + self.radius.as_ref().map_or(0, |_| 1 + 4)
        + self.build_progress.as_ref().map_or(0, |_| 1 + 4)
        + self.cloak.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.buff_ids.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.detect_range.as_ref().map_or(0, |_| 2 + 4)
        + self.radar_range.as_ref().map_or(0, |_| 2 + 4)
        + self.is_selected.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.is_on_screen.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.is_blip.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.is_powered.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.is_active.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.attack_upgrade_level.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.armor_upgrade_level.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.shield_upgrade_level.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.health.as_ref().map_or(0, |_| 1 + 4)
        + self.health_max.as_ref().map_or(0, |_| 1 + 4)
        + self.shield.as_ref().map_or(0, |_| 2 + 4)
        + self.shield_max.as_ref().map_or(0, |_| 2 + 4)
        + self.energy.as_ref().map_or(0, |_| 2 + 4)
        + self.energy_max.as_ref().map_or(0, |_| 2 + 4)
        + self.mineral_contents.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.vespene_contents.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.is_flying.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.is_burrowed.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.is_hallucination.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.orders.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.add_on_tag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.passengers.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.cargo_space_taken.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.cargo_space_max.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.assigned_harvesters.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.ideal_harvesters.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.weapon_cooldown.as_ref().map_or(0, |_| 2 + 4)
        + self.engaged_target_tag.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.buff_duration_remain.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.buff_duration_max.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.rally_targets.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.display_type { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.alliance { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.tag { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.unit_type { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.owner { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pos { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.facing { w.write_with_tag(61, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.radius { w.write_with_tag(69, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.build_progress { w.write_with_tag(77, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.cloak { w.write_with_tag(80, |w| w.write_enum(*s as i32))?; }
        for s in &self.buff_ids { w.write_with_tag(216, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.detect_range { w.write_with_tag(253, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.radar_range { w.write_with_tag(261, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.is_selected { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_on_screen { w.write_with_tag(96, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_blip { w.write_with_tag(104, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_powered { w.write_with_tag(280, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_active { w.write_with_tag(312, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.attack_upgrade_level { w.write_with_tag(320, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.armor_upgrade_level { w.write_with_tag(328, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.shield_upgrade_level { w.write_with_tag(336, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.health { w.write_with_tag(117, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.health_max { w.write_with_tag(125, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.shield { w.write_with_tag(133, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.shield_max { w.write_with_tag(293, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.energy { w.write_with_tag(141, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.energy_max { w.write_with_tag(301, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.mineral_contents { w.write_with_tag(144, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.vespene_contents { w.write_with_tag(152, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.is_flying { w.write_with_tag(160, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_burrowed { w.write_with_tag(168, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_hallucination { w.write_with_tag(304, |w| w.write_bool(*s))?; }
        for s in &self.orders { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.add_on_tag { w.write_with_tag(184, |w| w.write_uint64(*s))?; }
        for s in &self.passengers { w.write_with_tag(194, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cargo_space_taken { w.write_with_tag(200, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.cargo_space_max { w.write_with_tag(208, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.assigned_harvesters { w.write_with_tag(224, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.ideal_harvesters { w.write_with_tag(232, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.weapon_cooldown { w.write_with_tag(245, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.engaged_target_tag { w.write_with_tag(272, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.buff_duration_remain { w.write_with_tag(344, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.buff_duration_max { w.write_with_tag(352, |w| w.write_int32(*s))?; }
        for s in &self.rally_targets { w.write_with_tag(362, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MapState<'a> {
    pub visibility: Option<ImageData<'a>>,
    pub creep: Option<ImageData<'a>>,
}

impl<'a> MessageRead<'a> for MapState<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.visibility = Some(r.read_message::<ImageData>(bytes)?),
                Ok(18) => msg.creep = Some(r.read_message::<ImageData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MapState<'a> {
    fn get_size(&self) -> usize {
        0
        + self.visibility.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.creep.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.visibility { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.creep { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Event {
    pub dead_units: Vec<u64>,
}

impl<'a> MessageRead<'a> for Event {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.dead_units.push(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Event {
    fn get_size(&self) -> usize {
        0
        + self.dead_units.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.dead_units { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Effect {
    pub effect_id: Option<u32>,
    pub pos: Vec<Point2D>,
    pub alliance: Option<Alliance>,
    pub owner: Option<i32>,
    pub radius: Option<f32>,
}

impl<'a> MessageRead<'a> for Effect {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.effect_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.pos.push(r.read_message::<Point2D>(bytes)?),
                Ok(24) => msg.alliance = Some(r.read_enum(bytes)?),
                Ok(32) => msg.owner = Some(r.read_int32(bytes)?),
                Ok(45) => msg.radius = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Effect {
    fn get_size(&self) -> usize {
        0
        + self.effect_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pos.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.alliance.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.owner.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.radius.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.effect_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        for s in &self.pos { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.alliance { w.write_with_tag(24, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.owner { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.radius { w.write_with_tag(45, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionRaw {
    pub action: mod_ActionRaw::OneOfaction,
}

impl<'a> MessageRead<'a> for ActionRaw {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action = mod_ActionRaw::OneOfaction::unit_command(r.read_message::<ActionRawUnitCommand>(bytes)?),
                Ok(18) => msg.action = mod_ActionRaw::OneOfaction::camera_move(r.read_message::<ActionRawCameraMove>(bytes)?),
                Ok(26) => msg.action = mod_ActionRaw::OneOfaction::toggle_autocast(r.read_message::<ActionRawToggleAutocast>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionRaw {
    fn get_size(&self) -> usize {
        0
        + match self.action {
            mod_ActionRaw::OneOfaction::unit_command(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionRaw::OneOfaction::camera_move(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionRaw::OneOfaction::toggle_autocast(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionRaw::OneOfaction::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.action {            mod_ActionRaw::OneOfaction::unit_command(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_ActionRaw::OneOfaction::camera_move(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ActionRaw::OneOfaction::toggle_autocast(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_ActionRaw::OneOfaction::None => {},
    }        Ok(())
    }
}

pub mod mod_ActionRaw {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfaction {
    unit_command(ActionRawUnitCommand),
    camera_move(ActionRawCameraMove),
    toggle_autocast(ActionRawToggleAutocast),
    None,
}

impl Default for OneOfaction {
    fn default() -> Self {
        OneOfaction::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionRawUnitCommand {
    pub ability_id: Option<i32>,
    pub unit_tags: Vec<u64>,
    pub queue_command: Option<bool>,
    pub target: mod_ActionRawUnitCommand::OneOftarget,
}

impl<'a> MessageRead<'a> for ActionRawUnitCommand {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_int32(bytes)?),
                Ok(32) => msg.unit_tags.push(r.read_uint64(bytes)?),
                Ok(40) => msg.queue_command = Some(r.read_bool(bytes)?),
                Ok(18) => msg.target = mod_ActionRawUnitCommand::OneOftarget::target_world_space_pos(r.read_message::<Point2D>(bytes)?),
                Ok(24) => msg.target = mod_ActionRawUnitCommand::OneOftarget::target_unit_tag(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionRawUnitCommand {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unit_tags.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.queue_command.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + match self.target {
            mod_ActionRawUnitCommand::OneOftarget::target_world_space_pos(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionRawUnitCommand::OneOftarget::target_unit_tag(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_ActionRawUnitCommand::OneOftarget::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        for s in &self.unit_tags { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.queue_command { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        match self.target {            mod_ActionRawUnitCommand::OneOftarget::target_world_space_pos(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ActionRawUnitCommand::OneOftarget::target_unit_tag(ref m) => { w.write_with_tag(24, |w| w.write_uint64(*m))? },
            mod_ActionRawUnitCommand::OneOftarget::None => {},
    }        Ok(())
    }
}

pub mod mod_ActionRawUnitCommand {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOftarget {
    target_world_space_pos(Point2D),
    target_unit_tag(u64),
    None,
}

impl Default for OneOftarget {
    fn default() -> Self {
        OneOftarget::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionRawCameraMove {
    pub center_world_space: Option<Point>,
}

impl<'a> MessageRead<'a> for ActionRawCameraMove {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.center_world_space = Some(r.read_message::<Point>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionRawCameraMove {
    fn get_size(&self) -> usize {
        0
        + self.center_world_space.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.center_world_space { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionRawToggleAutocast {
    pub ability_id: Option<i32>,
    pub unit_tags: Vec<u64>,
}

impl<'a> MessageRead<'a> for ActionRawToggleAutocast {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_int32(bytes)?),
                Ok(16) => msg.unit_tags.push(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionRawToggleAutocast {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unit_tags.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        for s in &self.unit_tags { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

