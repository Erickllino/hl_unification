#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "brain__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__brain__msg__Kick() -> *const std::ffi::c_void;
}

#[link(name = "brain__rosidl_generator_c")]
extern "C" {
    fn brain__msg__Kick__init(msg: *mut Kick) -> bool;
    fn brain__msg__Kick__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Kick>, size: usize) -> bool;
    fn brain__msg__Kick__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Kick>);
    fn brain__msg__Kick__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Kick>, out_seq: *mut rosidl_runtime_rs::Sequence<Kick>) -> bool;
}

// Corresponds to brain__msg__Kick
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Kick {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// ball pos to robot x
    pub x: f64,

    /// ball pos to robot y
    pub y: f64,

    /// disired kick dir relative to robot, rad
    pub dir: f64,

    /// goal pos to robot x
    pub goal_x: f64,

    /// goal pos to robot y
    pub goal_y: f64,

    /// robot theta to field, rad
    pub robot_theta_to_field: f64,

}



impl Default for Kick {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !brain__msg__Kick__init(&mut msg as *mut _) {
        panic!("Call to brain__msg__Kick__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Kick {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { brain__msg__Kick__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { brain__msg__Kick__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { brain__msg__Kick__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Kick {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Kick where Self: Sized {
  const TYPE_NAME: &'static str = "brain/msg/Kick";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__brain__msg__Kick() }
  }
}


