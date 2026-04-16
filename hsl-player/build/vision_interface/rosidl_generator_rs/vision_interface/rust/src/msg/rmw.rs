#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__DetectedObject() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__DetectedObject__init(msg: *mut DetectedObject) -> bool;
    fn vision_interface__msg__DetectedObject__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedObject>, size: usize) -> bool;
    fn vision_interface__msg__DetectedObject__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedObject>);
    fn vision_interface__msg__DetectedObject__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedObject>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedObject>) -> bool;
}

// Corresponds to vision_interface__msg__DetectedObject
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedObject {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub color: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub confidence: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub xmin: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ymin: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub xmax: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ymax: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_uv: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub received_pos: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_projection: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_cam: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_confidence: i32,

}



impl Default for DetectedObject {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__DetectedObject__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__DetectedObject__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedObject {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__DetectedObject__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__DetectedObject__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__DetectedObject__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedObject {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedObject where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/DetectedObject";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__DetectedObject() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__Detections() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__Detections__init(msg: *mut Detections) -> bool;
    fn vision_interface__msg__Detections__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Detections>, size: usize) -> bool;
    fn vision_interface__msg__Detections__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Detections>);
    fn vision_interface__msg__Detections__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Detections>, out_seq: *mut rosidl_runtime_rs::Sequence<Detections>) -> bool;
}

// Corresponds to vision_interface__msg__Detections
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Detections {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub detected_objects: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DetectedObject>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_x: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_y: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub corner_pos: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for Detections {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__Detections__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__Detections__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Detections {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__Detections__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__Detections__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__Detections__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Detections {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Detections where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/Detections";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__Detections() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__LineSegments() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__LineSegments__init(msg: *mut LineSegments) -> bool;
    fn vision_interface__msg__LineSegments__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LineSegments>, size: usize) -> bool;
    fn vision_interface__msg__LineSegments__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LineSegments>);
    fn vision_interface__msg__LineSegments__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LineSegments>, out_seq: *mut rosidl_runtime_rs::Sequence<LineSegments>) -> bool;
}

// Corresponds to vision_interface__msg__LineSegments
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LineSegments {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coordinates: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coordinates_uv: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for LineSegments {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__LineSegments__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__LineSegments__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LineSegments {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__LineSegments__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__LineSegments__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__LineSegments__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LineSegments {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LineSegments where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/LineSegments";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__LineSegments() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__CalParam() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__CalParam__init(msg: *mut CalParam) -> bool;
    fn vision_interface__msg__CalParam__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalParam>, size: usize) -> bool;
    fn vision_interface__msg__CalParam__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalParam>);
    fn vision_interface__msg__CalParam__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalParam>, out_seq: *mut rosidl_runtime_rs::Sequence<CalParam>) -> bool;
}

// Corresponds to vision_interface__msg__CalParam
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalParam {
    /// degs
    pub pitch_compensation: f32,

    /// degs
    pub yaw_compensation: f32,

    /// m
    pub z_compensation: f32,

}



impl Default for CalParam {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__CalParam__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__CalParam__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalParam {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__CalParam__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__CalParam__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__CalParam__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalParam {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalParam where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/CalParam";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__CalParam() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__DetectionsTrack() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__DetectionsTrack__init(msg: *mut DetectionsTrack) -> bool;
    fn vision_interface__msg__DetectionsTrack__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectionsTrack>, size: usize) -> bool;
    fn vision_interface__msg__DetectionsTrack__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectionsTrack>);
    fn vision_interface__msg__DetectionsTrack__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectionsTrack>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectionsTrack>) -> bool;
}

// Corresponds to vision_interface__msg__DetectionsTrack
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectionsTrack {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// "yolo" or "pysot" or "none"
    pub ball_type: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub detected_objects: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DetectedObject>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_x: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_y: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub corner_pos: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for DetectionsTrack {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__DetectionsTrack__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__DetectionsTrack__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectionsTrack {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__DetectionsTrack__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__DetectionsTrack__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__DetectionsTrack__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectionsTrack {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectionsTrack where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/DetectionsTrack";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__DetectionsTrack() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__SegmentationPoint() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__SegmentationPoint__init(msg: *mut SegmentationPoint) -> bool;
    fn vision_interface__msg__SegmentationPoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SegmentationPoint>, size: usize) -> bool;
    fn vision_interface__msg__SegmentationPoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SegmentationPoint>);
    fn vision_interface__msg__SegmentationPoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SegmentationPoint>, out_seq: *mut rosidl_runtime_rs::Sequence<SegmentationPoint>) -> bool;
}

