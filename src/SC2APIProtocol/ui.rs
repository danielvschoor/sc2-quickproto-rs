// Automatically generated rust module for 'ui.proto' file

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
pub struct ObservationUI {
    pub groups: Vec<ControlGroup>,
    pub panel: mod_ObservationUI::OneOfpanel,
}

impl<'a> MessageRead<'a> for ObservationUI {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.groups.push(r.read_message::<ControlGroup>(bytes)?),
                Ok(18) => msg.panel = mod_ObservationUI::OneOfpanel::single(r.read_message::<SinglePanel>(bytes)?),
                Ok(26) => msg.panel = mod_ObservationUI::OneOfpanel::multi(r.read_message::<MultiPanel>(bytes)?),
                Ok(34) => msg.panel = mod_ObservationUI::OneOfpanel::cargo(r.read_message::<CargoPanel>(bytes)?),
                Ok(42) => msg.panel = mod_ObservationUI::OneOfpanel::production(r.read_message::<ProductionPanel>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ObservationUI {
    fn get_size(&self) -> usize {
        0
        + self.groups.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + match self.panel {
            mod_ObservationUI::OneOfpanel::single(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObservationUI::OneOfpanel::multi(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObservationUI::OneOfpanel::cargo(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObservationUI::OneOfpanel::production(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObservationUI::OneOfpanel::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.groups { w.write_with_tag(10, |w| w.write_message(s))?; }
        match self.panel {            mod_ObservationUI::OneOfpanel::single(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ObservationUI::OneOfpanel::multi(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_ObservationUI::OneOfpanel::cargo(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_ObservationUI::OneOfpanel::production(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_ObservationUI::OneOfpanel::None => {},
    }        Ok(())
    }
}

pub mod mod_ObservationUI {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpanel {
    single(SinglePanel),
    multi(MultiPanel),
    cargo(CargoPanel),
    production(ProductionPanel),
    None,
}

impl Default for OneOfpanel {
    fn default() -> Self {
        OneOfpanel::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ControlGroup {
    pub control_group_index: Option<u32>,
    pub leader_unit_type: Option<u32>,
    pub count: Option<u32>,
}

impl<'a> MessageRead<'a> for ControlGroup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.control_group_index = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.leader_unit_type = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.count = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ControlGroup {
    fn get_size(&self) -> usize {
        0
        + self.control_group_index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.leader_unit_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.control_group_index { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.leader_unit_type { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.count { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnitInfo {
    pub unit_type: Option<u32>,
    pub player_relative: Option<u32>,
    pub health: Option<i32>,
    pub shields: Option<i32>,
    pub energy: Option<i32>,
    pub transport_slots_taken: Option<i32>,
    pub build_progress: Option<f32>,
    pub add_on: Option<Box<UnitInfo>>,
    pub max_health: Option<i32>,
    pub max_shields: Option<i32>,
    pub max_energy: Option<i32>,
}

impl<'a> MessageRead<'a> for UnitInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_type = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.player_relative = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.health = Some(r.read_int32(bytes)?),
                Ok(32) => msg.shields = Some(r.read_int32(bytes)?),
                Ok(40) => msg.energy = Some(r.read_int32(bytes)?),
                Ok(48) => msg.transport_slots_taken = Some(r.read_int32(bytes)?),
                Ok(61) => msg.build_progress = Some(r.read_float(bytes)?),
                Ok(66) => msg.add_on = Some(Box::new(r.read_message::<UnitInfo>(bytes)?)),
                Ok(72) => msg.max_health = Some(r.read_int32(bytes)?),
                Ok(80) => msg.max_shields = Some(r.read_int32(bytes)?),
                Ok(88) => msg.max_energy = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UnitInfo {
    fn get_size(&self) -> usize {
        0
        + self.unit_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.player_relative.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.health.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.shields.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.energy.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.transport_slots_taken.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.build_progress.as_ref().map_or(0, |_| 1 + 4)
        + self.add_on.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.max_health.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.max_shields.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.max_energy.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_type { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.player_relative { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.health { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.shields { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.energy { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.transport_slots_taken { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.build_progress { w.write_with_tag(61, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.add_on { w.write_with_tag(66, |w| w.write_message(&**s))?; }
        if let Some(ref s) = self.max_health { w.write_with_tag(72, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.max_shields { w.write_with_tag(80, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.max_energy { w.write_with_tag(88, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SinglePanel {
    pub unit: Option<UnitInfo>,
    pub attack_upgrade_level: Option<i32>,
    pub armor_upgrade_level: Option<i32>,
    pub shield_upgrade_level: Option<i32>,
    pub buffs: Vec<i32>,
}

impl<'a> MessageRead<'a> for SinglePanel {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.unit = Some(r.read_message::<UnitInfo>(bytes)?),
                Ok(16) => msg.attack_upgrade_level = Some(r.read_int32(bytes)?),
                Ok(24) => msg.armor_upgrade_level = Some(r.read_int32(bytes)?),
                Ok(32) => msg.shield_upgrade_level = Some(r.read_int32(bytes)?),
                Ok(40) => msg.buffs.push(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SinglePanel {
    fn get_size(&self) -> usize {
        0
        + self.unit.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.attack_upgrade_level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.armor_upgrade_level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.shield_upgrade_level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.buffs.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.attack_upgrade_level { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.armor_upgrade_level { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.shield_upgrade_level { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        for s in &self.buffs { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MultiPanel {
    pub units: Vec<UnitInfo>,
}

impl<'a> MessageRead<'a> for MultiPanel {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.units.push(r.read_message::<UnitInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MultiPanel {
    fn get_size(&self) -> usize {
        0
        + self.units.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.units { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CargoPanel {
    pub unit: Option<UnitInfo>,
    pub passengers: Vec<UnitInfo>,
    pub slots_available: Option<i32>,
}

impl<'a> MessageRead<'a> for CargoPanel {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.unit = Some(r.read_message::<UnitInfo>(bytes)?),
                Ok(18) => msg.passengers.push(r.read_message::<UnitInfo>(bytes)?),
                Ok(24) => msg.slots_available = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CargoPanel {
    fn get_size(&self) -> usize {
        0
        + self.unit.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.passengers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.slots_available.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.passengers { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.slots_available { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BuildItem {
    pub ability_id: Option<u32>,
    pub build_progress: Option<f32>,
}

impl<'a> MessageRead<'a> for BuildItem {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_uint32(bytes)?),
                Ok(21) => msg.build_progress = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BuildItem {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.build_progress.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.build_progress { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProductionPanel {
    pub unit: Option<UnitInfo>,
    pub build_queue: Vec<UnitInfo>,
    pub production_queue: Vec<BuildItem>,
}

impl<'a> MessageRead<'a> for ProductionPanel {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.unit = Some(r.read_message::<UnitInfo>(bytes)?),
                Ok(18) => msg.build_queue.push(r.read_message::<UnitInfo>(bytes)?),
                Ok(26) => msg.production_queue.push(r.read_message::<BuildItem>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProductionPanel {
    fn get_size(&self) -> usize {
        0
        + self.unit.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.build_queue.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.production_queue.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.build_queue { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.production_queue { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionUI {
    pub action: mod_ActionUI::OneOfaction,
}

impl<'a> MessageRead<'a> for ActionUI {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action = mod_ActionUI::OneOfaction::control_group(r.read_message::<ActionControlGroup>(bytes)?),
                Ok(18) => msg.action = mod_ActionUI::OneOfaction::select_army(r.read_message::<ActionSelectArmy>(bytes)?),
                Ok(26) => msg.action = mod_ActionUI::OneOfaction::select_warp_gates(r.read_message::<ActionSelectWarpGates>(bytes)?),
                Ok(34) => msg.action = mod_ActionUI::OneOfaction::select_larva(r.read_message::<ActionSelectLarva>(bytes)?),
                Ok(42) => msg.action = mod_ActionUI::OneOfaction::select_idle_worker(r.read_message::<ActionSelectIdleWorker>(bytes)?),
                Ok(50) => msg.action = mod_ActionUI::OneOfaction::multi_panel(r.read_message::<ActionMultiPanel>(bytes)?),
                Ok(58) => msg.action = mod_ActionUI::OneOfaction::cargo_panel(r.read_message::<ActionCargoPanelUnload>(bytes)?),
                Ok(66) => msg.action = mod_ActionUI::OneOfaction::production_panel(r.read_message::<ActionProductionPanelRemoveFromQueue>(bytes)?),
                Ok(74) => msg.action = mod_ActionUI::OneOfaction::toggle_autocast(r.read_message::<ActionToggleAutocast>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionUI {
    fn get_size(&self) -> usize {
        0
        + match self.action {
            mod_ActionUI::OneOfaction::control_group(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::select_army(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::select_warp_gates(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::select_larva(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::select_idle_worker(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::multi_panel(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::cargo_panel(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::production_panel(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::toggle_autocast(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ActionUI::OneOfaction::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.action {            mod_ActionUI::OneOfaction::control_group(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::select_army(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::select_warp_gates(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::select_larva(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::select_idle_worker(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::multi_panel(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::cargo_panel(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::production_panel(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::toggle_autocast(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            mod_ActionUI::OneOfaction::None => {},
    }        Ok(())
    }
}

pub mod mod_ActionUI {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfaction {
    control_group(ActionControlGroup),
    select_army(ActionSelectArmy),
    select_warp_gates(ActionSelectWarpGates),
    select_larva(ActionSelectLarva),
    select_idle_worker(ActionSelectIdleWorker),
    multi_panel(ActionMultiPanel),
    cargo_panel(ActionCargoPanelUnload),
    production_panel(ActionProductionPanelRemoveFromQueue),
    toggle_autocast(ActionToggleAutocast),
    None,
}

impl Default for OneOfaction {
    fn default() -> Self {
        OneOfaction::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionControlGroup {
    pub action: Option<mod_ActionControlGroup::ControlGroupAction>,
    pub control_group_index: Option<u32>,
}

impl<'a> MessageRead<'a> for ActionControlGroup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.action = Some(r.read_enum(bytes)?),
                Ok(16) => msg.control_group_index = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionControlGroup {
    fn get_size(&self) -> usize {
        0
        + self.action.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.control_group_index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.action { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.control_group_index { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

pub mod mod_ActionControlGroup {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ControlGroupAction {
    Recall = 1,
    Set = 2,
    Append = 3,
    SetAndSteal = 4,
    AppendAndSteal = 5,
}

impl Default for ControlGroupAction {
    fn default() -> Self {
        ControlGroupAction::Recall
    }
}

impl From<i32> for ControlGroupAction {
    fn from(i: i32) -> Self {
        match i {
            1 => ControlGroupAction::Recall,
            2 => ControlGroupAction::Set,
            3 => ControlGroupAction::Append,
            4 => ControlGroupAction::SetAndSteal,
            5 => ControlGroupAction::AppendAndSteal,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ControlGroupAction {
    fn from(s: &'a str) -> Self {
        match s {
            "Recall" => ControlGroupAction::Recall,
            "Set" => ControlGroupAction::Set,
            "Append" => ControlGroupAction::Append,
            "SetAndSteal" => ControlGroupAction::SetAndSteal,
            "AppendAndSteal" => ControlGroupAction::AppendAndSteal,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSelectArmy {
    pub selection_add: Option<bool>,
}

impl<'a> MessageRead<'a> for ActionSelectArmy {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.selection_add = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSelectArmy {
    fn get_size(&self) -> usize {
        0
        + self.selection_add.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.selection_add { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSelectWarpGates {
    pub selection_add: Option<bool>,
}

impl<'a> MessageRead<'a> for ActionSelectWarpGates {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.selection_add = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSelectWarpGates {
    fn get_size(&self) -> usize {
        0
        + self.selection_add.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.selection_add { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSelectLarva { }

impl<'a> MessageRead<'a> for ActionSelectLarva {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ActionSelectLarva { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionSelectIdleWorker {
    pub type_pb: Option<mod_ActionSelectIdleWorker::Type>,
}

impl<'a> MessageRead<'a> for ActionSelectIdleWorker {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionSelectIdleWorker {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_ActionSelectIdleWorker {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Set = 1,
    Add = 2,
    All = 3,
    AddAll = 4,
}

impl Default for Type {
    fn default() -> Self {
        Type::Set
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            1 => Type::Set,
            2 => Type::Add,
            3 => Type::All,
            4 => Type::AddAll,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "Set" => Type::Set,
            "Add" => Type::Add,
            "All" => Type::All,
            "AddAll" => Type::AddAll,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionMultiPanel {
    pub type_pb: Option<mod_ActionMultiPanel::Type>,
    pub unit_index: Option<i32>,
}

impl<'a> MessageRead<'a> for ActionMultiPanel {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(16) => msg.unit_index = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionMultiPanel {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unit_index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.unit_index { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

pub mod mod_ActionMultiPanel {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    SingleSelect = 1,
    DeselectUnit = 2,
    SelectAllOfType = 3,
    DeselectAllOfType = 4,
}

impl Default for Type {
    fn default() -> Self {
        Type::SingleSelect
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            1 => Type::SingleSelect,
            2 => Type::DeselectUnit,
            3 => Type::SelectAllOfType,
            4 => Type::DeselectAllOfType,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "SingleSelect" => Type::SingleSelect,
            "DeselectUnit" => Type::DeselectUnit,
            "SelectAllOfType" => Type::SelectAllOfType,
            "DeselectAllOfType" => Type::DeselectAllOfType,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionCargoPanelUnload {
    pub unit_index: Option<i32>,
}

impl<'a> MessageRead<'a> for ActionCargoPanelUnload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_index = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionCargoPanelUnload {
    fn get_size(&self) -> usize {
        0
        + self.unit_index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_index { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionProductionPanelRemoveFromQueue {
    pub unit_index: Option<i32>,
}

impl<'a> MessageRead<'a> for ActionProductionPanelRemoveFromQueue {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_index = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionProductionPanelRemoveFromQueue {
    fn get_size(&self) -> usize {
        0
        + self.unit_index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_index { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionToggleAutocast {
    pub ability_id: Option<i32>,
}

impl<'a> MessageRead<'a> for ActionToggleAutocast {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionToggleAutocast {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

