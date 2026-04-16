#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "game_controller_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__game_controller_interface__msg__GameControlData() -> *const std::ffi::c_void;
}

#[link(name = "game_controller_interface__rosidl_generator_c")]
extern "C" {
    fn game_controller_interface__msg__GameControlData__init(msg: *mut GameControlData) -> bool;
    fn game_controller_interface__msg__GameControlData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GameControlData>, size: usize) -> bool;
    fn game_controller_interface__msg__GameControlData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GameControlData>);
    fn game_controller_interface__msg__GameControlData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GameControlData>, out_seq: *mut rosidl_runtime_rs::Sequence<GameControlData>) -> bool;
}

// Corresponds to game_controller_interface__msg__GameControlData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 跟 include/RoboCupGameControllData.h 中的 RoboCupGameControlData 保持一致的结构

#[repr(C)]
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
    pub teams: [super::super::msg::rmw::TeamInfo; 2],

}



impl Default for GameControlData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !game_controller_interface__msg__GameControlData__init(&mut msg as *mut _) {
        panic!("Call to game_controller_interface__msg__GameControlData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GameControlData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__GameControlData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__GameControlData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__GameControlData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GameControlData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GameControlData where Self: Sized {
  const TYPE_NAME: &'static str = "game_controller_interface/msg/GameControlData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__game_controller_interface__msg__GameControlData() }
  }
}


#[link(name = "game_controller_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__game_controller_interface__msg__RobotInfo() -> *const std::ffi::c_void;
}

#[link(name = "game_controller_interface__rosidl_generator_c")]
extern "C" {
    fn game_controller_interface__msg__RobotInfo__init(msg: *mut RobotInfo) -> bool;
    fn game_controller_interface__msg__RobotInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotInfo>, size: usize) -> bool;
    fn game_controller_interface__msg__RobotInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotInfo>);
    fn game_controller_interface__msg__RobotInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotInfo>) -> bool;
}

// Corresponds to game_controller_interface__msg__RobotInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 跟 include/RoboCupGameControllData.h 中的 RobotInfo 保持一致的结构

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !game_controller_interface__msg__RobotInfo__init(&mut msg as *mut _) {
        panic!("Call to game_controller_interface__msg__RobotInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__RobotInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__RobotInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__RobotInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotInfo where Self: Sized {
  const TYPE_NAME: &'static str = "game_controller_interface/msg/RobotInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__game_controller_interface__msg__RobotInfo() }
  }
}


#[link(name = "game_controller_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__game_controller_interface__msg__TeamInfo() -> *const std::ffi::c_void;
}

#[link(name = "game_controller_interface__rosidl_generator_c")]
extern "C" {
    fn game_controller_interface__msg__TeamInfo__init(msg: *mut TeamInfo) -> bool;
    fn game_controller_interface__msg__TeamInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TeamInfo>, size: usize) -> bool;
    fn game_controller_interface__msg__TeamInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TeamInfo>);
    fn game_controller_interface__msg__TeamInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TeamInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<TeamInfo>) -> bool;
}

// Corresponds to game_controller_interface__msg__TeamInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 跟 include/RoboCupGameControllData.h 中的 TeamInfo 保持一致的结构
/// 稍注意一下，coach_message 和 players 在这里定义为不定长数组，更符合语义

#[repr(C)]
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
    pub coach_message: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coach: super::super::msg::rmw::RobotInfo,

    /// the team's players (length=11)
    pub players: rosidl_runtime_rs::Sequence<super::super::msg::rmw::RobotInfo>,

}



impl Default for TeamInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !game_controller_interface__msg__TeamInfo__init(&mut msg as *mut _) {
        panic!("Call to game_controller_interface__msg__TeamInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TeamInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__TeamInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__TeamInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { game_controller_interface__msg__TeamInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TeamInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TeamInfo where Self: Sized {
  const TYPE_NAME: &'static str = "game_controller_interface/msg/TeamInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__game_controller_interface__msg__TeamInfo() }
  }
}