// Corresponds to vision_interface__msg__SegmentationPoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationPoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub u: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub v: f64,

}



impl Default for SegmentationPoint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__SegmentationPoint__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__SegmentationPoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SegmentationPoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationPoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationPoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationPoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SegmentationPoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SegmentationPoint where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/SegmentationPoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__SegmentationPoint() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__SegmentationLine() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__SegmentationLine__init(msg: *mut SegmentationLine) -> bool;
    fn vision_interface__msg__SegmentationLine__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SegmentationLine>, size: usize) -> bool;
    fn vision_interface__msg__SegmentationLine__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SegmentationLine>);
    fn vision_interface__msg__SegmentationLine__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SegmentationLine>, out_seq: *mut rosidl_runtime_rs::Sequence<SegmentationLine>) -> bool;
}

// Corresponds to vision_interface__msg__SegmentationLine
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationLine {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub u1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub v1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub u2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub v2: f64,

}



impl Default for SegmentationLine {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__SegmentationLine__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__SegmentationLine__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SegmentationLine {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationLine__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationLine__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationLine__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SegmentationLine {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SegmentationLine where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/SegmentationLine";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__SegmentationLine() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__SegmentationResult() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__SegmentationResult__init(msg: *mut SegmentationResult) -> bool;
    fn vision_interface__msg__SegmentationResult__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SegmentationResult>, size: usize) -> bool;
    fn vision_interface__msg__SegmentationResult__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SegmentationResult>);
    fn vision_interface__msg__SegmentationResult__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SegmentationResult>, out_seq: *mut rosidl_runtime_rs::Sequence<SegmentationResult>) -> bool;
}

// Corresponds to vision_interface__msg__SegmentationResult
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationResult {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lines: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SegmentationLine>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub points: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SegmentationPoint>,

}



impl Default for SegmentationResult {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__SegmentationResult__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__SegmentationResult__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SegmentationResult {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationResult__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationResult__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__SegmentationResult__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SegmentationResult {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SegmentationResult where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/SegmentationResult";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__SegmentationResult() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__ImageHeadPosition() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__ImageHeadPosition__init(msg: *mut ImageHeadPosition) -> bool;
    fn vision_interface__msg__ImageHeadPosition__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ImageHeadPosition>, size: usize) -> bool;
    fn vision_interface__msg__ImageHeadPosition__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ImageHeadPosition>);
    fn vision_interface__msg__ImageHeadPosition__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ImageHeadPosition>, out_seq: *mut rosidl_runtime_rs::Sequence<ImageHeadPosition>) -> bool;
}

// Corresponds to vision_interface__msg__ImageHeadPosition
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ImageHeadPosition {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub image: sensor_msgs::msg::rmw::Image,

}



impl Default for ImageHeadPosition {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__ImageHeadPosition__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__ImageHeadPosition__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ImageHeadPosition {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__ImageHeadPosition__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__ImageHeadPosition__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__ImageHeadPosition__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ImageHeadPosition {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ImageHeadPosition where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/ImageHeadPosition";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__ImageHeadPosition() }
  }
}


#[link(name = "vision_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__Ball() -> *const std::ffi::c_void;
}

#[link(name = "vision_interface__rosidl_generator_c")]
extern "C" {
    fn vision_interface__msg__Ball__init(msg: *mut Ball) -> bool;
    fn vision_interface__msg__Ball__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Ball>, size: usize) -> bool;
    fn vision_interface__msg__Ball__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Ball>);
    fn vision_interface__msg__Ball__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Ball>, out_seq: *mut rosidl_runtime_rs::Sequence<Ball>) -> bool;
}

// Corresponds to vision_interface__msg__Ball
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Ball {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub confidence: f64,

}



impl Default for Ball {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !vision_interface__msg__Ball__init(&mut msg as *mut _) {
        panic!("Call to vision_interface__msg__Ball__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Ball {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__Ball__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__Ball__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { vision_interface__msg__Ball__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Ball {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Ball where Self: Sized {
  const TYPE_NAME: &'static str = "vision_interface/msg/Ball";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__vision_interface__msg__Ball() }
  }
}


