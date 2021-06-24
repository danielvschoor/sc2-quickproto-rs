// Automatically generated rust module for 'data.proto' file

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
use crate::SC2APIProtocol::common::Race;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Attribute {
    Light = 1,
    Armored = 2,
    Biological = 3,
    Mechanical = 4,
    Robotic = 5,
    Psionic = 6,
    Massive = 7,
    Structure = 8,
    Hover = 9,
    Heroic = 10,
    Summoned = 11,
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute::Light
    }
}

impl From<i32> for Attribute {
    fn from(i: i32) -> Self {
        match i {
            1 => Attribute::Light,
            2 => Attribute::Armored,
            3 => Attribute::Biological,
            4 => Attribute::Mechanical,
            5 => Attribute::Robotic,
            6 => Attribute::Psionic,
            7 => Attribute::Massive,
            8 => Attribute::Structure,
            9 => Attribute::Hover,
            10 => Attribute::Heroic,
            11 => Attribute::Summoned,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Attribute {
    fn from(s: &'a str) -> Self {
        match s {
            "Light" => Attribute::Light,
            "Armored" => Attribute::Armored,
            "Biological" => Attribute::Biological,
            "Mechanical" => Attribute::Mechanical,
            "Robotic" => Attribute::Robotic,
            "Psionic" => Attribute::Psionic,
            "Massive" => Attribute::Massive,
            "Structure" => Attribute::Structure,
            "Hover" => Attribute::Hover,
            "Heroic" => Attribute::Heroic,
            "Summoned" => Attribute::Summoned,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AbilityData<'a> {
    pub ability_id: Option<u32>,
    pub link_name: Option<Cow<'a, str>>,
    pub link_index: Option<u32>,
    pub button_name: Option<Cow<'a, str>>,
    pub friendly_name: Option<Cow<'a, str>>,
    pub hotkey: Option<Cow<'a, str>>,
    pub remaps_to_ability_id: Option<u32>,
    pub available: Option<bool>,
    pub target: Option<mod_AbilityData::Target>,
    pub allow_minimap: Option<bool>,
    pub allow_autocast: Option<bool>,
    pub is_building: Option<bool>,
    pub footprint_radius: Option<f32>,
    pub is_instant_placement: Option<bool>,
    pub cast_range: Option<f32>,
}

impl<'a> MessageRead<'a> for AbilityData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.link_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.link_index = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.button_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.friendly_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.hotkey = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.remaps_to_ability_id = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.available = Some(r.read_bool(bytes)?),
                Ok(72) => msg.target = Some(r.read_enum(bytes)?),
                Ok(80) => msg.allow_minimap = Some(r.read_bool(bytes)?),
                Ok(88) => msg.allow_autocast = Some(r.read_bool(bytes)?),
                Ok(96) => msg.is_building = Some(r.read_bool(bytes)?),
                Ok(109) => msg.footprint_radius = Some(r.read_float(bytes)?),
                Ok(112) => msg.is_instant_placement = Some(r.read_bool(bytes)?),
                Ok(125) => msg.cast_range = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AbilityData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.link_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.link_index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.button_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.friendly_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.hotkey.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.remaps_to_ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.available.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.target.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.allow_minimap.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.allow_autocast.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.is_building.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.footprint_radius.as_ref().map_or(0, |_| 1 + 4)
        + self.is_instant_placement.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cast_range.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.link_name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.link_index { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.button_name { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.friendly_name { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.hotkey { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.remaps_to_ability_id { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.available { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.target { w.write_with_tag(72, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.allow_minimap { w.write_with_tag(80, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.allow_autocast { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_building { w.write_with_tag(96, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.footprint_radius { w.write_with_tag(109, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.is_instant_placement { w.write_with_tag(112, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.cast_range { w.write_with_tag(125, |w| w.write_float(*s))?; }
        Ok(())
    }
}

pub mod mod_AbilityData {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Target {
    None_pb = 1,
    Point = 2,
    Unit = 3,
    PointOrUnit = 4,
    PointOrNone = 5,
}

impl Default for Target {
    fn default() -> Self {
        Target::None_pb
    }
}

impl From<i32> for Target {
    fn from(i: i32) -> Self {
        match i {
            1 => Target::None_pb,
            2 => Target::Point,
            3 => Target::Unit,
            4 => Target::PointOrUnit,
            5 => Target::PointOrNone,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Target {
    fn from(s: &'a str) -> Self {
        match s {
            "None_pb" => Target::None_pb,
            "Point" => Target::Point,
            "Unit" => Target::Unit,
            "PointOrUnit" => Target::PointOrUnit,
            "PointOrNone" => Target::PointOrNone,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DamageBonus {
    pub attribute: Option<Attribute>,
    pub bonus: Option<f32>,
}

impl<'a> MessageRead<'a> for DamageBonus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.attribute = Some(r.read_enum(bytes)?),
                Ok(21) => msg.bonus = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DamageBonus {
    fn get_size(&self) -> usize {
        0
        + self.attribute.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bonus.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.attribute { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.bonus { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Weapon {
    pub type_pb: Option<mod_Weapon::TargetType>,
    pub damage: Option<f32>,
    pub damage_bonus: Vec<DamageBonus>,
    pub attacks: Option<u32>,
    pub range: Option<f32>,
    pub speed: Option<f32>,
}

impl<'a> MessageRead<'a> for Weapon {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(21) => msg.damage = Some(r.read_float(bytes)?),
                Ok(26) => msg.damage_bonus.push(r.read_message::<DamageBonus>(bytes)?),
                Ok(32) => msg.attacks = Some(r.read_uint32(bytes)?),
                Ok(45) => msg.range = Some(r.read_float(bytes)?),
                Ok(53) => msg.speed = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Weapon {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.damage.as_ref().map_or(0, |_| 1 + 4)
        + self.damage_bonus.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.attacks.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.range.as_ref().map_or(0, |_| 1 + 4)
        + self.speed.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.damage { w.write_with_tag(21, |w| w.write_float(*s))?; }
        for s in &self.damage_bonus { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.attacks { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.range { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.speed { w.write_with_tag(53, |w| w.write_float(*s))?; }
        Ok(())
    }
}

pub mod mod_Weapon {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TargetType {
    Ground = 1,
    Air = 2,
    Any = 3,
}

impl Default for TargetType {
    fn default() -> Self {
        TargetType::Ground
    }
}

impl From<i32> for TargetType {
    fn from(i: i32) -> Self {
        match i {
            1 => TargetType::Ground,
            2 => TargetType::Air,
            3 => TargetType::Any,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TargetType {
    fn from(s: &'a str) -> Self {
        match s {
            "Ground" => TargetType::Ground,
            "Air" => TargetType::Air,
            "Any" => TargetType::Any,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitTypeData<'a> {
    pub unit_id: Option<u32>,
    pub name: Option<Cow<'a, str>>,
    pub available: Option<bool>,
    pub cargo_size: Option<u32>,
    pub mineral_cost: Option<u32>,
    pub vespene_cost: Option<u32>,
    pub food_required: Option<f32>,
    pub food_provided: Option<f32>,
    pub ability_id: Option<u32>,
    pub race: Option<Race>,
    pub build_time: Option<f32>,
    pub has_vespene: Option<bool>,
    pub has_minerals: Option<bool>,
    pub sight_range: Option<f32>,
    pub tech_alias: Vec<u32>,
    pub unit_alias: Option<u32>,
    pub tech_requirement: Option<u32>,
    pub require_attached: Option<bool>,
    pub attributes: Vec<Attribute>,
    pub movement_speed: Option<f32>,
    pub armor: Option<f32>,
    pub weapons: Vec<Weapon>,
}

impl<'a> MessageRead<'a> for UnitTypeData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.available = Some(r.read_bool(bytes)?),
                Ok(32) => msg.cargo_size = Some(r.read_uint32(bytes)?),
                Ok(96) => msg.mineral_cost = Some(r.read_uint32(bytes)?),
                Ok(104) => msg.vespene_cost = Some(r.read_uint32(bytes)?),
                Ok(117) => msg.food_required = Some(r.read_float(bytes)?),
                Ok(149) => msg.food_provided = Some(r.read_float(bytes)?),
                Ok(120) => msg.ability_id = Some(r.read_uint32(bytes)?),
                Ok(128) => msg.race = Some(r.read_enum(bytes)?),
                Ok(141) => msg.build_time = Some(r.read_float(bytes)?),
                Ok(152) => msg.has_vespene = Some(r.read_bool(bytes)?),
                Ok(160) => msg.has_minerals = Some(r.read_bool(bytes)?),
                Ok(205) => msg.sight_range = Some(r.read_float(bytes)?),
                Ok(168) => msg.tech_alias.push(r.read_uint32(bytes)?),
                Ok(176) => msg.unit_alias = Some(r.read_uint32(bytes)?),
                Ok(184) => msg.tech_requirement = Some(r.read_uint32(bytes)?),
                Ok(192) => msg.require_attached = Some(r.read_bool(bytes)?),
                Ok(64) => msg.attributes.push(r.read_enum(bytes)?),
                Ok(77) => msg.movement_speed = Some(r.read_float(bytes)?),
                Ok(85) => msg.armor = Some(r.read_float(bytes)?),
                Ok(90) => msg.weapons.push(r.read_message::<Weapon>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UnitTypeData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.unit_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.available.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.cargo_size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.mineral_cost.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.vespene_cost.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.food_required.as_ref().map_or(0, |_| 1 + 4)
        + self.food_provided.as_ref().map_or(0, |_| 2 + 4)
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.race.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.build_time.as_ref().map_or(0, |_| 2 + 4)
        + self.has_vespene.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.has_minerals.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.sight_range.as_ref().map_or(0, |_| 2 + 4)
        + self.tech_alias.iter().map(|s| 2 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.unit_alias.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.tech_requirement.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.require_attached.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.attributes.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.movement_speed.as_ref().map_or(0, |_| 1 + 4)
        + self.armor.as_ref().map_or(0, |_| 1 + 4)
        + self.weapons.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.available { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.cargo_size { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.mineral_cost { w.write_with_tag(96, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.vespene_cost { w.write_with_tag(104, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.food_required { w.write_with_tag(117, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.food_provided { w.write_with_tag(149, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.ability_id { w.write_with_tag(120, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.race { w.write_with_tag(128, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.build_time { w.write_with_tag(141, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.has_vespene { w.write_with_tag(152, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.has_minerals { w.write_with_tag(160, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.sight_range { w.write_with_tag(205, |w| w.write_float(*s))?; }
        for s in &self.tech_alias { w.write_with_tag(168, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.unit_alias { w.write_with_tag(176, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.tech_requirement { w.write_with_tag(184, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.require_attached { w.write_with_tag(192, |w| w.write_bool(*s))?; }
        for s in &self.attributes { w.write_with_tag(64, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.movement_speed { w.write_with_tag(77, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.armor { w.write_with_tag(85, |w| w.write_float(*s))?; }
        for s in &self.weapons { w.write_with_tag(90, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpgradeData<'a> {
    pub upgrade_id: Option<u32>,
    pub name: Option<Cow<'a, str>>,
    pub mineral_cost: Option<u32>,
    pub vespene_cost: Option<u32>,
    pub research_time: Option<f32>,
    pub ability_id: Option<u32>,
}

impl<'a> MessageRead<'a> for UpgradeData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.upgrade_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.mineral_cost = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.vespene_cost = Some(r.read_uint32(bytes)?),
                Ok(45) => msg.research_time = Some(r.read_float(bytes)?),
                Ok(48) => msg.ability_id = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpgradeData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.upgrade_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.mineral_cost.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.vespene_cost.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.research_time.as_ref().map_or(0, |_| 1 + 4)
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.upgrade_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.mineral_cost { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.vespene_cost { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.research_time { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.ability_id { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BuffData<'a> {
    pub buff_id: Option<u32>,
    pub name: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BuffData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.buff_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BuffData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.buff_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.buff_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct EffectData<'a> {
    pub effect_id: Option<u32>,
    pub name: Option<Cow<'a, str>>,
    pub friendly_name: Option<Cow<'a, str>>,
    pub radius: Option<f32>,
}

impl<'a> MessageRead<'a> for EffectData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.effect_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.friendly_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(37) => msg.radius = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for EffectData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.effect_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.friendly_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.radius.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.effect_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.friendly_name { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.radius { w.write_with_tag(37, |w| w.write_float(*s))?; }
        Ok(())
    }
}

