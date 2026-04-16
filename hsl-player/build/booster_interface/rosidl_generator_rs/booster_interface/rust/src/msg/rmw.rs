#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__BoosterApiReqMsg() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__BoosterApiReqMsg__init(msg: *mut BoosterApiReqMsg) -> bool;
    fn booster_interface__msg__BoosterApiReqMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BoosterApiReqMsg>, size: usize) -> bool;
    fn booster_interface__msg__BoosterApiReqMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BoosterApiReqMsg>);
    fn booster_interface__msg__BoosterApiReqMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BoosterApiReqMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<BoosterApiReqMsg>) -> bool;
}

// Corresponds to booster_interface__msg__BoosterApiReqMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoosterApiReqMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub api_id: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for BoosterApiReqMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__BoosterApiReqMsg__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__BoosterApiReqMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BoosterApiReqMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__BoosterApiReqMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__BoosterApiReqMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__BoosterApiReqMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BoosterApiReqMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BoosterApiReqMsg where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/BoosterApiReqMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__BoosterApiReqMsg() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__BoosterApiRespMsg() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__BoosterApiRespMsg__init(msg: *mut BoosterApiRespMsg) -> bool;
    fn booster_interface__msg__BoosterApiRespMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BoosterApiRespMsg>, size: usize) -> bool;
    fn booster_interface__msg__BoosterApiRespMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BoosterApiRespMsg>);
    fn booster_interface__msg__BoosterApiRespMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BoosterApiRespMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<BoosterApiRespMsg>) -> bool;
}

// Corresponds to booster_interface__msg__BoosterApiRespMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoosterApiRespMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for BoosterApiRespMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__BoosterApiRespMsg__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__BoosterApiRespMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BoosterApiRespMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__BoosterApiRespMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__BoosterApiRespMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__BoosterApiRespMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BoosterApiRespMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BoosterApiRespMsg where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/BoosterApiRespMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__BoosterApiRespMsg() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__ButtonEventMsg() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__ButtonEventMsg__init(msg: *mut ButtonEventMsg) -> bool;
    fn booster_interface__msg__ButtonEventMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ButtonEventMsg>, size: usize) -> bool;
    fn booster_interface__msg__ButtonEventMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ButtonEventMsg>);
    fn booster_interface__msg__ButtonEventMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ButtonEventMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<ButtonEventMsg>) -> bool;
}

// Corresponds to booster_interface__msg__ButtonEventMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// constants

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ButtonEventMsg {
    /// fields
    pub event: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub button: i32,

}

impl ButtonEventMsg {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PRESS_DOWN: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PRESS_UP: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SINGLE_CLICK: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DOUBLE_CLICK: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TRIPLE_CLICK: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LONG_PRESS_START: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LONG_PRESS_HOLD: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LONG_PRESS_END: i8 = 1;

}


impl Default for ButtonEventMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__ButtonEventMsg__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__ButtonEventMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ButtonEventMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__ButtonEventMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__ButtonEventMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__ButtonEventMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ButtonEventMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ButtonEventMsg where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/ButtonEventMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__ButtonEventMsg() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__MotorState() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__MotorState__init(msg: *mut MotorState) -> bool;
    fn booster_interface__msg__MotorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorState>, size: usize) -> bool;
    fn booster_interface__msg__MotorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorState>);
    fn booster_interface__msg__MotorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorState>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorState>) -> bool;
}

// Corresponds to booster_interface__msg__MotorState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub q: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ddq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tau_est: f32,

    /// 电机温度信息：类型：int8_t ，可按照实际数值显示（范围：-100 - 150）。
    pub temperature: i8,

    /// 电机丢包信息：可按照实际数值显示（范围：0-9999999999）。
    pub lost: u32,

    /// 当前电机通信频率+电机错误标志位：（数组：0-电机错误标志位（范围：0-255，可按照实际数值显示），1-当前电机通信频率（范围：0-800，可按照实际数值显示））
    pub reserve: [u32; 2],

}



impl Default for MotorState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__MotorState__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__MotorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__MotorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__MotorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__MotorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorState where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/MotorState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__MotorState() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__MotorCmd() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__MotorCmd__init(msg: *mut MotorCmd) -> bool;
    fn booster_interface__msg__MotorCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>, size: usize) -> bool;
    fn booster_interface__msg__MotorCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>);
    fn booster_interface__msg__MotorCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorCmd>) -> bool;
}

// Corresponds to booster_interface__msg__MotorCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorCmd {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub q: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tau: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kp: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kd: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub weight: f32,

}



