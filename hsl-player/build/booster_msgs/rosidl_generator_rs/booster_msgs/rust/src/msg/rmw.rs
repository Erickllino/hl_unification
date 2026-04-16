#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "booster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_msgs__msg__BinaryData() -> *const std::ffi::c_void;
}

#[link(name = "booster_msgs__rosidl_generator_c")]
extern "C" {
    fn booster_msgs__msg__BinaryData__init(msg: *mut BinaryData) -> bool;
    fn booster_msgs__msg__BinaryData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BinaryData>, size: usize) -> bool;
    fn booster_msgs__msg__BinaryData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BinaryData>);
    fn booster_msgs__msg__BinaryData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BinaryData>, out_seq: *mut rosidl_runtime_rs::Sequence<BinaryData>) -> bool;
}

// Corresponds to booster_msgs__msg__BinaryData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BinaryData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for BinaryData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_msgs__msg__BinaryData__init(&mut msg as *mut _) {
        panic!("Call to booster_msgs__msg__BinaryData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BinaryData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__BinaryData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__BinaryData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__BinaryData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BinaryData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BinaryData where Self: Sized {
  const TYPE_NAME: &'static str = "booster_msgs/msg/BinaryData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_msgs__msg__BinaryData() }
  }
}


#[link(name = "booster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_msgs__msg__RpcReqMsg() -> *const std::ffi::c_void;
}

#[link(name = "booster_msgs__rosidl_generator_c")]
extern "C" {
    fn booster_msgs__msg__RpcReqMsg__init(msg: *mut RpcReqMsg) -> bool;
    fn booster_msgs__msg__RpcReqMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RpcReqMsg>, size: usize) -> bool;
    fn booster_msgs__msg__RpcReqMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RpcReqMsg>);
    fn booster_msgs__msg__RpcReqMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RpcReqMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<RpcReqMsg>) -> bool;
}

// Corresponds to booster_msgs__msg__RpcReqMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcReqMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub header: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for RpcReqMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_msgs__msg__RpcReqMsg__init(&mut msg as *mut _) {
        panic!("Call to booster_msgs__msg__RpcReqMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RpcReqMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__RpcReqMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__RpcReqMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__RpcReqMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RpcReqMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RpcReqMsg where Self: Sized {
  const TYPE_NAME: &'static str = "booster_msgs/msg/RpcReqMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_msgs__msg__RpcReqMsg() }
  }
}


#[link(name = "booster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_msgs__msg__RpcRespMsg() -> *const std::ffi::c_void;
}

#[link(name = "booster_msgs__rosidl_generator_c")]
extern "C" {
    fn booster_msgs__msg__RpcRespMsg__init(msg: *mut RpcRespMsg) -> bool;
    fn booster_msgs__msg__RpcRespMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RpcRespMsg>, size: usize) -> bool;
    fn booster_msgs__msg__RpcRespMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RpcRespMsg>);
    fn booster_msgs__msg__RpcRespMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RpcRespMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<RpcRespMsg>) -> bool;
}

// Corresponds to booster_msgs__msg__RpcRespMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcRespMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub header: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for RpcRespMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_msgs__msg__RpcRespMsg__init(&mut msg as *mut _) {
        panic!("Call to booster_msgs__msg__RpcRespMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RpcRespMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__RpcRespMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__RpcRespMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_msgs__msg__RpcRespMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RpcRespMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RpcRespMsg where Self: Sized {
  const TYPE_NAME: &'static str = "booster_msgs/msg/RpcRespMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_msgs__msg__RpcRespMsg() }
  }
}


