// Automatically generated rust module for 'score.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Score {
    pub score_type: Option<mod_Score::ScoreType>,
    pub score: Option<i32>,
    pub score_details: Option<ScoreDetails>,
}

impl<'a> MessageRead<'a> for Score {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(48) => msg.score_type = Some(r.read_enum(bytes)?),
                Ok(56) => msg.score = Some(r.read_int32(bytes)?),
                Ok(66) => msg.score_details = Some(r.read_message::<ScoreDetails>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Score {
    fn get_size(&self) -> usize {
        0
        + self.score_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.score.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.score_details.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.score_type { w.write_with_tag(48, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.score { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.score_details { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Score {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScoreType {
    Curriculum = 1,
    Melee = 2,
}

impl Default for ScoreType {
    fn default() -> Self {
        ScoreType::Curriculum
    }
}

impl From<i32> for ScoreType {
    fn from(i: i32) -> Self {
        match i {
            1 => ScoreType::Curriculum,
            2 => ScoreType::Melee,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ScoreType {
    fn from(s: &'a str) -> Self {
        match s {
            "Curriculum" => ScoreType::Curriculum,
            "Melee" => ScoreType::Melee,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CategoryScoreDetails {
    pub none: Option<f32>,
    pub army: Option<f32>,
    pub economy: Option<f32>,
    pub technology: Option<f32>,
    pub upgrade: Option<f32>,
}

impl<'a> MessageRead<'a> for CategoryScoreDetails {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.none = Some(r.read_float(bytes)?),
                Ok(21) => msg.army = Some(r.read_float(bytes)?),
                Ok(29) => msg.economy = Some(r.read_float(bytes)?),
                Ok(37) => msg.technology = Some(r.read_float(bytes)?),
                Ok(45) => msg.upgrade = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CategoryScoreDetails {
    fn get_size(&self) -> usize {
        0
        + self.none.as_ref().map_or(0, |_| 1 + 4)
        + self.army.as_ref().map_or(0, |_| 1 + 4)
        + self.economy.as_ref().map_or(0, |_| 1 + 4)
        + self.technology.as_ref().map_or(0, |_| 1 + 4)
        + self.upgrade.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.none { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.army { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.economy { w.write_with_tag(29, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.technology { w.write_with_tag(37, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.upgrade { w.write_with_tag(45, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct VitalScoreDetails {
    pub life: Option<f32>,
    pub shields: Option<f32>,
    pub energy: Option<f32>,
}

impl<'a> MessageRead<'a> for VitalScoreDetails {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.life = Some(r.read_float(bytes)?),
                Ok(21) => msg.shields = Some(r.read_float(bytes)?),
                Ok(29) => msg.energy = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for VitalScoreDetails {
    fn get_size(&self) -> usize {
        0
        + self.life.as_ref().map_or(0, |_| 1 + 4)
        + self.shields.as_ref().map_or(0, |_| 1 + 4)
        + self.energy.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.life { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.shields { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.energy { w.write_with_tag(29, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ScoreDetails {
    pub idle_production_time: Option<f32>,
    pub idle_worker_time: Option<f32>,
    pub total_value_units: Option<f32>,
    pub total_value_structures: Option<f32>,
    pub killed_value_units: Option<f32>,
    pub killed_value_structures: Option<f32>,
    pub collected_minerals: Option<f32>,
    pub collected_vespene: Option<f32>,
    pub collection_rate_minerals: Option<f32>,
    pub collection_rate_vespene: Option<f32>,
    pub spent_minerals: Option<f32>,
    pub spent_vespene: Option<f32>,
    pub food_used: Option<CategoryScoreDetails>,
    pub killed_minerals: Option<CategoryScoreDetails>,
    pub killed_vespene: Option<CategoryScoreDetails>,
    pub lost_minerals: Option<CategoryScoreDetails>,
    pub lost_vespene: Option<CategoryScoreDetails>,
    pub friendly_fire_minerals: Option<CategoryScoreDetails>,
    pub friendly_fire_vespene: Option<CategoryScoreDetails>,
    pub used_minerals: Option<CategoryScoreDetails>,
    pub used_vespene: Option<CategoryScoreDetails>,
    pub total_used_minerals: Option<CategoryScoreDetails>,
    pub total_used_vespene: Option<CategoryScoreDetails>,
    pub total_damage_dealt: Option<VitalScoreDetails>,
    pub total_damage_taken: Option<VitalScoreDetails>,
    pub total_healed: Option<VitalScoreDetails>,
    pub current_apm: Option<f32>,
    pub current_effective_apm: Option<f32>,
}

impl<'a> MessageRead<'a> for ScoreDetails {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.idle_production_time = Some(r.read_float(bytes)?),
                Ok(21) => msg.idle_worker_time = Some(r.read_float(bytes)?),
                Ok(29) => msg.total_value_units = Some(r.read_float(bytes)?),
                Ok(37) => msg.total_value_structures = Some(r.read_float(bytes)?),
                Ok(45) => msg.killed_value_units = Some(r.read_float(bytes)?),
                Ok(53) => msg.killed_value_structures = Some(r.read_float(bytes)?),
                Ok(61) => msg.collected_minerals = Some(r.read_float(bytes)?),
                Ok(69) => msg.collected_vespene = Some(r.read_float(bytes)?),
                Ok(77) => msg.collection_rate_minerals = Some(r.read_float(bytes)?),
                Ok(85) => msg.collection_rate_vespene = Some(r.read_float(bytes)?),
                Ok(93) => msg.spent_minerals = Some(r.read_float(bytes)?),
                Ok(101) => msg.spent_vespene = Some(r.read_float(bytes)?),
                Ok(106) => msg.food_used = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(114) => msg.killed_minerals = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(122) => msg.killed_vespene = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(130) => msg.lost_minerals = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(138) => msg.lost_vespene = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(146) => msg.friendly_fire_minerals = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(154) => msg.friendly_fire_vespene = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(162) => msg.used_minerals = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(170) => msg.used_vespene = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(178) => msg.total_used_minerals = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(186) => msg.total_used_vespene = Some(r.read_message::<CategoryScoreDetails>(bytes)?),
                Ok(194) => msg.total_damage_dealt = Some(r.read_message::<VitalScoreDetails>(bytes)?),
                Ok(202) => msg.total_damage_taken = Some(r.read_message::<VitalScoreDetails>(bytes)?),
                Ok(210) => msg.total_healed = Some(r.read_message::<VitalScoreDetails>(bytes)?),
                Ok(221) => msg.current_apm = Some(r.read_float(bytes)?),
                Ok(229) => msg.current_effective_apm = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ScoreDetails {
    fn get_size(&self) -> usize {
        0
        + self.idle_production_time.as_ref().map_or(0, |_| 1 + 4)
        + self.idle_worker_time.as_ref().map_or(0, |_| 1 + 4)
        + self.total_value_units.as_ref().map_or(0, |_| 1 + 4)
        + self.total_value_structures.as_ref().map_or(0, |_| 1 + 4)
        + self.killed_value_units.as_ref().map_or(0, |_| 1 + 4)
        + self.killed_value_structures.as_ref().map_or(0, |_| 1 + 4)
        + self.collected_minerals.as_ref().map_or(0, |_| 1 + 4)
        + self.collected_vespene.as_ref().map_or(0, |_| 1 + 4)
        + self.collection_rate_minerals.as_ref().map_or(0, |_| 1 + 4)
        + self.collection_rate_vespene.as_ref().map_or(0, |_| 1 + 4)
        + self.spent_minerals.as_ref().map_or(0, |_| 1 + 4)
        + self.spent_vespene.as_ref().map_or(0, |_| 1 + 4)
        + self.food_used.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.killed_minerals.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.killed_vespene.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.lost_minerals.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.lost_vespene.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.friendly_fire_minerals.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.friendly_fire_vespene.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.used_minerals.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.used_vespene.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.total_used_minerals.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.total_used_vespene.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.total_damage_dealt.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.total_damage_taken.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.total_healed.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.current_apm.as_ref().map_or(0, |_| 2 + 4)
        + self.current_effective_apm.as_ref().map_or(0, |_| 2 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.idle_production_time { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.idle_worker_time { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.total_value_units { w.write_with_tag(29, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.total_value_structures { w.write_with_tag(37, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.killed_value_units { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.killed_value_structures { w.write_with_tag(53, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.collected_minerals { w.write_with_tag(61, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.collected_vespene { w.write_with_tag(69, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.collection_rate_minerals { w.write_with_tag(77, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.collection_rate_vespene { w.write_with_tag(85, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.spent_minerals { w.write_with_tag(93, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.spent_vespene { w.write_with_tag(101, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.food_used { w.write_with_tag(106, |w| w.write_message(s))?; }
        if let Some(ref s) = self.killed_minerals { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.killed_vespene { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.lost_minerals { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.lost_vespene { w.write_with_tag(138, |w| w.write_message(s))?; }
        if let Some(ref s) = self.friendly_fire_minerals { w.write_with_tag(146, |w| w.write_message(s))?; }
        if let Some(ref s) = self.friendly_fire_vespene { w.write_with_tag(154, |w| w.write_message(s))?; }
        if let Some(ref s) = self.used_minerals { w.write_with_tag(162, |w| w.write_message(s))?; }
        if let Some(ref s) = self.used_vespene { w.write_with_tag(170, |w| w.write_message(s))?; }
        if let Some(ref s) = self.total_used_minerals { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.total_used_vespene { w.write_with_tag(186, |w| w.write_message(s))?; }
        if let Some(ref s) = self.total_damage_dealt { w.write_with_tag(194, |w| w.write_message(s))?; }
        if let Some(ref s) = self.total_damage_taken { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.total_healed { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.current_apm { w.write_with_tag(221, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.current_effective_apm { w.write_with_tag(229, |w| w.write_float(*s))?; }
        Ok(())
    }
}

