#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to game_controller_interface__msg__GameControlData
/// 跟 include/RoboCupGameControllData.h 中的 RoboCupGameControlData 保持一致的结构

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GameControlData {
    /// header to identify the structure
    pub header: [u8; 4],

    /// version of the data structure
    pub version: u16,

    /// number incremented with each packet sent (with wraparound)
    pub packet_number: u8,

    /// the number of players on a team
    pub players_per_team: u8,

    /// type of the game (GAME_ROUNDROBIN, GAME_PLAYOFF, GAME_DROPIN)
    pub game_type: u8,

    /// state of the game (STATE_READY, STATE_PLAYING, etc)
    pub state: u8,

    /// 1 = game in first half, 0 otherwise
    pub first_half: u8,

    /// the team number of the next team to kick off or DROPBALL
    pub kick_off_team: u8,

    /// extra state information - (STATE2_NORMAL, STATE2_PENALTYSHOOT, etc)
    pub secondary_state: u8,

    /// Extra info on the secondary state
    pub secondary_state_info: [u8; 4],

    /// number of team that caused last drop in
    pub drop_in_team: u8,

    /// number of seconds passed since the last drop in. -1 (0xffff) before first dropin
    pub drop_in_time: u16,

    /// estimate of number of seconds remaining in the half
    pub secs_remaining: u16,

    /// number of seconds shown as secondary time (remaining ready, until free ball, etc)
    pub secondary_time: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub teams: [super::msg::TeamInfo; 2],

}



impl Default for GameControlData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GameControlData::default())
  }
}

impl rosidl_runtime_rs::Message for GameControlData {
  type RmwMsg = super::msg::rmw::GameControlData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: msg.header,
        version: msg.version,
        packet_number: msg.packet_number,
        players_per_team: msg.players_per_team,
        game_type: msg.game_type,
        state: msg.state,
        first_half: msg.first_half,
        kick_off_team: msg.kick_off_team,
        secondary_state: msg.secondary_state,
        secondary_state_info: msg.secondary_state_info,
        drop_in_team: msg.drop_in_team,
        drop_in_time: msg.drop_in_time,
        secs_remaining: msg.secs_remaining,
        secondary_time: msg.secondary_time,
        teams: msg.teams
          .map(|elem| super::msg::TeamInfo::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: msg.header,
      version: msg.version,
      packet_number: msg.packet_number,
      players_per_team: msg.players_per_team,
      game_type: msg.game_type,
      state: msg.state,
      first_half: msg.first_half,
      kick_off_team: msg.kick_off_team,
      secondary_state: msg.secondary_state,
        secondary_state_info: msg.secondary_state_info,
      drop_in_team: msg.drop_in_team,
      drop_in_time: msg.drop_in_time,
      secs_remaining: msg.secs_remaining,
      secondary_time: msg.secondary_time,
        teams: msg.teams
          .iter()
          .map(|elem| super::msg::TeamInfo::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: msg.header,
      version: msg.version,
      packet_number: msg.packet_number,
      players_per_team: msg.players_per_team,
      game_type: msg.game_type,
      state: msg.state,
      first_half: msg.first_half,
      kick_off_team: msg.kick_off_team,
      secondary_state: msg.secondary_state,
      secondary_state_info: msg.secondary_state_info,
      drop_in_team: msg.drop_in_team,
      drop_in_time: msg.drop_in_time,
      secs_remaining: msg.secs_remaining,
      secondary_time: msg.secondary_time,
      teams: msg.teams
        .map(super::msg::TeamInfo::from_rmw_message),
    }
  }
}


// Corresponds to game_controller_interface__msg__RobotInfo
/// 跟 include/RoboCupGameControllData.h 中的 RobotInfo 保持一致的结构

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotInfo {
    /// penalty state of the player
    pub penalty: u8,

    /// estimate of time till unpenalised
    pub secs_till_unpenalised: u8,

    /// number of warnings
    pub number_of_warnings: u8,

    /// number of yellow cards
    pub yellow_card_count: u8,

    /// number of red cards
    pub red_card_count: u8,

    /// flags if robot is goal keeper
    pub goal_keeper: bool,

}



