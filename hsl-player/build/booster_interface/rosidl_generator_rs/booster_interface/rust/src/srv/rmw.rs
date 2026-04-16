#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__RpcService_Request() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__srv__RpcService_Request__init(msg: *mut RpcService_Request) -> bool;
    fn booster_interface__srv__RpcService_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RpcService_Request>, size: usize) -> bool;
    fn booster_interface__srv__RpcService_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RpcService_Request>);
    fn booster_interface__srv__RpcService_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RpcService_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RpcService_Request>) -> bool;
}

// Corresponds to booster_interface__srv__RpcService_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcService_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: super::super::msg::rmw::BoosterApiReqMsg,

}



impl Default for RpcService_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__srv__RpcService_Request__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__srv__RpcService_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RpcService_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__RpcService_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__RpcService_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__RpcService_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RpcService_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RpcService_Request where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/srv/RpcService_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__RpcService_Request() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__RpcService_Response() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__srv__RpcService_Response__init(msg: *mut RpcService_Response) -> bool;
    fn booster_interface__srv__RpcService_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RpcService_Response>, size: usize) -> bool;
    fn booster_interface__srv__RpcService_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RpcService_Response>);
    fn booster_interface__srv__RpcService_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RpcService_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RpcService_Response>) -> bool;
}

// Corresponds to booster_interface__srv__RpcService_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: super::super::msg::rmw::BoosterApiRespMsg,

}



impl Default for RpcService_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__srv__RpcService_Response__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__srv__RpcService_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RpcService_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__RpcService_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__RpcService_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__RpcService_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RpcService_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RpcService_Response where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/srv/RpcService_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__RpcService_Response() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__AgentService_Request() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__srv__AgentService_Request__init(msg: *mut AgentService_Request) -> bool;
    fn booster_interface__srv__AgentService_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AgentService_Request>, size: usize) -> bool;
    fn booster_interface__srv__AgentService_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AgentService_Request>);
    fn booster_interface__srv__AgentService_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AgentService_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AgentService_Request>) -> bool;
}

// Corresponds to booster_interface__srv__AgentService_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgentService_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for AgentService_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__srv__AgentService_Request__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__srv__AgentService_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AgentService_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__AgentService_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__AgentService_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__AgentService_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AgentService_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AgentService_Request where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/srv/AgentService_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__AgentService_Request() }
  }
}


#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__AgentService_Response() -> *const std::ffi::c_void;
}

#[link(name = "booster_interface__rosidl_generator_c")]
extern "C" {
    fn booster_interface__srv__AgentService_Response__init(msg: *mut AgentService_Response) -> bool;
    fn booster_interface__srv__AgentService_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AgentService_Response>, size: usize) -> bool;
    fn booster_interface__srv__AgentService_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AgentService_Response>);
    fn booster_interface__srv__AgentService_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AgentService_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AgentService_Response>) -> bool;
}

// Corresponds to booster_interface__srv__AgentService_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgentService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::String,

}



impl Default for AgentService_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !booster_interface__srv__AgentService_Response__init(&mut msg as *mut _) {
        panic!("Call to booster_interface__srv__AgentService_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AgentService_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__AgentService_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__AgentService_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { booster_interface__srv__AgentService_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AgentService_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AgentService_Response where Self: Sized {
  const TYPE_NAME: &'static str = "booster_interface/srv/AgentService_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__booster_interface__srv__AgentService_Response() }
  }
}






#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__booster_interface__srv__RpcService() -> *const std::ffi::c_void;
}

// Corresponds to booster_interface__srv__RpcService
#[allow(missing_docs, non_camel_case_types)]
pub struct RpcService;

impl rosidl_runtime_rs::Service for RpcService {
    type Request = RpcService_Request;
    type Response = RpcService_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__booster_interface__srv__RpcService() }
    }
}




#[link(name = "booster_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__booster_interface__srv__AgentService() -> *const std::ffi::c_void;
}

// Corresponds to booster_interface__srv__AgentService
#[allow(missing_docs, non_camel_case_types)]
pub struct AgentService;

impl rosidl_runtime_rs::Service for AgentService {
    type Request = AgentService_Request;
    type Response = AgentService_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__booster_interface__srv__AgentService() }
    }
}


