// Automatically generated rust module for 'spatial.proto' file

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
use crate::SC2APIProtocol::common::{ImageData, PointI};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObservationFeatureLayer<'a> {
    pub renders: Option<FeatureLayers<'a>>,
    pub minimap_renders: Option<FeatureLayersMinimap<'a>>,
}

impl<'a> MessageRead<'a> for ObservationFeatureLayer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.renders = Some(r.read_message::<FeatureLayers>(bytes)?),
                Ok(18) => msg.minimap_renders = Some(r.read_message::<FeatureLayersMinimap>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObservationFeatureLayer<'a> {
    fn get_size(&self) -> usize {
        0
        + self.renders.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.minimap_renders.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.renders { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.minimap_renders { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeatureLayers<'a> {
    pub height_map: Option<ImageData<'a>>,
    pub visibility_map: Option<ImageData<'a>>,
    pub creep: Option<ImageData<'a>>,
    pub power: Option<ImageData<'a>>,
    pub player_id: Option<ImageData<'a>>,
    pub unit_type: Option<ImageData<'a>>,
    pub selected: Option<ImageData<'a>>,
    pub unit_hit_points: Option<ImageData<'a>>,
    pub unit_hit_points_ratio: Option<ImageData<'a>>,
    pub unit_energy: Option<ImageData<'a>>,
    pub unit_energy_ratio: Option<ImageData<'a>>,
    pub unit_shields: Option<ImageData<'a>>,
    pub unit_shields_ratio: Option<ImageData<'a>>,
    pub player_relative: Option<ImageData<'a>>,
    pub unit_density_aa: Option<ImageData<'a>>,
    pub unit_density: Option<ImageData<'a>>,
    pub effects: Option<ImageData<'a>>,
    pub hallucinations: Option<ImageData<'a>>,
    pub cloaked: Option<ImageData<'a>>,
    pub blip: Option<ImageData<'a>>,
    pub buffs: Option<ImageData<'a>>,
    pub buff_duration: Option<ImageData<'a>>,
    pub active: Option<ImageData<'a>>,
    pub build_progress: Option<ImageData<'a>>,
    pub buildable: Option<ImageData<'a>>,
    pub pathable: Option<ImageData<'a>>,
    pub placeholder: Option<ImageData<'a>>,
}

impl<'a> MessageRead<'a> for FeatureLayers<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.height_map = Some(r.read_message::<ImageData>(bytes)?),
                Ok(18) => msg.visibility_map = Some(r.read_message::<ImageData>(bytes)?),
                Ok(26) => msg.creep = Some(r.read_message::<ImageData>(bytes)?),
                Ok(34) => msg.power = Some(r.read_message::<ImageData>(bytes)?),
                Ok(42) => msg.player_id = Some(r.read_message::<ImageData>(bytes)?),
                Ok(50) => msg.unit_type = Some(r.read_message::<ImageData>(bytes)?),
                Ok(58) => msg.selected = Some(r.read_message::<ImageData>(bytes)?),
                Ok(66) => msg.unit_hit_points = Some(r.read_message::<ImageData>(bytes)?),
                Ok(138) => msg.unit_hit_points_ratio = Some(r.read_message::<ImageData>(bytes)?),
                Ok(74) => msg.unit_energy = Some(r.read_message::<ImageData>(bytes)?),
                Ok(146) => msg.unit_energy_ratio = Some(r.read_message::<ImageData>(bytes)?),
                Ok(82) => msg.unit_shields = Some(r.read_message::<ImageData>(bytes)?),
                Ok(154) => msg.unit_shields_ratio = Some(r.read_message::<ImageData>(bytes)?),
                Ok(90) => msg.player_relative = Some(r.read_message::<ImageData>(bytes)?),
                Ok(114) => msg.unit_density_aa = Some(r.read_message::<ImageData>(bytes)?),
                Ok(122) => msg.unit_density = Some(r.read_message::<ImageData>(bytes)?),
                Ok(162) => msg.effects = Some(r.read_message::<ImageData>(bytes)?),
                Ok(170) => msg.hallucinations = Some(r.read_message::<ImageData>(bytes)?),
                Ok(178) => msg.cloaked = Some(r.read_message::<ImageData>(bytes)?),
                Ok(186) => msg.blip = Some(r.read_message::<ImageData>(bytes)?),
                Ok(194) => msg.buffs = Some(r.read_message::<ImageData>(bytes)?),
                Ok(210) => msg.buff_duration = Some(r.read_message::<ImageData>(bytes)?),
                Ok(202) => msg.active = Some(r.read_message::<ImageData>(bytes)?),
                Ok(218) => msg.build_progress = Some(r.read_message::<ImageData>(bytes)?),
                Ok(226) => msg.buildable = Some(r.read_message::<ImageData>(bytes)?),
                Ok(234) => msg.pathable = Some(r.read_message::<ImageData>(bytes)?),
                Ok(242) => msg.placeholder = Some(r.read_message::<ImageData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FeatureLayers<'a> {
    fn get_size(&self) -> usize {
        0
        + self.height_map.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.visibility_map.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.creep.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.power.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_type.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.selected.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_hit_points.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_hit_points_ratio.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.unit_energy.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_energy_ratio.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.unit_shields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_shields_ratio.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.player_relative.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_density_aa.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_density.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.effects.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.hallucinations.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.cloaked.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.blip.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.buffs.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.buff_duration.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.active.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.build_progress.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.buildable.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.pathable.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.placeholder.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.height_map { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.visibility_map { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.creep { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.power { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.player_id { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_type { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.selected { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_hit_points { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_hit_points_ratio { w.write_with_tag(138, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_energy { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_energy_ratio { w.write_with_tag(146, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_shields { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_shields_ratio { w.write_with_tag(154, |w| w.write_message(s))?; }
        if let Some(ref s) = self.player_relative { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_density_aa { w.write_with_tag(114, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_density { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.effects { w.write_with_tag(162, |w| w.write_message(s))?; }
        if let Some(ref s) = self.hallucinations { w.write_with_tag(170, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cloaked { w.write_with_tag(178, |w| w.write_message(s))?; }
        if let Some(ref s) = self.blip { w.write_with_tag(186, |w| w.write_message(s))?; }
        if let Some(ref s) = self.buffs { w.write_with_tag(194, |w| w.write_message(s))?; }
        if let Some(ref s) = self.buff_duration { w.write_with_tag(210, |w| w.write_message(s))?; }
        if let Some(ref s) = self.active { w.write_with_tag(202, |w| w.write_message(s))?; }
        if let Some(ref s) = self.build_progress { w.write_with_tag(218, |w| w.write_message(s))?; }
        if let Some(ref s) = self.buildable { w.write_with_tag(226, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pathable { w.write_with_tag(234, |w| w.write_message(s))?; }
        if let Some(ref s) = self.placeholder { w.write_with_tag(242, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeatureLayersMinimap<'a> {
    pub height_map: Option<ImageData<'a>>,
    pub visibility_map: Option<ImageData<'a>>,
    pub creep: Option<ImageData<'a>>,
    pub camera: Option<ImageData<'a>>,
    pub player_id: Option<ImageData<'a>>,
    pub player_relative: Option<ImageData<'a>>,
    pub selected: Option<ImageData<'a>>,
    pub alerts: Option<ImageData<'a>>,
    pub buildable: Option<ImageData<'a>>,
    pub pathable: Option<ImageData<'a>>,
    pub unit_type: Option<ImageData<'a>>,
}

impl<'a> MessageRead<'a> for FeatureLayersMinimap<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.height_map = Some(r.read_message::<ImageData>(bytes)?),
                Ok(18) => msg.visibility_map = Some(r.read_message::<ImageData>(bytes)?),
                Ok(26) => msg.creep = Some(r.read_message::<ImageData>(bytes)?),
                Ok(34) => msg.camera = Some(r.read_message::<ImageData>(bytes)?),
                Ok(42) => msg.player_id = Some(r.read_message::<ImageData>(bytes)?),
                Ok(50) => msg.player_relative = Some(r.read_message::<ImageData>(bytes)?),
                Ok(58) => msg.selected = Some(r.read_message::<ImageData>(bytes)?),
                Ok(74) => msg.alerts = Some(r.read_message::<ImageData>(bytes)?),
                Ok(82) => msg.buildable = Some(r.read_message::<ImageData>(bytes)?),
                Ok(90) => msg.pathable = Some(r.read_message::<ImageData>(bytes)?),
                Ok(66) => msg.unit_type = Some(r.read_message::<ImageData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FeatureLayersMinimap<'a> {
    fn get_size(&self) -> usize {
        0
        + self.height_map.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.visibility_map.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.creep.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.camera.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.player_relative.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.selected.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.alerts.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.buildable.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.pathable.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.unit_type.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.height_map { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.visibility_map { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.creep { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.camera { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.player_id { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.player_relative { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.selected { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.alerts { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.buildable { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.pathable { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.unit_type { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObservationRender<'a> {
    pub map: Option<ImageData<'a>>,
    pub minimap: Option<ImageData<'a>>,
}

impl<'a> MessageRead<'a> for ObservationRender<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.map = Some(r.read_message::<ImageData>(bytes)?),
                Ok(18) => msg.minimap = Some(r.read_message::<ImageData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObservationRender<'a> {
    fn get_size(&self) -> usize {
        0
        + self.map.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.minimap.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.map { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.minimap { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSpatial {
    pub action: mod_ActionSpatial::OneOfaction,
}

impl<'a> MessageRead<'a> for ActionSpatial {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action = mod_ActionSpatial::OneOfaction::unit_command(r.read_message::<ActionSpatialUnitCommand>(bytes)?),
                Ok(18) => msg.action = mod_ActionSpatial::OneOfaction::camera_move(r.read_message::<ActionSpatialCameraMove>(bytes)?),
                Ok(26) => msg.action = mod_ActionSpatial::OneOfaction::unit_selection_point(r.read_message::<ActionSpatialUnitSelectionPoint>(bytes)?),
                Ok(34) => msg.action = mod_ActionSpatial::OneOfaction::unit_selection_rect(r.read_message::<ActionSpatialUnitSelectionRect>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSpatial {
    fn get_size(&self) -> usize {
        0
        + match self.action {
            mod_ActionSpatial::OneOfaction::unit_command(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionSpatial::OneOfaction::camera_move(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionSpatial::OneOfaction::unit_selection_point(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionSpatial::OneOfaction::unit_selection_rect(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionSpatial::OneOfaction::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.action {            mod_ActionSpatial::OneOfaction::unit_command(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_ActionSpatial::OneOfaction::camera_move(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ActionSpatial::OneOfaction::unit_selection_point(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_ActionSpatial::OneOfaction::unit_selection_rect(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_ActionSpatial::OneOfaction::None => {},
    }        Ok(())
    }
}

pub mod mod_ActionSpatial {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfaction {
    unit_command(ActionSpatialUnitCommand),
    camera_move(ActionSpatialCameraMove),
    unit_selection_point(ActionSpatialUnitSelectionPoint),
    unit_selection_rect(ActionSpatialUnitSelectionRect),
    None,
}

impl Default for OneOfaction {
    fn default() -> Self {
        OneOfaction::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSpatialUnitCommand {
    pub ability_id: Option<i32>,
    pub queue_command: Option<bool>,
    pub target: mod_ActionSpatialUnitCommand::OneOftarget,
}

impl<'a> MessageRead<'a> for ActionSpatialUnitCommand {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_int32(bytes)?),
                Ok(32) => msg.queue_command = Some(r.read_bool(bytes)?),
                Ok(18) => msg.target = mod_ActionSpatialUnitCommand::OneOftarget::target_screen_coord(r.read_message::<PointI>(bytes)?),
                Ok(26) => msg.target = mod_ActionSpatialUnitCommand::OneOftarget::target_minimap_coord(r.read_message::<PointI>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSpatialUnitCommand {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.queue_command.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + match self.target {
            mod_ActionSpatialUnitCommand::OneOftarget::target_screen_coord(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionSpatialUnitCommand::OneOftarget::target_minimap_coord(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionSpatialUnitCommand::OneOftarget::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.queue_command { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        match self.target {            mod_ActionSpatialUnitCommand::OneOftarget::target_screen_coord(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ActionSpatialUnitCommand::OneOftarget::target_minimap_coord(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_ActionSpatialUnitCommand::OneOftarget::None => {},
    }        Ok(())
    }
}

pub mod mod_ActionSpatialUnitCommand {

use super::*;
    use crate::SC2APIProtocol::common::PointI;

    #[derive(Debug, PartialEq, Clone)]
pub enum OneOftarget {
    target_screen_coord(PointI),
    target_minimap_coord(PointI),
    None,
}

impl Default for OneOftarget {
    fn default() -> Self {
        OneOftarget::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSpatialCameraMove {
    pub center_minimap: Option<PointI>,
}

impl<'a> MessageRead<'a> for ActionSpatialCameraMove {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.center_minimap = Some(r.read_message::<PointI>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSpatialCameraMove {
    fn get_size(&self) -> usize {
        0
        + self.center_minimap.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.center_minimap { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSpatialUnitSelectionPoint {
    pub selection_screen_coord: Option<PointI>,
    pub type_pb: Option<mod_ActionSpatialUnitSelectionPoint::Type>,
}

impl<'a> MessageRead<'a> for ActionSpatialUnitSelectionPoint {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.selection_screen_coord = Some(r.read_message::<PointI>(bytes)?),
                Ok(16) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSpatialUnitSelectionPoint {
    fn get_size(&self) -> usize {
        0
        + self.selection_screen_coord.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.selection_screen_coord { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_ActionSpatialUnitSelectionPoint {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Select = 1,
    Toggle = 2,
    AllType = 3,
    AddAllType = 4,
}

impl Default for Type {
    fn default() -> Self {
        Type::Select
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            1 => Type::Select,
            2 => Type::Toggle,
            3 => Type::AllType,
            4 => Type::AddAllType,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "Select" => Type::Select,
            "Toggle" => Type::Toggle,
            "AllType" => Type::AllType,
            "AddAllType" => Type::AddAllType,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSpatialUnitSelectionRect {
    pub selection_screen_coord: Vec<common::RectangleI>,
    pub selection_add: Option<bool>,
}

impl<'a> MessageRead<'a> for ActionSpatialUnitSelectionRect {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.selection_screen_coord.push(r.read_message::<common::RectangleI>(bytes)?),
                Ok(16) => msg.selection_add = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSpatialUnitSelectionRect {
    fn get_size(&self) -> usize {
        0
        + self.selection_screen_coord.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.selection_add.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.selection_screen_coord { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.selection_add { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