impl Default for RobotInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RobotInfo::default())
  }
}

impl rosidl_runtime_rs::Message for RobotInfo {
  type RmwMsg = super::msg::rmw::RobotInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        penalty: msg.penalty,
        secs_till_unpenalised: msg.secs_till_unpenalised,
        number_of_warnings: msg.number_of_warnings,
        yellow_card_count: msg.yellow_card_count,
        red_card_count: msg.red_card_count,
        goal_keeper: msg.goal_keeper,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      penalty: msg.penalty,
      secs_till_unpenalised: msg.secs_till_unpenalised,
      number_of_warnings: msg.number_of_warnings,
      yellow_card_count: msg.yellow_card_count,
      red_card_count: msg.red_card_count,
      goal_keeper: msg.goal_keeper,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      penalty: msg.penalty,
      secs_till_unpenalised: msg.secs_till_unpenalised,
      number_of_warnings: msg.number_of_warnings,
      yellow_card_count: msg.yellow_card_count,
      red_card_count: msg.red_card_count,
      goal_keeper: msg.goal_keeper,
    }
  }
}


// Corresponds to game_controller_interface__msg__TeamInfo
/// 跟 include/RoboCupGameControllData.h 中的 TeamInfo 保持一致的结构
/// 稍注意一下，coach_message 和 players 在这里定义为不定长数组，更符合语义

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TeamInfo {
    /// unique team number
    pub team_number: u8,

    /// colour of the team
    pub field_player_colour: u8,

    /// team's score
    pub score: u8,

    /// penalty shot counter
    pub penalty_shot: u8,

    /// bits represent penalty shot success
    pub single_shots: u16,

    /// sequence number of the coach's message
    pub coach_sequence: u8,

    /// the coach's message to the team (length=253)
    pub coach_message: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coach: super::msg::RobotInfo,

    /// the team's players (length=11)
    pub players: Vec<super::msg::RobotInfo>,

}



impl Default for TeamInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TeamInfo::default())
  }
}

impl rosidl_runtime_rs::Message for TeamInfo {
  type RmwMsg = super::msg::rmw::TeamInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        team_number: msg.team_number,
        field_player_colour: msg.field_player_colour,
        score: msg.score,
        penalty_shot: msg.penalty_shot,
        single_shots: msg.single_shots,
        coach_sequence: msg.coach_sequence,
        coach_message: msg.coach_message.into(),
        coach: super::msg::RobotInfo::into_rmw_message(std::borrow::Cow::Owned(msg.coach)).into_owned(),
        players: msg.players
          .into_iter()
          .map(|elem| super::msg::RobotInfo::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      team_number: msg.team_number,
      field_player_colour: msg.field_player_colour,
      score: msg.score,
      penalty_shot: msg.penalty_shot,
      single_shots: msg.single_shots,
      coach_sequence: msg.coach_sequence,
        coach_message: msg.coach_message.as_slice().into(),
        coach: super::msg::RobotInfo::into_rmw_message(std::borrow::Cow::Borrowed(&msg.coach)).into_owned(),
        players: msg.players
          .iter()
          .map(|elem| super::msg::RobotInfo::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      team_number: msg.team_number,
      field_player_colour: msg.field_player_colour,
      score: msg.score,
      penalty_shot: msg.penalty_shot,
      single_shots: msg.single_shots,
      coach_sequence: msg.coach_sequence,
      coach_message: msg.coach_message
          .into_iter()
          .collect(),
      coach: super::msg::RobotInfo::from_rmw_message(msg.coach),
      players: msg.players
          .into_iter()
          .map(super::msg::RobotInfo::from_rmw_message)
          .collect(),
    }
  }
}


