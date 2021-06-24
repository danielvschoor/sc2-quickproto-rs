// Automatically generated rust module for 'debug.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DebugGameState {
    show_map = 1,
    control_enemy = 2,
    food = 3,
    free = 4,
    all_resources = 5,
    god = 6,
    minerals = 7,
    gas = 8,
    cooldown = 9,
    tech_tree = 10,
    upgrade = 11,
    fast_build = 12,
}

impl Default for DebugGameState {
    fn default() -> Self {
        DebugGameState::show_map
    }
}

impl From<i32> for DebugGameState {
    fn from(i: i32) -> Self {
        match i {
            1 => DebugGameState::show_map,
            2 => DebugGameState::control_enemy,
            3 => DebugGameState::food,
            4 => DebugGameState::free,
            5 => DebugGameState::all_resources,
            6 => DebugGameState::god,
            7 => DebugGameState::minerals,
            8 => DebugGameState::gas,
            9 => DebugGameState::cooldown,
            10 => DebugGameState::tech_tree,
            11 => DebugGameState::upgrade,
            12 => DebugGameState::fast_build,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DebugGameState {
    fn from(s: &'a str) -> Self {
        match s {
            "show_map" => DebugGameState::show_map,
            "control_enemy" => DebugGameState::control_enemy,
            "food" => DebugGameState::food,
            "free" => DebugGameState::free,
            "all_resources" => DebugGameState::all_resources,
            "god" => DebugGameState::god,
            "minerals" => DebugGameState::minerals,
            "gas" => DebugGameState::gas,
            "cooldown" => DebugGameState::cooldown,
            "tech_tree" => DebugGameState::tech_tree,
            "upgrade" => DebugGameState::upgrade,
            "fast_build" => DebugGameState::fast_build,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugCommand<'a> {
    pub command: mod_DebugCommand::OneOfcommand<'a>,
}

impl<'a> MessageRead<'a> for DebugCommand<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.command = mod_DebugCommand::OneOfcommand::draw(r.read_message::<DebugDraw>(bytes)?),
                Ok(16) => msg.command = mod_DebugCommand::OneOfcommand::game_state(r.read_enum(bytes)?),
                Ok(26) => msg.command = mod_DebugCommand::OneOfcommand::create_unit(r.read_message::<DebugCreateUnit>(bytes)?),
                Ok(34) => msg.command = mod_DebugCommand::OneOfcommand::kill_unit(r.read_message::<DebugKillUnit>(bytes)?),
                Ok(42) => msg.command = mod_DebugCommand::OneOfcommand::test_process(r.read_message::<DebugTestProcess>(bytes)?),
                Ok(50) => msg.command = mod_DebugCommand::OneOfcommand::score(r.read_message::<DebugSetScore>(bytes)?),
                Ok(58) => msg.command = mod_DebugCommand::OneOfcommand::end_game(r.read_message::<DebugEndGame>(bytes)?),
                Ok(66) => msg.command = mod_DebugCommand::OneOfcommand::unit_value(r.read_message::<DebugSetUnitValue>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DebugCommand<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.command {
            mod_DebugCommand::OneOfcommand::draw(ref m) => 1 + sizeof_len((m).get_size()),
            mod_DebugCommand::OneOfcommand::game_state(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_DebugCommand::OneOfcommand::create_unit(ref m) => 1 + sizeof_len((m).get_size()),
            mod_DebugCommand::OneOfcommand::kill_unit(ref m) => 1 + sizeof_len((m).get_size()),
            mod_DebugCommand::OneOfcommand::test_process(ref m) => 1 + sizeof_len((m).get_size()),
            mod_DebugCommand::OneOfcommand::score(ref m) => 1 + sizeof_len((m).get_size()),
            mod_DebugCommand::OneOfcommand::end_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_DebugCommand::OneOfcommand::unit_value(ref m) => 1 + sizeof_len((m).get_size()),
            mod_DebugCommand::OneOfcommand::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.command {            mod_DebugCommand::OneOfcommand::draw(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_DebugCommand::OneOfcommand::game_state(ref m) => { w.write_with_tag(16, |w| w.write_enum(*m as i32))? },
            mod_DebugCommand::OneOfcommand::create_unit(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_DebugCommand::OneOfcommand::kill_unit(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_DebugCommand::OneOfcommand::test_process(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_DebugCommand::OneOfcommand::score(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_DebugCommand::OneOfcommand::end_game(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_DebugCommand::OneOfcommand::unit_value(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_DebugCommand::OneOfcommand::None => {},
    }        Ok(())
    }
}

pub mod mod_DebugCommand {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfcommand<'a> {
    draw(DebugDraw<'a>),
    game_state(DebugGameState),
    create_unit(DebugCreateUnit),
    kill_unit(DebugKillUnit),
    test_process(DebugTestProcess),
    score(DebugSetScore),
    end_game(DebugEndGame),
    unit_value(DebugSetUnitValue),
    None,
}

impl<'a> Default for OneOfcommand<'a> {
    fn default() -> Self {
        OneOfcommand::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugDraw<'a> {
    pub text: Vec<DebugText<'a>>,
    pub lines: Vec<DebugLine>,
    pub boxes: Vec<DebugBox>,
    pub spheres: Vec<DebugSphere>,
}

impl<'a> MessageRead<'a> for DebugDraw<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text.push(r.read_message::<DebugText>(bytes)?),
                Ok(18) => msg.lines.push(r.read_message::<DebugLine>(bytes)?),
                Ok(26) => msg.boxes.push(r.read_message::<DebugBox>(bytes)?),
                Ok(34) => msg.spheres.push(r.read_message::<DebugSphere>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DebugDraw<'a> {
    fn get_size(&self) -> usize {
        0
        + self.text.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.lines.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.boxes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.spheres.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.text { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.lines { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.boxes { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.spheres { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Line {
    pub p0: Option<Point>,
    pub p1: Option<Point>,
}

impl<'a> MessageRead<'a> for Line {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.p0 = Some(r.read_message::<Point>(bytes)?),
                Ok(18) => msg.p1 = Some(r.read_message::<Point>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Line {
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
pub struct Color {
    pub r: Option<u32>,
    pub g: Option<u32>,
    pub b: Option<u32>,
}

impl<'a> MessageRead<'a> for Color {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.r = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.g = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.b = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Color {
    fn get_size(&self) -> usize {
        0
        + self.r.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.g.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.r { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.g { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.b { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugText<'a> {
    pub color: Option<Color>,
    pub text: Option<Cow<'a, str>>,
    pub virtual_pos: Option<Point>,
    pub world_pos: Option<Point>,
    pub size: Option<u32>,
}

impl<'a> MessageRead<'a> for DebugText<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.color = Some(r.read_message::<Color>(bytes)?),
                Ok(18) => msg.text = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.virtual_pos = Some(r.read_message::<Point>(bytes)?),
                Ok(34) => msg.world_pos = Some(r.read_message::<Point>(bytes)?),
                Ok(40) => msg.size = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DebugText<'a> {
    fn get_size(&self) -> usize {
        0
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.virtual_pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.world_pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.color { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.text { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.virtual_pos { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.world_pos { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugLine {
    pub color: Option<Color>,
    pub line: Option<Line>,
}

impl<'a> MessageRead<'a> for DebugLine {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.color = Some(r.read_message::<Color>(bytes)?),
                Ok(18) => msg.line = Some(r.read_message::<Line>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugLine {
    fn get_size(&self) -> usize {
        0
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.line.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.color { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.line { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugBox {
    pub color: Option<Color>,
    pub min: Option<Point>,
    pub max: Option<Point>,
}

impl<'a> MessageRead<'a> for DebugBox {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.color = Some(r.read_message::<Color>(bytes)?),
                Ok(18) => msg.min = Some(r.read_message::<Point>(bytes)?),
                Ok(26) => msg.max = Some(r.read_message::<Point>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugBox {
    fn get_size(&self) -> usize {
        0
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.min.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.max.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.color { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.min { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.max { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugSphere {
    pub color: Option<Color>,
    pub p: Option<Point>,
    pub r: Option<f32>,
}

impl<'a> MessageRead<'a> for DebugSphere {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.color = Some(r.read_message::<Color>(bytes)?),
                Ok(18) => msg.p = Some(r.read_message::<Point>(bytes)?),
                Ok(29) => msg.r = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugSphere {
    fn get_size(&self) -> usize {
        0
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.p.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.r.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.color { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.p { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.r { w.write_with_tag(29, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugCreateUnit {
    pub unit_type: Option<u32>,
    pub owner: Option<i32>,
    pub pos: Option<Point2D>,
    pub quantity: Option<u32>,
}

impl<'a> MessageRead<'a> for DebugCreateUnit {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_type = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.owner = Some(r.read_int32(bytes)?),
                Ok(26) => msg.pos = Some(r.read_message::<Point2D>(bytes)?),
                Ok(32) => msg.quantity = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugCreateUnit {
    fn get_size(&self) -> usize {
        0
        + self.unit_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.owner.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.quantity.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_type { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.owner { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.pos { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.quantity { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugKillUnit {
    pub tag: Vec<u64>,
}

impl<'a> MessageRead<'a> for DebugKillUnit {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.tag.push(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugKillUnit {
    fn get_size(&self) -> usize {
        0
        + self.tag.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tag { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugTestProcess {
    pub test: Option<mod_DebugTestProcess::Test>,
    pub delay_ms: Option<i32>,
}

impl<'a> MessageRead<'a> for DebugTestProcess {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.test = Some(r.read_enum(bytes)?),
                Ok(16) => msg.delay_ms = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugTestProcess {
    fn get_size(&self) -> usize {
        0
        + self.test.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.delay_ms.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.test { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.delay_ms { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

pub mod mod_DebugTestProcess {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Test {
    hang = 1,
    crash = 2,
    exit = 3,
}

impl Default for Test {
    fn default() -> Self {
        Test::hang
    }
}

impl From<i32> for Test {
    fn from(i: i32) -> Self {
        match i {
            1 => Test::hang,
            2 => Test::crash,
            3 => Test::exit,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Test {
    fn from(s: &'a str) -> Self {
        match s {
            "hang" => Test::hang,
            "crash" => Test::crash,
            "exit" => Test::exit,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugSetScore {
    pub score: Option<f32>,
}

impl<'a> MessageRead<'a> for DebugSetScore {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.score = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugSetScore {
    fn get_size(&self) -> usize {
        0
        + self.score.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.score { w.write_with_tag(13, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugEndGame {
    pub end_result: Option<mod_DebugEndGame::EndResult>,
}

impl<'a> MessageRead<'a> for DebugEndGame {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.end_result = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugEndGame {
    fn get_size(&self) -> usize {
        0
        + self.end_result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.end_result { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_DebugEndGame {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EndResult {
    Surrender = 1,
    DeclareVictory = 2,
}

impl Default for EndResult {
    fn default() -> Self {
        EndResult::Surrender
    }
}

impl From<i32> for EndResult {
    fn from(i: i32) -> Self {
        match i {
            1 => EndResult::Surrender,
            2 => EndResult::DeclareVictory,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for EndResult {
    fn from(s: &'a str) -> Self {
        match s {
            "Surrender" => EndResult::Surrender,
            "DeclareVictory" => EndResult::DeclareVictory,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DebugSetUnitValue {
    pub unit_value: Option<mod_DebugSetUnitValue::UnitValue>,
    pub value: Option<f32>,
    pub unit_tag: Option<u64>,
}

impl<'a> MessageRead<'a> for DebugSetUnitValue {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_value = Some(r.read_enum(bytes)?),
                Ok(21) => msg.value = Some(r.read_float(bytes)?),
                Ok(24) => msg.unit_tag = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DebugSetUnitValue {
    fn get_size(&self) -> usize {
        0
        + self.unit_value.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.value.as_ref().map_or(0, |_| 1 + 4)
        + self.unit_tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_value { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.value { w.write_with_tag(21, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.unit_tag { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

pub mod mod_DebugSetUnitValue {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnitValue {
    Energy = 1,
    Life = 2,
    Shields = 3,
}

impl Default for UnitValue {
    fn default() -> Self {
        UnitValue::Energy
    }
}

impl From<i32> for UnitValue {
    fn from(i: i32) -> Self {
        match i {
            1 => UnitValue::Energy,
            2 => UnitValue::Life,
            3 => UnitValue::Shields,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for UnitValue {
    fn from(s: &'a str) -> Self {
        match s {
            "Energy" => UnitValue::Energy,
            "Life" => UnitValue::Life,
            "Shields" => UnitValue::Shields,
            _ => Self::default(),
        }
    }
}

}

