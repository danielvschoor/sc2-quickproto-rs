// Automatically generated rust module for 'sc2api.proto' file

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
use crate::SC2APIProtocol::query::*;
use crate::SC2APIProtocol::error::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    launched = 1,
    init_game = 2,
    in_game = 3,
    in_replay = 4,
    ended = 5,
    quit = 6,
    unknown = 99,
}

impl Default for Status {
    fn default() -> Self {
        Status::launched
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            1 => Status::launched,
            2 => Status::init_game,
            3 => Status::in_game,
            4 => Status::in_replay,
            5 => Status::ended,
            6 => Status::quit,
            99 => Status::unknown,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "launched" => Status::launched,
            "init_game" => Status::init_game,
            "in_game" => Status::in_game,
            "in_replay" => Status::in_replay,
            "ended" => Status::ended,
            "quit" => Status::quit,
            "unknown" => Status::unknown,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Difficulty {
    VeryEasy = 1,
    Easy = 2,
    Medium = 3,
    MediumHard = 4,
    Hard = 5,
    Harder = 6,
    VeryHard = 7,
    CheatVision = 8,
    CheatMoney = 9,
    CheatInsane = 10,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::VeryEasy
    }
}

impl From<i32> for Difficulty {
    fn from(i: i32) -> Self {
        match i {
            1 => Difficulty::VeryEasy,
            2 => Difficulty::Easy,
            3 => Difficulty::Medium,
            4 => Difficulty::MediumHard,
            5 => Difficulty::Hard,
            6 => Difficulty::Harder,
            7 => Difficulty::VeryHard,
            8 => Difficulty::CheatVision,
            9 => Difficulty::CheatMoney,
            10 => Difficulty::CheatInsane,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Difficulty {
    fn from(s: &'a str) -> Self {
        match s {
            "VeryEasy" => Difficulty::VeryEasy,
            "Easy" => Difficulty::Easy,
            "Medium" => Difficulty::Medium,
            "MediumHard" => Difficulty::MediumHard,
            "Hard" => Difficulty::Hard,
            "Harder" => Difficulty::Harder,
            "VeryHard" => Difficulty::VeryHard,
            "CheatVision" => Difficulty::CheatVision,
            "CheatMoney" => Difficulty::CheatMoney,
            "CheatInsane" => Difficulty::CheatInsane,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerType {
    Participant = 1,
    Computer = 2,
    Observer = 3,
}

impl Default for PlayerType {
    fn default() -> Self {
        PlayerType::Participant
    }
}

impl From<i32> for PlayerType {
    fn from(i: i32) -> Self {
        match i {
            1 => PlayerType::Participant,
            2 => PlayerType::Computer,
            3 => PlayerType::Observer,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PlayerType {
    fn from(s: &'a str) -> Self {
        match s {
            "Participant" => PlayerType::Participant,
            "Computer" => PlayerType::Computer,
            "Observer" => PlayerType::Observer,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AIBuild {
    RandomBuild = 1,
    Rush = 2,
    Timing = 3,
    Power = 4,
    Macro = 5,
    Air = 6,
}

impl Default for AIBuild {
    fn default() -> Self {
        AIBuild::RandomBuild
    }
}

impl From<i32> for AIBuild {
    fn from(i: i32) -> Self {
        match i {
            1 => AIBuild::RandomBuild,
            2 => AIBuild::Rush,
            3 => AIBuild::Timing,
            4 => AIBuild::Power,
            5 => AIBuild::Macro,
            6 => AIBuild::Air,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for AIBuild {
    fn from(s: &'a str) -> Self {
        match s {
            "RandomBuild" => AIBuild::RandomBuild,
            "Rush" => AIBuild::Rush,
            "Timing" => AIBuild::Timing,
            "Power" => AIBuild::Power,
            "Macro" => AIBuild::Macro,
            "Air" => AIBuild::Air,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alert {
    AlertError = 3,
    AddOnComplete = 4,
    BuildingComplete = 5,
    BuildingUnderAttack = 6,
    LarvaHatched = 7,
    MergeComplete = 8,
    MineralsExhausted = 9,
    MorphComplete = 10,
    MothershipComplete = 11,
    MULEExpired = 12,
    NuclearLaunchDetected = 1,
    NukeComplete = 13,
    NydusWormDetected = 2,
    ResearchComplete = 14,
    TrainError = 15,
    TrainUnitComplete = 16,
    TrainWorkerComplete = 17,
    TransformationComplete = 18,
    UnitUnderAttack = 19,
    UpgradeComplete = 20,
    VespeneExhausted = 21,
    WarpInComplete = 22,
}

impl Default for Alert {
    fn default() -> Self {
        Alert::AlertError
    }
}

impl From<i32> for Alert {
    fn from(i: i32) -> Self {
        match i {
            3 => Alert::AlertError,
            4 => Alert::AddOnComplete,
            5 => Alert::BuildingComplete,
            6 => Alert::BuildingUnderAttack,
            7 => Alert::LarvaHatched,
            8 => Alert::MergeComplete,
            9 => Alert::MineralsExhausted,
            10 => Alert::MorphComplete,
            11 => Alert::MothershipComplete,
            12 => Alert::MULEExpired,
            1 => Alert::NuclearLaunchDetected,
            13 => Alert::NukeComplete,
            2 => Alert::NydusWormDetected,
            14 => Alert::ResearchComplete,
            15 => Alert::TrainError,
            16 => Alert::TrainUnitComplete,
            17 => Alert::TrainWorkerComplete,
            18 => Alert::TransformationComplete,
            19 => Alert::UnitUnderAttack,
            20 => Alert::UpgradeComplete,
            21 => Alert::VespeneExhausted,
            22 => Alert::WarpInComplete,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Alert {
    fn from(s: &'a str) -> Self {
        match s {
            "AlertError" => Alert::AlertError,
            "AddOnComplete" => Alert::AddOnComplete,
            "BuildingComplete" => Alert::BuildingComplete,
            "BuildingUnderAttack" => Alert::BuildingUnderAttack,
            "LarvaHatched" => Alert::LarvaHatched,
            "MergeComplete" => Alert::MergeComplete,
            "MineralsExhausted" => Alert::MineralsExhausted,
            "MorphComplete" => Alert::MorphComplete,
            "MothershipComplete" => Alert::MothershipComplete,
            "MULEExpired" => Alert::MULEExpired,
            "NuclearLaunchDetected" => Alert::NuclearLaunchDetected,
            "NukeComplete" => Alert::NukeComplete,
            "NydusWormDetected" => Alert::NydusWormDetected,
            "ResearchComplete" => Alert::ResearchComplete,
            "TrainError" => Alert::TrainError,
            "TrainUnitComplete" => Alert::TrainUnitComplete,
            "TrainWorkerComplete" => Alert::TrainWorkerComplete,
            "TransformationComplete" => Alert::TransformationComplete,
            "UnitUnderAttack" => Alert::UnitUnderAttack,
            "UpgradeComplete" => Alert::UpgradeComplete,
            "VespeneExhausted" => Alert::VespeneExhausted,
            "WarpInComplete" => Alert::WarpInComplete,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Result_pb {
    Victory = 1,
    Defeat = 2,
    Tie = 3,
    Undecided = 4,
}

impl Default for Result_pb {
    fn default() -> Self {
        Result_pb::Victory
    }
}

impl From<i32> for Result_pb {
    fn from(i: i32) -> Self {
        match i {
            1 => Result_pb::Victory,
            2 => Result_pb::Defeat,
            3 => Result_pb::Tie,
            4 => Result_pb::Undecided,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Result_pb {
    fn from(s: &'a str) -> Self {
        match s {
            "Victory" => Result_pb::Victory,
            "Defeat" => Result_pb::Defeat,
            "Tie" => Result_pb::Tie,
            "Undecided" => Result_pb::Undecided,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Request<'a> {
    pub id: Option<u32>,
    pub request: mod_Request::OneOfrequest<'a>,
}

impl<'a> MessageRead<'a> for Request<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(776) => msg.id = Some(r.read_uint32(bytes)?),
                Ok(10) => msg.request = mod_Request::OneOfrequest::create_game(r.read_message::<RequestCreateGame>(bytes)?),
                Ok(18) => msg.request = mod_Request::OneOfrequest::join_game(r.read_message::<RequestJoinGame>(bytes)?),
                Ok(26) => msg.request = mod_Request::OneOfrequest::restart_game(r.read_message::<RequestRestartGame>(bytes)?),
                Ok(34) => msg.request = mod_Request::OneOfrequest::start_replay(r.read_message::<RequestStartReplay>(bytes)?),
                Ok(42) => msg.request = mod_Request::OneOfrequest::leave_game(r.read_message::<RequestLeaveGame>(bytes)?),
                Ok(50) => msg.request = mod_Request::OneOfrequest::quick_save(r.read_message::<RequestQuickSave>(bytes)?),
                Ok(58) => msg.request = mod_Request::OneOfrequest::quick_load(r.read_message::<RequestQuickLoad>(bytes)?),
                Ok(66) => msg.request = mod_Request::OneOfrequest::quit(r.read_message::<RequestQuit>(bytes)?),
                Ok(74) => msg.request = mod_Request::OneOfrequest::game_info(r.read_message::<RequestGameInfo>(bytes)?),
                Ok(82) => msg.request = mod_Request::OneOfrequest::observation(r.read_message::<RequestObservation>(bytes)?),
                Ok(90) => msg.request = mod_Request::OneOfrequest::action(r.read_message::<RequestAction>(bytes)?),
                Ok(170) => msg.request = mod_Request::OneOfrequest::obs_action(r.read_message::<RequestObserverAction>(bytes)?),
                Ok(98) => msg.request = mod_Request::OneOfrequest::step(r.read_message::<RequestStep>(bytes)?),
                Ok(106) => msg.request = mod_Request::OneOfrequest::data(r.read_message::<RequestData>(bytes)?),
                Ok(114) => msg.request = mod_Request::OneOfrequest::query(r.read_message::<RequestQuery>(bytes)?),
                Ok(122) => msg.request = mod_Request::OneOfrequest::save_replay(r.read_message::<RequestSaveReplay>(bytes)?),
                Ok(178) => msg.request = mod_Request::OneOfrequest::map_command(r.read_message::<RequestMapCommand>(bytes)?),
                Ok(130) => msg.request = mod_Request::OneOfrequest::replay_info(r.read_message::<RequestReplayInfo>(bytes)?),
                Ok(138) => msg.request = mod_Request::OneOfrequest::available_maps(r.read_message::<RequestAvailableMaps>(bytes)?),
                Ok(146) => msg.request = mod_Request::OneOfrequest::save_map(r.read_message::<RequestSaveMap>(bytes)?),
                Ok(154) => msg.request = mod_Request::OneOfrequest::ping(r.read_message::<RequestPing>(bytes)?),
                Ok(162) => msg.request = mod_Request::OneOfrequest::debug(r.read_message::<RequestDebug>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Request<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + match self.request {
            mod_Request::OneOfrequest::create_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::join_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::restart_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::start_replay(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::leave_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::quick_save(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::quick_load(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::quit(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::game_info(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::observation(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::action(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::obs_action(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::step(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::data(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::query(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::save_replay(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::map_command(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::replay_info(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::available_maps(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::save_map(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::ping(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::debug(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(776, |w| w.write_uint32(*s))?; }
        match self.request {            mod_Request::OneOfrequest::create_game(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::join_game(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::restart_game(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::start_replay(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::leave_game(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::quick_save(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::quick_load(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::quit(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::game_info(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::observation(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::action(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::obs_action(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::step(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::data(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::query(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::save_replay(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::map_command(ref m) => { w.write_with_tag(178, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::replay_info(ref m) => { w.write_with_tag(130, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::available_maps(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::save_map(ref m) => { w.write_with_tag(146, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::ping(ref m) => { w.write_with_tag(154, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::debug(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::None => {},
    }        Ok(())
    }
}

pub mod mod_Request {

use super::*;
    use crate::SC2APIProtocol::query::RequestQuery;

    #[derive(Debug, PartialEq, Clone)]
pub enum OneOfrequest<'a> {
    create_game(RequestCreateGame<'a>),
    join_game(RequestJoinGame<'a>),
    restart_game(RequestRestartGame),
    start_replay(RequestStartReplay<'a>),
    leave_game(RequestLeaveGame),
    quick_save(RequestQuickSave),
    quick_load(RequestQuickLoad),
    quit(RequestQuit),
    game_info(RequestGameInfo),
    observation(RequestObservation),
    action(RequestAction<'a>),
    obs_action(RequestObserverAction),
    step(RequestStep),
    data(RequestData),
    query(RequestQuery),
    save_replay(RequestSaveReplay),
    map_command(RequestMapCommand<'a>),
    replay_info(RequestReplayInfo<'a>),
    available_maps(RequestAvailableMaps),
    save_map(RequestSaveMap<'a>),
    ping(RequestPing),
    debug(RequestDebug<'a>),
    None,
}

impl<'a> Default for OneOfrequest<'a> {
    fn default() -> Self {
        OneOfrequest::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response<'a> {
    pub id: Option<u32>,
    pub error: Vec<Cow<'a, str>>,
    pub status: Option<Status>,
    pub response: mod_Response::OneOfresponse<'a>,
}

impl<'a> MessageRead<'a> for Response<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(776) => msg.id = Some(r.read_uint32(bytes)?),
                Ok(786) => msg.error.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(792) => msg.status = Some(r.read_enum(bytes)?),
                Ok(10) => msg.response = mod_Response::OneOfresponse::create_game(r.read_message::<ResponseCreateGame>(bytes)?),
                Ok(18) => msg.response = mod_Response::OneOfresponse::join_game(r.read_message::<ResponseJoinGame>(bytes)?),
                Ok(26) => msg.response = mod_Response::OneOfresponse::restart_game(r.read_message::<ResponseRestartGame>(bytes)?),
                Ok(34) => msg.response = mod_Response::OneOfresponse::start_replay(r.read_message::<ResponseStartReplay>(bytes)?),
                Ok(42) => msg.response = mod_Response::OneOfresponse::leave_game(r.read_message::<ResponseLeaveGame>(bytes)?),
                Ok(50) => msg.response = mod_Response::OneOfresponse::quick_save(r.read_message::<ResponseQuickSave>(bytes)?),
                Ok(58) => msg.response = mod_Response::OneOfresponse::quick_load(r.read_message::<ResponseQuickLoad>(bytes)?),
                Ok(66) => msg.response = mod_Response::OneOfresponse::quit(r.read_message::<ResponseQuit>(bytes)?),
                Ok(74) => msg.response = mod_Response::OneOfresponse::game_info(r.read_message::<ResponseGameInfo>(bytes)?),
                Ok(82) => msg.response = mod_Response::OneOfresponse::observation(r.read_message::<ResponseObservation>(bytes)?),
                Ok(90) => msg.response = mod_Response::OneOfresponse::action(r.read_message::<ResponseAction>(bytes)?),
                Ok(170) => msg.response = mod_Response::OneOfresponse::obs_action(r.read_message::<ResponseObserverAction>(bytes)?),
                Ok(98) => msg.response = mod_Response::OneOfresponse::step(r.read_message::<ResponseStep>(bytes)?),
                Ok(106) => msg.response = mod_Response::OneOfresponse::data(r.read_message::<ResponseData>(bytes)?),
                Ok(114) => msg.response = mod_Response::OneOfresponse::query(r.read_message::<ResponseQuery>(bytes)?),
                Ok(122) => msg.response = mod_Response::OneOfresponse::save_replay(r.read_message::<ResponseSaveReplay>(bytes)?),
                Ok(130) => msg.response = mod_Response::OneOfresponse::replay_info(r.read_message::<ResponseReplayInfo>(bytes)?),
                Ok(138) => msg.response = mod_Response::OneOfresponse::available_maps(r.read_message::<ResponseAvailableMaps>(bytes)?),
                Ok(146) => msg.response = mod_Response::OneOfresponse::save_map(r.read_message::<ResponseSaveMap>(bytes)?),
                Ok(178) => msg.response = mod_Response::OneOfresponse::map_command(r.read_message::<ResponseMapCommand>(bytes)?),
                Ok(154) => msg.response = mod_Response::OneOfresponse::ping(r.read_message::<ResponsePing>(bytes)?),
                Ok(162) => msg.response = mod_Response::OneOfresponse::debug(r.read_message::<ResponseDebug>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Response<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.error.iter().map(|s| 2 + sizeof_len((s).len())).sum::<usize>()
        + self.status.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + match self.response {
            mod_Response::OneOfresponse::create_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::join_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::restart_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::start_replay(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::leave_game(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::quick_save(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::quick_load(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::quit(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::game_info(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::observation(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::action(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::obs_action(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::step(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::data(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::query(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::save_replay(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::replay_info(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::available_maps(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::save_map(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::map_command(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::ping(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::debug(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(776, |w| w.write_uint32(*s))?; }
        for s in &self.error { w.write_with_tag(786, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(792, |w| w.write_enum(*s as i32))?; }
        match self.response {            mod_Response::OneOfresponse::create_game(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::join_game(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::restart_game(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::start_replay(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::leave_game(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::quick_save(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::quick_load(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::quit(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::game_info(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::observation(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::action(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::obs_action(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::step(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::data(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::query(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::save_replay(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::replay_info(ref m) => { w.write_with_tag(130, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::available_maps(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::save_map(ref m) => { w.write_with_tag(146, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::map_command(ref m) => { w.write_with_tag(178, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::ping(ref m) => { w.write_with_tag(154, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::debug(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::None => {},
    }        Ok(())
    }
}

pub mod mod_Response {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfresponse<'a> {
    create_game(ResponseCreateGame<'a>),
    join_game(ResponseJoinGame<'a>),
    restart_game(ResponseRestartGame<'a>),
    start_replay(ResponseStartReplay<'a>),
    leave_game(ResponseLeaveGame),
    quick_save(ResponseQuickSave),
    quick_load(ResponseQuickLoad),
    quit(ResponseQuit),
    game_info(ResponseGameInfo<'a>),
    observation(ResponseObservation<'a>),
    action(ResponseAction),
    obs_action(ResponseObserverAction),
    step(ResponseStep),
    data(ResponseData<'a>),
    query(ResponseQuery),
    save_replay(ResponseSaveReplay<'a>),
    replay_info(ResponseReplayInfo<'a>),
    available_maps(ResponseAvailableMaps<'a>),
    save_map(ResponseSaveMap),
    map_command(ResponseMapCommand<'a>),
    ping(ResponsePing<'a>),
    debug(ResponseDebug),
    None,
}

impl<'a> Default for OneOfresponse<'a> {
    fn default() -> Self {
        OneOfresponse::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestCreateGame<'a> {
    pub player_setup: Vec<PlayerSetup<'a>>,
    pub disable_fog: Option<bool>,
    pub random_seed: Option<u32>,
    pub realtime: Option<bool>,
    pub Map: mod_RequestCreateGame::OneOfMap<'a>,
}

impl<'a> MessageRead<'a> for RequestCreateGame<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.player_setup.push(r.read_message::<PlayerSetup>(bytes)?),
                Ok(32) => msg.disable_fog = Some(r.read_bool(bytes)?),
                Ok(40) => msg.random_seed = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.realtime = Some(r.read_bool(bytes)?),
                Ok(10) => msg.Map = mod_RequestCreateGame::OneOfMap::local_map(r.read_message::<LocalMap>(bytes)?),
                Ok(18) => msg.Map = mod_RequestCreateGame::OneOfMap::battlenet_map_name(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestCreateGame<'a> {
    fn get_size(&self) -> usize {
        0
        + self.player_setup.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.disable_fog.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.random_seed.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.realtime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + match self.Map {
            mod_RequestCreateGame::OneOfMap::local_map(ref m) => 1 + sizeof_len((m).get_size()),
            mod_RequestCreateGame::OneOfMap::battlenet_map_name(ref m) => 1 + sizeof_len((m).len()),
            mod_RequestCreateGame::OneOfMap::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.player_setup { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.disable_fog { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.random_seed { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.realtime { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        match self.Map {            mod_RequestCreateGame::OneOfMap::local_map(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_RequestCreateGame::OneOfMap::battlenet_map_name(ref m) => { w.write_with_tag(18, |w| w.write_string(&**m))? },
            mod_RequestCreateGame::OneOfMap::None => {},
    }        Ok(())
    }
}

pub mod mod_RequestCreateGame {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfMap<'a> {
    local_map(LocalMap<'a>),
    battlenet_map_name(Cow<'a, str>),
    None,
}

impl<'a> Default for OneOfMap<'a> {
    fn default() -> Self {
        OneOfMap::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LocalMap<'a> {
    pub map_path: Option<Cow<'a, str>>,
    pub map_data: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for LocalMap<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.map_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.map_data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LocalMap<'a> {
    fn get_size(&self) -> usize {
        0
        + self.map_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.map_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.map_path { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.map_data { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseCreateGame<'a> {
    pub error: Option<mod_ResponseCreateGame::Error>,
    pub error_details: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ResponseCreateGame<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error = Some(r.read_enum(bytes)?),
                Ok(18) => msg.error_details = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseCreateGame<'a> {
    fn get_size(&self) -> usize {
        0
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error_details.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.error { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.error_details { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_ResponseCreateGame {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    MissingMap = 1,
    InvalidMapPath = 2,
    InvalidMapData = 3,
    InvalidMapName = 4,
    InvalidMapHandle = 5,
    MissingPlayerSetup = 6,
    InvalidPlayerSetup = 7,
    MultiplayerUnsupported = 8,
}

impl Default for Error {
    fn default() -> Self {
        Error::MissingMap
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            1 => Error::MissingMap,
            2 => Error::InvalidMapPath,
            3 => Error::InvalidMapData,
            4 => Error::InvalidMapName,
            5 => Error::InvalidMapHandle,
            6 => Error::MissingPlayerSetup,
            7 => Error::InvalidPlayerSetup,
            8 => Error::MultiplayerUnsupported,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        match s {
            "MissingMap" => Error::MissingMap,
            "InvalidMapPath" => Error::InvalidMapPath,
            "InvalidMapData" => Error::InvalidMapData,
            "InvalidMapName" => Error::InvalidMapName,
            "InvalidMapHandle" => Error::InvalidMapHandle,
            "MissingPlayerSetup" => Error::MissingPlayerSetup,
            "InvalidPlayerSetup" => Error::InvalidPlayerSetup,
            "MultiplayerUnsupported" => Error::MultiplayerUnsupported,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestJoinGame<'a> {
    pub options: Option<InterfaceOptions>,
    pub server_ports: Option<PortSet>,
    pub client_ports: Vec<PortSet>,
    pub shared_port: Option<i32>,
    pub player_name: Option<Cow<'a, str>>,
    pub host_ip: Option<Cow<'a, str>>,
    pub participation: mod_RequestJoinGame::OneOfparticipation,
}

impl<'a> MessageRead<'a> for RequestJoinGame<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.options = Some(r.read_message::<InterfaceOptions>(bytes)?),
                Ok(34) => msg.server_ports = Some(r.read_message::<PortSet>(bytes)?),
                Ok(42) => msg.client_ports.push(r.read_message::<PortSet>(bytes)?),
                Ok(48) => msg.shared_port = Some(r.read_int32(bytes)?),
                Ok(58) => msg.player_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.host_ip = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(8) => msg.participation = mod_RequestJoinGame::OneOfparticipation::race(r.read_enum(bytes)?),
                Ok(16) => msg.participation = mod_RequestJoinGame::OneOfparticipation::observed_player_id(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestJoinGame<'a> {
    fn get_size(&self) -> usize {
        0
        + self.options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.server_ports.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.client_ports.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.shared_port.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.player_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.host_ip.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + match self.participation {
            mod_RequestJoinGame::OneOfparticipation::race(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_RequestJoinGame::OneOfparticipation::observed_player_id(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_RequestJoinGame::OneOfparticipation::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.options { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.server_ports { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.client_ports { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shared_port { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.player_name { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.host_ip { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        match self.participation {            mod_RequestJoinGame::OneOfparticipation::race(ref m) => { w.write_with_tag(8, |w| w.write_enum(*m as i32))? },
            mod_RequestJoinGame::OneOfparticipation::observed_player_id(ref m) => { w.write_with_tag(16, |w| w.write_uint32(*m))? },
            mod_RequestJoinGame::OneOfparticipation::None => {},
    }        Ok(())
    }
}

pub mod mod_RequestJoinGame {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfparticipation {
    race(Race),
    observed_player_id(u32),
    None,
}

impl Default for OneOfparticipation {
    fn default() -> Self {
        OneOfparticipation::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PortSet {
    pub game_port: Option<i32>,
    pub base_port: Option<i32>,
}

impl<'a> MessageRead<'a> for PortSet {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.game_port = Some(r.read_int32(bytes)?),
                Ok(16) => msg.base_port = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PortSet {
    fn get_size(&self) -> usize {
        0
        + self.game_port.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.base_port.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.game_port { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.base_port { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseJoinGame<'a> {
    pub player_id: Option<u32>,
    pub error: Option<mod_ResponseJoinGame::Error>,
    pub error_details: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ResponseJoinGame<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.error = Some(r.read_enum(bytes)?),
                Ok(26) => msg.error_details = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseJoinGame<'a> {
    fn get_size(&self) -> usize {
        0
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error_details.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.error { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.error_details { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_ResponseJoinGame {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    MissingParticipation = 1,
    InvalidObservedPlayerId = 2,
    MissingOptions = 3,
    MissingPorts = 4,
    GameFull = 5,
    LaunchError = 6,
    FeatureUnsupported = 7,
    NoSpaceForUser = 8,
    MapDoesNotExist = 9,
    CannotOpenMap = 10,
    ChecksumError = 11,
    NetworkError = 12,
    OtherError = 13,
}

impl Default for Error {
    fn default() -> Self {
        Error::MissingParticipation
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            1 => Error::MissingParticipation,
            2 => Error::InvalidObservedPlayerId,
            3 => Error::MissingOptions,
            4 => Error::MissingPorts,
            5 => Error::GameFull,
            6 => Error::LaunchError,
            7 => Error::FeatureUnsupported,
            8 => Error::NoSpaceForUser,
            9 => Error::MapDoesNotExist,
            10 => Error::CannotOpenMap,
            11 => Error::ChecksumError,
            12 => Error::NetworkError,
            13 => Error::OtherError,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        match s {
            "MissingParticipation" => Error::MissingParticipation,
            "InvalidObservedPlayerId" => Error::InvalidObservedPlayerId,
            "MissingOptions" => Error::MissingOptions,
            "MissingPorts" => Error::MissingPorts,
            "GameFull" => Error::GameFull,
            "LaunchError" => Error::LaunchError,
            "FeatureUnsupported" => Error::FeatureUnsupported,
            "NoSpaceForUser" => Error::NoSpaceForUser,
            "MapDoesNotExist" => Error::MapDoesNotExist,
            "CannotOpenMap" => Error::CannotOpenMap,
            "ChecksumError" => Error::ChecksumError,
            "NetworkError" => Error::NetworkError,
            "OtherError" => Error::OtherError,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestRestartGame { }

impl<'a> MessageRead<'a> for RequestRestartGame {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestRestartGame { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseRestartGame<'a> {
    pub error: Option<mod_ResponseRestartGame::Error>,
    pub error_details: Option<Cow<'a, str>>,
    pub need_hard_reset: Option<bool>,
}

impl<'a> MessageRead<'a> for ResponseRestartGame<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error = Some(r.read_enum(bytes)?),
                Ok(18) => msg.error_details = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.need_hard_reset = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseRestartGame<'a> {
    fn get_size(&self) -> usize {
        0
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error_details.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.need_hard_reset.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.error { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.error_details { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.need_hard_reset { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

pub mod mod_ResponseRestartGame {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    LaunchError = 1,
}

impl Default for Error {
    fn default() -> Self {
        Error::LaunchError
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            1 => Error::LaunchError,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        match s {
            "LaunchError" => Error::LaunchError,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestStartReplay<'a> {
    pub map_data: Option<Cow<'a, [u8]>>,
    pub observed_player_id: Option<i32>,
    pub options: Option<InterfaceOptions>,
    pub disable_fog: Option<bool>,
    pub realtime: Option<bool>,
    pub record_replay: Option<bool>,
    pub replay: mod_RequestStartReplay::OneOfreplay<'a>,
}

impl<'a> MessageRead<'a> for RequestStartReplay<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(50) => msg.map_data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.observed_player_id = Some(r.read_int32(bytes)?),
                Ok(26) => msg.options = Some(r.read_message::<InterfaceOptions>(bytes)?),
                Ok(32) => msg.disable_fog = Some(r.read_bool(bytes)?),
                Ok(56) => msg.realtime = Some(r.read_bool(bytes)?),
                Ok(64) => msg.record_replay = Some(r.read_bool(bytes)?),
                Ok(10) => msg.replay = mod_RequestStartReplay::OneOfreplay::replay_path(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.replay = mod_RequestStartReplay::OneOfreplay::replay_data(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestStartReplay<'a> {
    fn get_size(&self) -> usize {
        0
        + self.map_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.observed_player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.disable_fog.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.realtime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.record_replay.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + match self.replay {
            mod_RequestStartReplay::OneOfreplay::replay_path(ref m) => 1 + sizeof_len((m).len()),
            mod_RequestStartReplay::OneOfreplay::replay_data(ref m) => 1 + sizeof_len((m).len()),
            mod_RequestStartReplay::OneOfreplay::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.map_data { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.observed_player_id { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.options { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.disable_fog { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.realtime { w.write_with_tag(56, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.record_replay { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        match self.replay {            mod_RequestStartReplay::OneOfreplay::replay_path(ref m) => { w.write_with_tag(10, |w| w.write_string(&**m))? },
            mod_RequestStartReplay::OneOfreplay::replay_data(ref m) => { w.write_with_tag(42, |w| w.write_bytes(&**m))? },
            mod_RequestStartReplay::OneOfreplay::None => {},
    }        Ok(())
    }
}

pub mod mod_RequestStartReplay {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfreplay<'a> {
    replay_path(Cow<'a, str>),
    replay_data(Cow<'a, [u8]>),
    None,
}

impl<'a> Default for OneOfreplay<'a> {
    fn default() -> Self {
        OneOfreplay::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseStartReplay<'a> {
    pub error: Option<mod_ResponseStartReplay::Error>,
    pub error_details: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ResponseStartReplay<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error = Some(r.read_enum(bytes)?),
                Ok(18) => msg.error_details = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseStartReplay<'a> {
    fn get_size(&self) -> usize {
        0
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error_details.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.error { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.error_details { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_ResponseStartReplay {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    MissingReplay = 1,
    InvalidReplayPath = 2,
    InvalidReplayData = 3,
    InvalidMapData = 4,
    InvalidObservedPlayerId = 5,
    MissingOptions = 6,
    LaunchError = 7,
}

impl Default for Error {
    fn default() -> Self {
        Error::MissingReplay
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            1 => Error::MissingReplay,
            2 => Error::InvalidReplayPath,
            3 => Error::InvalidReplayData,
            4 => Error::InvalidMapData,
            5 => Error::InvalidObservedPlayerId,
            6 => Error::MissingOptions,
            7 => Error::LaunchError,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        match s {
            "MissingReplay" => Error::MissingReplay,
            "InvalidReplayPath" => Error::InvalidReplayPath,
            "InvalidReplayData" => Error::InvalidReplayData,
            "InvalidMapData" => Error::InvalidMapData,
            "InvalidObservedPlayerId" => Error::InvalidObservedPlayerId,
            "MissingOptions" => Error::MissingOptions,
            "LaunchError" => Error::LaunchError,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestMapCommand<'a> {
    pub trigger_cmd: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for RequestMapCommand<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.trigger_cmd = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestMapCommand<'a> {
    fn get_size(&self) -> usize {
        0
        + self.trigger_cmd.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.trigger_cmd { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseMapCommand<'a> {
    pub error: Option<mod_ResponseMapCommand::Error>,
    pub error_details: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ResponseMapCommand<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error = Some(r.read_enum(bytes)?),
                Ok(18) => msg.error_details = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseMapCommand<'a> {
    fn get_size(&self) -> usize {
        0
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error_details.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.error { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.error_details { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_ResponseMapCommand {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    NoTriggerError = 1,
}

impl Default for Error {
    fn default() -> Self {
        Error::NoTriggerError
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            1 => Error::NoTriggerError,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        match s {
            "NoTriggerError" => Error::NoTriggerError,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestLeaveGame { }

impl<'a> MessageRead<'a> for RequestLeaveGame {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestLeaveGame { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseLeaveGame { }

impl<'a> MessageRead<'a> for ResponseLeaveGame {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ResponseLeaveGame { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestQuickSave { }

impl<'a> MessageRead<'a> for RequestQuickSave {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestQuickSave { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseQuickSave { }

impl<'a> MessageRead<'a> for ResponseQuickSave {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ResponseQuickSave { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestQuickLoad { }

impl<'a> MessageRead<'a> for RequestQuickLoad {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestQuickLoad { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseQuickLoad { }

impl<'a> MessageRead<'a> for ResponseQuickLoad {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ResponseQuickLoad { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestQuit { }

impl<'a> MessageRead<'a> for RequestQuit {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestQuit { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseQuit { }

impl<'a> MessageRead<'a> for ResponseQuit {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ResponseQuit { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestGameInfo { }

impl<'a> MessageRead<'a> for RequestGameInfo {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestGameInfo { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseGameInfo<'a> {
    pub map_name: Option<Cow<'a, str>>,
    pub mod_names: Vec<Cow<'a, str>>,
    pub local_map_path: Option<Cow<'a, str>>,
    pub player_info: Vec<PlayerInfo<'a>>,
    pub start_raw: Option<StartRaw<'a>>,
    pub options: Option<InterfaceOptions>,
}

impl<'a> MessageRead<'a> for ResponseGameInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.map_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.mod_names.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.local_map_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.player_info.push(r.read_message::<PlayerInfo>(bytes)?),
                Ok(34) => msg.start_raw = Some(r.read_message::<StartRaw>(bytes)?),
                Ok(42) => msg.options = Some(r.read_message::<InterfaceOptions>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseGameInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.map_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.mod_names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.local_map_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.player_info.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.start_raw.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.map_name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.mod_names { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.local_map_path { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.player_info { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.start_raw { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.options { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestObservation {
    pub disable_fog: Option<bool>,
    pub game_loop: Option<u32>,
}

impl<'a> MessageRead<'a> for RequestObservation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.disable_fog = Some(r.read_bool(bytes)?),
                Ok(16) => msg.game_loop = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestObservation {
    fn get_size(&self) -> usize {
        0
        + self.disable_fog.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.game_loop.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.disable_fog { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.game_loop { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseObservation<'a> {
    pub actions: Vec<Action<'a>>,
    pub action_errors: Vec<ActionError>,
    pub observation: Option<Observation<'a>>,
    pub player_result: Vec<PlayerResult>,
    pub chat: Vec<ChatReceived<'a>>,
}

impl<'a> MessageRead<'a> for ResponseObservation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.actions.push(r.read_message::<Action>(bytes)?),
                Ok(18) => msg.action_errors.push(r.read_message::<ActionError>(bytes)?),
                Ok(26) => msg.observation = Some(r.read_message::<Observation>(bytes)?),
                Ok(34) => msg.player_result.push(r.read_message::<PlayerResult>(bytes)?),
                Ok(42) => msg.chat.push(r.read_message::<ChatReceived>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseObservation<'a> {
    fn get_size(&self) -> usize {
        0
        + self.actions.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.action_errors.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.observation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.player_result.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.chat.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.actions { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.action_errors { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.observation { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.player_result { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.chat { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatReceived<'a> {
    pub player_id: Option<u32>,
    pub message: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ChatReceived<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.message = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatReceived<'a> {
    fn get_size(&self) -> usize {
        0
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.message { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestAction<'a> {
    pub actions: Vec<Action<'a>>,
}

impl<'a> MessageRead<'a> for RequestAction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.actions.push(r.read_message::<Action>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestAction<'a> {
    fn get_size(&self) -> usize {
        0
        + self.actions.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.actions { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseAction {
    pub result: Vec<ActionResult>,
}

impl<'a> MessageRead<'a> for ResponseAction {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result.push(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponseAction {
    fn get_size(&self) -> usize {
        0
        + self.result.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.result { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestObserverAction {
    pub actions: Vec<ObserverAction>,
}

impl<'a> MessageRead<'a> for RequestObserverAction {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.actions.push(r.read_message::<ObserverAction>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestObserverAction {
    fn get_size(&self) -> usize {
        0
        + self.actions.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.actions { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseObserverAction { }

impl<'a> MessageRead<'a> for ResponseObserverAction {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ResponseObserverAction { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestStep {
    pub count: Option<u32>,
}

impl<'a> MessageRead<'a> for RequestStep {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.count = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestStep {
    fn get_size(&self) -> usize {
        0
        + self.count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.count { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseStep {
    pub simulation_loop: Option<u32>,
}

impl<'a> MessageRead<'a> for ResponseStep {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.simulation_loop = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponseStep {
    fn get_size(&self) -> usize {
        0
        + self.simulation_loop.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.simulation_loop { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestData {
    pub ability_id: Option<bool>,
    pub unit_type_id: Option<bool>,
    pub upgrade_id: Option<bool>,
    pub buff_id: Option<bool>,
    pub effect_id: Option<bool>,
}

impl<'a> MessageRead<'a> for RequestData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ability_id = Some(r.read_bool(bytes)?),
                Ok(16) => msg.unit_type_id = Some(r.read_bool(bytes)?),
                Ok(24) => msg.upgrade_id = Some(r.read_bool(bytes)?),
                Ok(32) => msg.buff_id = Some(r.read_bool(bytes)?),
                Ok(40) => msg.effect_id = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestData {
    fn get_size(&self) -> usize {
        0
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.unit_type_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.upgrade_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.buff_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.effect_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.ability_id { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.unit_type_id { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.upgrade_id { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.buff_id { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.effect_id { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseData<'a> {
    pub abilities: Vec<AbilityData<'a>>,
    pub units: Vec<UnitTypeData<'a>>,
    pub upgrades: Vec<UpgradeData<'a>>,
    pub buffs: Vec<BuffData<'a>>,
    pub effects: Vec<EffectData<'a>>,
}

impl<'a> MessageRead<'a> for ResponseData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.abilities.push(r.read_message::<AbilityData>(bytes)?),
                Ok(18) => msg.units.push(r.read_message::<UnitTypeData>(bytes)?),
                Ok(26) => msg.upgrades.push(r.read_message::<UpgradeData>(bytes)?),
                Ok(34) => msg.buffs.push(r.read_message::<BuffData>(bytes)?),
                Ok(42) => msg.effects.push(r.read_message::<EffectData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseData<'a> {
    fn get_size(&self) -> usize {
        0
        + self.abilities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.units.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.upgrades.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.buffs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.effects.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.abilities { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.units { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.upgrades { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.buffs { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.effects { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestSaveReplay { }

impl<'a> MessageRead<'a> for RequestSaveReplay {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestSaveReplay { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseSaveReplay<'a> {
    pub data: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for ResponseSaveReplay<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseSaveReplay<'a> {
    fn get_size(&self) -> usize {
        0
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.data { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestReplayInfo<'a> {
    pub download_data: Option<bool>,
    pub replay: mod_RequestReplayInfo::OneOfreplay<'a>,
}

impl<'a> MessageRead<'a> for RequestReplayInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(24) => msg.download_data = Some(r.read_bool(bytes)?),
                Ok(10) => msg.replay = mod_RequestReplayInfo::OneOfreplay::replay_path(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.replay = mod_RequestReplayInfo::OneOfreplay::replay_data(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestReplayInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.download_data.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + match self.replay {
            mod_RequestReplayInfo::OneOfreplay::replay_path(ref m) => 1 + sizeof_len((m).len()),
            mod_RequestReplayInfo::OneOfreplay::replay_data(ref m) => 1 + sizeof_len((m).len()),
            mod_RequestReplayInfo::OneOfreplay::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.download_data { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        match self.replay {            mod_RequestReplayInfo::OneOfreplay::replay_path(ref m) => { w.write_with_tag(10, |w| w.write_string(&**m))? },
            mod_RequestReplayInfo::OneOfreplay::replay_data(ref m) => { w.write_with_tag(18, |w| w.write_bytes(&**m))? },
            mod_RequestReplayInfo::OneOfreplay::None => {},
    }        Ok(())
    }
}

pub mod mod_RequestReplayInfo {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfreplay<'a> {
    replay_path(Cow<'a, str>),
    replay_data(Cow<'a, [u8]>),
    None,
}

impl<'a> Default for OneOfreplay<'a> {
    fn default() -> Self {
        OneOfreplay::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerInfoExtra<'a> {
    pub player_info: Option<PlayerInfo<'a>>,
    pub player_result: Option<PlayerResult>,
    pub player_mmr: Option<i32>,
    pub player_apm: Option<i32>,
}

impl<'a> MessageRead<'a> for PlayerInfoExtra<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.player_info = Some(r.read_message::<PlayerInfo>(bytes)?),
                Ok(18) => msg.player_result = Some(r.read_message::<PlayerResult>(bytes)?),
                Ok(24) => msg.player_mmr = Some(r.read_int32(bytes)?),
                Ok(32) => msg.player_apm = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PlayerInfoExtra<'a> {
    fn get_size(&self) -> usize {
        0
        + self.player_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.player_result.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.player_mmr.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.player_apm.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_info { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.player_result { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.player_mmr { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.player_apm { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseReplayInfo<'a> {
    pub map_name: Option<Cow<'a, str>>,
    pub local_map_path: Option<Cow<'a, str>>,
    pub player_info: Vec<PlayerInfoExtra<'a>>,
    pub game_duration_loops: Option<u32>,
    pub game_duration_seconds: Option<f32>,
    pub game_version: Option<Cow<'a, str>>,
    pub data_version: Option<Cow<'a, str>>,
    pub data_build: Option<u32>,
    pub base_build: Option<u32>,
    pub error: Option<mod_ResponseReplayInfo::Error>,
    pub error_details: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ResponseReplayInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.map_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.local_map_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.player_info.push(r.read_message::<PlayerInfoExtra>(bytes)?),
                Ok(32) => msg.game_duration_loops = Some(r.read_uint32(bytes)?),
                Ok(45) => msg.game_duration_seconds = Some(r.read_float(bytes)?),
                Ok(50) => msg.game_version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.data_version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.data_build = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.base_build = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.error = Some(r.read_enum(bytes)?),
                Ok(82) => msg.error_details = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseReplayInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.map_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.local_map_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.player_info.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.game_duration_loops.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.game_duration_seconds.as_ref().map_or(0, |_| 1 + 4)
        + self.game_version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.data_version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.data_build.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.base_build.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.error_details.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.map_name { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.local_map_path { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.player_info { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.game_duration_loops { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.game_duration_seconds { w.write_with_tag(45, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.game_version { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.data_version { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.data_build { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.base_build { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.error { w.write_with_tag(72, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.error_details { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_ResponseReplayInfo {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    MissingReplay = 1,
    InvalidReplayPath = 2,
    InvalidReplayData = 3,
    ParsingError = 4,
    DownloadError = 5,
}

impl Default for Error {
    fn default() -> Self {
        Error::MissingReplay
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            1 => Error::MissingReplay,
            2 => Error::InvalidReplayPath,
            3 => Error::InvalidReplayData,
            4 => Error::ParsingError,
            5 => Error::DownloadError,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        match s {
            "MissingReplay" => Error::MissingReplay,
            "InvalidReplayPath" => Error::InvalidReplayPath,
            "InvalidReplayData" => Error::InvalidReplayData,
            "ParsingError" => Error::ParsingError,
            "DownloadError" => Error::DownloadError,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestAvailableMaps { }

impl<'a> MessageRead<'a> for RequestAvailableMaps {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestAvailableMaps { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseAvailableMaps<'a> {
    pub local_map_paths: Vec<Cow<'a, str>>,
    pub battlenet_map_names: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ResponseAvailableMaps<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.local_map_paths.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.battlenet_map_names.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseAvailableMaps<'a> {
    fn get_size(&self) -> usize {
        0
        + self.local_map_paths.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.battlenet_map_names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.local_map_paths { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.battlenet_map_names { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestSaveMap<'a> {
    pub map_path: Option<Cow<'a, str>>,
    pub map_data: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for RequestSaveMap<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.map_path = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.map_data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestSaveMap<'a> {
    fn get_size(&self) -> usize {
        0
        + self.map_path.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.map_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.map_path { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.map_data { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseSaveMap {
    pub error: Option<mod_ResponseSaveMap::Error>,
}

impl<'a> MessageRead<'a> for ResponseSaveMap {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ResponseSaveMap {
    fn get_size(&self) -> usize {
        0
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.error { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_ResponseSaveMap {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    InvalidMapData = 1,
}

impl Default for Error {
    fn default() -> Self {
        Error::InvalidMapData
    }
}

impl From<i32> for Error {
    fn from(i: i32) -> Self {
        match i {
            1 => Error::InvalidMapData,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        match s {
            "InvalidMapData" => Error::InvalidMapData,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestPing { }

impl<'a> MessageRead<'a> for RequestPing {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RequestPing { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponsePing<'a> {
    pub game_version: Option<Cow<'a, str>>,
    pub data_version: Option<Cow<'a, str>>,
    pub data_build: Option<u32>,
    pub base_build: Option<u32>,
}

impl<'a> MessageRead<'a> for ResponsePing<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.game_version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.data_version = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.data_build = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.base_build = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponsePing<'a> {
    fn get_size(&self) -> usize {
        0
        + self.game_version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.data_version.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.data_build.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.base_build.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.game_version { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.data_version { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.data_build { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.base_build { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestDebug<'a> {
    pub debug: Vec<DebugCommand<'a>>,
}

impl<'a> MessageRead<'a> for RequestDebug<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.debug.push(r.read_message::<DebugCommand>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestDebug<'a> {
    fn get_size(&self) -> usize {
        0
        + self.debug.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.debug { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseDebug { }

impl<'a> MessageRead<'a> for ResponseDebug {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ResponseDebug { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerSetup<'a> {
    pub type_pb: Option<PlayerType>,
    pub race: Option<Race>,
    pub difficulty: Option<Difficulty>,
    pub player_name: Option<Cow<'a, str>>,
    pub ai_build: Option<AIBuild>,
}

impl<'a> MessageRead<'a> for PlayerSetup<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(16) => msg.race = Some(r.read_enum(bytes)?),
                Ok(24) => msg.difficulty = Some(r.read_enum(bytes)?),
                Ok(34) => msg.player_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.ai_build = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PlayerSetup<'a> {
    fn get_size(&self) -> usize {
        0
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.race.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.difficulty.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.player_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.ai_build.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.type_pb { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.race { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.difficulty { w.write_with_tag(24, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.player_name { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.ai_build { w.write_with_tag(40, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SpatialCameraSetup {
    pub resolution: Option<Size2DI>,
    pub minimap_resolution: Option<Size2DI>,
    pub width: Option<f32>,
    pub crop_to_playable_area: Option<bool>,
    pub allow_cheating_layers: Option<bool>,
}

impl<'a> MessageRead<'a> for SpatialCameraSetup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.resolution = Some(r.read_message::<Size2DI>(bytes)?),
                Ok(26) => msg.minimap_resolution = Some(r.read_message::<Size2DI>(bytes)?),
                Ok(13) => msg.width = Some(r.read_float(bytes)?),
                Ok(32) => msg.crop_to_playable_area = Some(r.read_bool(bytes)?),
                Ok(40) => msg.allow_cheating_layers = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SpatialCameraSetup {
    fn get_size(&self) -> usize {
        0
        + self.resolution.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.minimap_resolution.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.width.as_ref().map_or(0, |_| 1 + 4)
        + self.crop_to_playable_area.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.allow_cheating_layers.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.resolution { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.minimap_resolution { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.width { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.crop_to_playable_area { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.allow_cheating_layers { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct InterfaceOptions {
    pub raw: Option<bool>,
    pub score: Option<bool>,
    pub feature_layer: Option<SpatialCameraSetup>,
    pub render: Option<SpatialCameraSetup>,
    pub show_cloaked: Option<bool>,
    pub show_burrowed_shadows: Option<bool>,
    pub show_placeholders: Option<bool>,
    pub raw_affects_selection: Option<bool>,
    pub raw_crop_to_playable_area: Option<bool>,
}

impl<'a> MessageRead<'a> for InterfaceOptions {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.raw = Some(r.read_bool(bytes)?),
                Ok(16) => msg.score = Some(r.read_bool(bytes)?),
                Ok(26) => msg.feature_layer = Some(r.read_message::<SpatialCameraSetup>(bytes)?),
                Ok(34) => msg.render = Some(r.read_message::<SpatialCameraSetup>(bytes)?),
                Ok(40) => msg.show_cloaked = Some(r.read_bool(bytes)?),
                Ok(72) => msg.show_burrowed_shadows = Some(r.read_bool(bytes)?),
                Ok(64) => msg.show_placeholders = Some(r.read_bool(bytes)?),
                Ok(48) => msg.raw_affects_selection = Some(r.read_bool(bytes)?),
                Ok(56) => msg.raw_crop_to_playable_area = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for InterfaceOptions {
    fn get_size(&self) -> usize {
        0
        + self.raw.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.score.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.feature_layer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.render.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.show_cloaked.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.show_burrowed_shadows.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.show_placeholders.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.raw_affects_selection.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.raw_crop_to_playable_area.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.raw { w.write_with_tag(8, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.score { w.write_with_tag(16, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.feature_layer { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.render { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.show_cloaked { w.write_with_tag(40, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.show_burrowed_shadows { w.write_with_tag(72, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.show_placeholders { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.raw_affects_selection { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.raw_crop_to_playable_area { w.write_with_tag(56, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerInfo<'a> {
    pub player_id: Option<u32>,
    pub type_pb: Option<PlayerType>,
    pub race_requested: Option<Race>,
    pub race_actual: Option<Race>,
    pub difficulty: Option<Difficulty>,
    pub ai_build: Option<AIBuild>,
    pub player_name: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for PlayerInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.type_pb = Some(r.read_enum(bytes)?),
                Ok(24) => msg.race_requested = Some(r.read_enum(bytes)?),
                Ok(32) => msg.race_actual = Some(r.read_enum(bytes)?),
                Ok(40) => msg.difficulty = Some(r.read_enum(bytes)?),
                Ok(56) => msg.ai_build = Some(r.read_enum(bytes)?),
                Ok(50) => msg.player_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PlayerInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.race_requested.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.race_actual.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.difficulty.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ai_build.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.player_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.race_requested { w.write_with_tag(24, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.race_actual { w.write_with_tag(32, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.difficulty { w.write_with_tag(40, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.ai_build { w.write_with_tag(56, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.player_name { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerCommon {
    pub player_id: Option<u32>,
    pub minerals: Option<u32>,
    pub vespene: Option<u32>,
    pub food_cap: Option<u32>,
    pub food_used: Option<u32>,
    pub food_army: Option<u32>,
    pub food_workers: Option<u32>,
    pub idle_worker_count: Option<u32>,
    pub army_count: Option<u32>,
    pub warp_gate_count: Option<u32>,
    pub larva_count: Option<u32>,
}

impl<'a> MessageRead<'a> for PlayerCommon {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.minerals = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.vespene = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.food_cap = Some(r.read_uint32(bytes)?),
                Ok(40) => msg.food_used = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.food_army = Some(r.read_uint32(bytes)?),
                Ok(56) => msg.food_workers = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.idle_worker_count = Some(r.read_uint32(bytes)?),
                Ok(72) => msg.army_count = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.warp_gate_count = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.larva_count = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerCommon {
    fn get_size(&self) -> usize {
        0
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.minerals.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.vespene.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.food_cap.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.food_used.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.food_army.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.food_workers.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.idle_worker_count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.army_count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.warp_gate_count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.larva_count.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.minerals { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.vespene { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.food_cap { w.write_with_tag(32, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.food_used { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.food_army { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.food_workers { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.idle_worker_count { w.write_with_tag(64, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.army_count { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.warp_gate_count { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.larva_count { w.write_with_tag(88, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Observation<'a> {
    pub game_loop: Option<u32>,
    pub player_common: Option<PlayerCommon>,
    pub alerts: Vec<Alert>,
    pub abilities: Vec<AvailableAbility>,
    pub score: Option<Score>,
    pub raw_data: Option<ObservationRaw<'a>>,
    pub feature_layer_data: Option<ObservationFeatureLayer<'a>>,
    pub render_data: Option<ObservationRender<'a>>,
    pub ui_data: Option<ObservationUI>,
}

impl<'a> MessageRead<'a> for Observation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(72) => msg.game_loop = Some(r.read_uint32(bytes)?),
                Ok(10) => msg.player_common = Some(r.read_message::<PlayerCommon>(bytes)?),
                Ok(80) => msg.alerts.push(r.read_enum(bytes)?),
                Ok(26) => msg.abilities.push(r.read_message::<AvailableAbility>(bytes)?),
                Ok(34) => msg.score = Some(r.read_message::<Score>(bytes)?),
                Ok(42) => msg.raw_data = Some(r.read_message::<ObservationRaw>(bytes)?),
                Ok(50) => msg.feature_layer_data = Some(r.read_message::<ObservationFeatureLayer>(bytes)?),
                Ok(58) => msg.render_data = Some(r.read_message::<ObservationRender>(bytes)?),
                Ok(66) => msg.ui_data = Some(r.read_message::<ObservationUI>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Observation<'a> {
    fn get_size(&self) -> usize {
        0
        + self.game_loop.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.player_common.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.alerts.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + self.abilities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.score.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.raw_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.feature_layer_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.render_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ui_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.game_loop { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.player_common { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.alerts { w.write_with_tag(80, |w| w.write_enum(*s as i32))?; }
        for s in &self.abilities { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.score { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.raw_data { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.feature_layer_data { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.render_data { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ui_data { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Action<'a> {
    pub action_raw: Option<ActionRaw>,
    pub action_feature_layer: Option<ActionSpatial>,
    pub action_render: Option<ActionSpatial>,
    pub action_ui: Option<ActionUI>,
    pub action_chat: Option<ActionChat<'a>>,
    pub game_loop: Option<u32>,
}

impl<'a> MessageRead<'a> for Action<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action_raw = Some(r.read_message::<ActionRaw>(bytes)?),
                Ok(18) => msg.action_feature_layer = Some(r.read_message::<ActionSpatial>(bytes)?),
                Ok(26) => msg.action_render = Some(r.read_message::<ActionSpatial>(bytes)?),
                Ok(34) => msg.action_ui = Some(r.read_message::<ActionUI>(bytes)?),
                Ok(50) => msg.action_chat = Some(r.read_message::<ActionChat>(bytes)?),
                Ok(56) => msg.game_loop = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Action<'a> {
    fn get_size(&self) -> usize {
        0
        + self.action_raw.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.action_feature_layer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.action_render.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.action_ui.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.action_chat.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.game_loop.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.action_raw { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.action_feature_layer { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.action_render { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.action_ui { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.action_chat { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.game_loop { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionChat<'a> {
    pub channel: Option<mod_ActionChat::Channel>,
    pub message: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ActionChat<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.channel = Some(r.read_enum(bytes)?),
                Ok(18) => msg.message = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ActionChat<'a> {
    fn get_size(&self) -> usize {
        0
        + self.channel.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.channel { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.message { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_ActionChat {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Channel {
    Broadcast = 1,
    Team = 2,
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Broadcast
    }
}

impl From<i32> for Channel {
    fn from(i: i32) -> Self {
        match i {
            1 => Channel::Broadcast,
            2 => Channel::Team,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Channel {
    fn from(s: &'a str) -> Self {
        match s {
            "Broadcast" => Channel::Broadcast,
            "Team" => Channel::Team,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionError {
    pub unit_tag: Option<u64>,
    pub ability_id: Option<u64>,
    pub result: Option<ActionResult>,
}

impl<'a> MessageRead<'a> for ActionError {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_tag = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.ability_id = Some(r.read_uint64(bytes)?),
                Ok(24) => msg.result = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionError {
    fn get_size(&self) -> usize {
        0
        + self.unit_tag.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ability_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.unit_tag { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.ability_id { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(24, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObserverAction {
    pub action: mod_ObserverAction::OneOfaction,
}

impl<'a> MessageRead<'a> for ObserverAction {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action = mod_ObserverAction::OneOfaction::player_perspective(r.read_message::<ActionObserverPlayerPerspective>(bytes)?),
                Ok(18) => msg.action = mod_ObserverAction::OneOfaction::camera_move(r.read_message::<ActionObserverCameraMove>(bytes)?),
                Ok(26) => msg.action = mod_ObserverAction::OneOfaction::camera_follow_player(r.read_message::<ActionObserverCameraFollowPlayer>(bytes)?),
                Ok(34) => msg.action = mod_ObserverAction::OneOfaction::camera_follow_units(r.read_message::<ActionObserverCameraFollowUnits>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ObserverAction {
    fn get_size(&self) -> usize {
        0
        + match self.action {
            mod_ObserverAction::OneOfaction::player_perspective(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObserverAction::OneOfaction::camera_move(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObserverAction::OneOfaction::camera_follow_player(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObserverAction::OneOfaction::camera_follow_units(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ObserverAction::OneOfaction::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.action {            mod_ObserverAction::OneOfaction::player_perspective(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_ObserverAction::OneOfaction::camera_move(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ObserverAction::OneOfaction::camera_follow_player(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_ObserverAction::OneOfaction::camera_follow_units(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_ObserverAction::OneOfaction::None => {},
    }        Ok(())
    }
}

pub mod mod_ObserverAction {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfaction {
    player_perspective(ActionObserverPlayerPerspective),
    camera_move(ActionObserverCameraMove),
    camera_follow_player(ActionObserverCameraFollowPlayer),
    camera_follow_units(ActionObserverCameraFollowUnits),
    None,
}

impl Default for OneOfaction {
    fn default() -> Self {
        OneOfaction::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionObserverPlayerPerspective {
    pub player_id: Option<u32>,
}

impl<'a> MessageRead<'a> for ActionObserverPlayerPerspective {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionObserverPlayerPerspective {
    fn get_size(&self) -> usize {
        0
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionObserverCameraMove {
    pub world_pos: Option<Point2D>,
    pub distance: Option<f32>,
}

impl<'a> MessageRead<'a> for ActionObserverCameraMove {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.world_pos = Some(r.read_message::<Point2D>(bytes)?),
                Ok(21) => msg.distance = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionObserverCameraMove {
    fn get_size(&self) -> usize {
        0
        + self.world_pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.distance.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.world_pos { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.distance { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionObserverCameraFollowPlayer {
    pub player_id: Option<u32>,
}

impl<'a> MessageRead<'a> for ActionObserverCameraFollowPlayer {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionObserverCameraFollowPlayer {
    fn get_size(&self) -> usize {
        0
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ActionObserverCameraFollowUnits {
    pub unit_tags: Vec<u64>,
}

impl<'a> MessageRead<'a> for ActionObserverCameraFollowUnits {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.unit_tags.push(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ActionObserverCameraFollowUnits {
    fn get_size(&self) -> usize {
        0
        + self.unit_tags.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.unit_tags { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PlayerResult {
    pub player_id: Option<u32>,
    pub result: Option<Result_pb>,
}

impl<'a> MessageRead<'a> for PlayerResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.player_id = Some(r.read_uint32(bytes)?),
                Ok(16) => msg.result = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerResult {
    fn get_size(&self) -> usize {
        0
        + self.player_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player_id { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.result { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