impl Default for MotorCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__MotorCmd__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__MotorCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__MotorCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__MotorCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__MotorCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorCmd where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/MotorCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__MotorCmd() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__LowCmd() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__LowCmd__init(msg: *mut LowCmd) -> bool;
    fn booster_interface__msg__LowCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LowCmd>, size: usize) -> bool;
    fn booster_interface__msg__LowCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LowCmd>);
    fn booster_interface__msg__LowCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LowCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<LowCmd>) -> bool;
}

// Corresponds to booster_interface__msg__LowCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// constants

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowCmd {
    /// fields
    /// use CMD_TYPE_PARALLEL or CMD_TYPE_SERIAL
    pub cmd_type: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_cmd: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotorCmd>,

}

impl LowCmd {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CMD_TYPE_PARALLEL: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CMD_TYPE_SERIAL: i8 = 1;

}


impl Default for LowCmd {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__LowCmd__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__LowCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LowCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__LowCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__LowCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__LowCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LowCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LowCmd where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/LowCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__LowCmd() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__ImuState() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__ImuState__init(msg: *mut ImuState) -> bool;
    fn booster_interface__msg__ImuState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ImuState>, size: usize) -> bool;
    fn booster_interface__msg__ImuState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ImuState>);
    fn booster_interface__msg__ImuState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ImuState>, out_seq: *mut rosidl_runtime_rs::Sequence<ImuState>) -> bool;
}

// Corresponds to booster_interface__msg__ImuState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 欧拉角信息（0 -> roll ,0 -> pitch ,0 -> yaw）

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ImuState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rpy: [f32; 3],

    /// 角速度信息（0 -> x ,0 -> y ,0 -> z）
    pub gyro: [f32; 3],

    /// 加速度信息（0 -> x ,0 -> y ,0 -> z）
    pub acc: [f32; 3],

}



impl Default for ImuState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__ImuState__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__ImuState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ImuState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__ImuState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__ImuState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__ImuState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ImuState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ImuState where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/ImuState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__ImuState() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__LowState() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__LowState__init(msg: *mut LowState) -> bool;
    fn booster_interface__msg__LowState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LowState>, size: usize) -> bool;
    fn booster_interface__msg__LowState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LowState>);
    fn booster_interface__msg__LowState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LowState>, out_seq: *mut rosidl_runtime_rs::Sequence<LowState>) -> bool;
}

// Corresponds to booster_interface__msg__LowState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::super::msg::rmw::ImuState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state_parallel: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotorState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state_serial: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MotorState>,

}



impl Default for LowState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__LowState__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__LowState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LowState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__LowState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__LowState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__LowState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LowState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LowState where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/LowState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__LowState() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__RawBytesMsg() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__RawBytesMsg__init(msg: *mut RawBytesMsg) -> bool;
    fn booster_interface__msg__RawBytesMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RawBytesMsg>, size: usize) -> bool;
    fn booster_interface__msg__RawBytesMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RawBytesMsg>);
    fn booster_interface__msg__RawBytesMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RawBytesMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<RawBytesMsg>) -> bool;
}

// Corresponds to booster_interface__msg__RawBytesMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawBytesMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for RawBytesMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__RawBytesMsg__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__RawBytesMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RawBytesMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RawBytesMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RawBytesMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RawBytesMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RawBytesMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RawBytesMsg where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/RawBytesMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__RawBytesMsg() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__RawBytesStamped() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__RawBytesStamped__init(msg: *mut RawBytesStamped) -> bool;
    fn booster_interface__msg__RawBytesStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RawBytesStamped>, size: usize) -> bool;
    fn booster_interface__msg__RawBytesStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RawBytesStamped>);
    fn booster_interface__msg__RawBytesStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RawBytesStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<RawBytesStamped>) -> bool;
}

// Corresponds to booster_interface__msg__RawBytesStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawBytesStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for RawBytesStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__RawBytesStamped__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__RawBytesStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RawBytesStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RawBytesStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RawBytesStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RawBytesStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RawBytesStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RawBytesStamped where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/RawBytesStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__RawBytesStamped() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__Odometer() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__Odometer__init(msg: *mut Odometer) -> bool;
    fn booster_interface__msg__Odometer__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Odometer>, size: usize) -> bool;
    fn booster_interface__msg__Odometer__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Odometer>);
    fn booster_interface__msg__Odometer__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Odometer>, out_seq: *mut rosidl_runtime_rs::Sequence<Odometer>) -> bool;
}

// Corresponds to booster_interface__msg__Odometer
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Odometer {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub theta: f32,

}



impl Default for Odometer {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__Odometer__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__Odometer__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Odometer {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__Odometer__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__Odometer__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__Odometer__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Odometer {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Odometer where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/Odometer";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__Odometer() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__FallDownState() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__FallDownState__init(msg: *mut FallDownState) -> bool;
    fn booster_interface__msg__FallDownState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FallDownState>, size: usize) -> bool;
    fn booster_interface__msg__FallDownState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FallDownState>);
    fn booster_interface__msg__FallDownState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FallDownState>, out_seq: *mut rosidl_runtime_rs::Sequence<FallDownState>) -> bool;
}

// Corresponds to booster_interface__msg__FallDownState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FallDownState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub fall_down_state: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_recovery_available: bool,

}

impl FallDownState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IS_READY: u32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IS_FALLING: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HAS_FALLEN: u32 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IS_GETTING_UP: u32 = 3;

}


impl Default for FallDownState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__FallDownState__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__FallDownState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FallDownState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__FallDownState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__FallDownState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__FallDownState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FallDownState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FallDownState where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/FallDownState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__FallDownState() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__RemoteControllerState() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__RemoteControllerState__init(msg: *mut RemoteControllerState) -> bool;
    fn booster_interface__msg__RemoteControllerState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RemoteControllerState>, size: usize) -> bool;
    fn booster_interface__msg__RemoteControllerState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RemoteControllerState>);
    fn booster_interface__msg__RemoteControllerState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RemoteControllerState>, out_seq: *mut rosidl_runtime_rs::Sequence<RemoteControllerState>) -> bool;
}

// Corresponds to booster_interface__msg__RemoteControllerState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoteControllerState {
    /// SDL_EventType, value reference from SDL2/SDL_events.h
    pub event: u32,

    /// left stick horizontal direction, push left to -1, push right to 1
    pub lx: f32,

    /// left stick vertical direction, push front to -1, push back to 1
    pub ly: f32,

    /// right stick horizontal direction, push left to -1, push right to 1
    pub rx: f32,

    /// right stick vertical direction, push front to -1, push back to 1
    pub ry: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lb: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rb: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lt: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rt: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ls: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rs: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub back: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start: bool,

    /// Hat centered
    pub hat_c: bool,

    /// Hat up
    pub hat_u: bool,

    /// Hat down
    pub hat_d: bool,

    /// Hat left
    pub hat_l: bool,

    /// Hat right
    pub hat_r: bool,

    /// Hat left up
    pub hat_lu: bool,

    /// Hat left down
    pub hat_ld: bool,

    /// Hat right up
    pub hat_ru: bool,

    /// Hat right down
    pub hat_rd: bool,

    /// Hat position, value reference from SDL2/SDL_joystick.h
    pub hat_pos: u8,

}



impl Default for RemoteControllerState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__RemoteControllerState__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__RemoteControllerState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RemoteControllerState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RemoteControllerState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RemoteControllerState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__RemoteControllerState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RemoteControllerState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RemoteControllerState where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/RemoteControllerState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__RemoteControllerState() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__HandCommand() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__HandCommand__init(msg: *mut HandCommand) -> bool;
    fn booster_interface__msg__HandCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HandCommand>, size: usize) -> bool;
    fn booster_interface__msg__HandCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HandCommand>);
    fn booster_interface__msg__HandCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HandCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<HandCommand>) -> bool;
}

// Corresponds to booster_interface__msg__HandCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hand_param: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HandParam>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub force_mode: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub hand_index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub hand_type: i32,

}



impl Default for HandCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__HandCommand__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__HandCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HandCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HandCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HandCommand where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/HandCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__HandCommand() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__HandDdsMsg() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__HandDdsMsg__init(msg: *mut HandDdsMsg) -> bool;
    fn booster_interface__msg__HandDdsMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HandDdsMsg>, size: usize) -> bool;
    fn booster_interface__msg__HandDdsMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HandDdsMsg>);
    fn booster_interface__msg__HandDdsMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HandDdsMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<HandDdsMsg>) -> bool;
}

// Corresponds to booster_interface__msg__HandDdsMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandDdsMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hands_vec: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HandCommand>,

}



impl Default for HandDdsMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__HandDdsMsg__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__HandDdsMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HandDdsMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandDdsMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandDdsMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandDdsMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HandDdsMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HandDdsMsg where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/HandDdsMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__HandDdsMsg() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__HandParam() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__msg__HandParam__init(msg: *mut HandParam) -> bool;
    fn booster_interface__msg__HandParam__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HandParam>, size: usize) -> bool;
    fn booster_interface__msg__HandParam__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HandParam>);
    fn booster_interface__msg__HandParam__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HandParam>, out_seq: *mut rosidl_runtime_rs::Sequence<HandParam>) -> bool;
}

// Corresponds to booster_interface__msg__HandParam
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandParam {

    // This member is not documented.
    #[allow(missing_docs)]
    pub angle: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub force: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub seq: i32,

}



impl Default for HandParam {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__msg__HandParam__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__msg__HandParam__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HandParam {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandParam__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandParam__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__msg__HandParam__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HandParam {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HandParam where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/msg/HandParam";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__msg__HandParam() }
  }
}


