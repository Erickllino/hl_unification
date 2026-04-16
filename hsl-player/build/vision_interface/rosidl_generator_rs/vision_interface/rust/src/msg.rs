#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to vision_interface__msg__DetectedObject

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedObject {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub color: std::string::String,


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
    pub target_uv: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub received_pos: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_projection: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_cam: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_confidence: i32,

}



impl Default for DetectedObject {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DetectedObject::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedObject {
  type RmwMsg = super::msg::rmw::DetectedObject;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
        color: msg.color.as_str().into(),
        confidence: msg.confidence,
        xmin: msg.xmin,
        ymin: msg.ymin,
        xmax: msg.xmax,
        ymax: msg.ymax,
        target_uv: msg.target_uv.into(),
        received_pos: msg.received_pos.into(),
        position: msg.position.into(),
        position_projection: msg.position_projection.into(),
        position_cam: msg.position_cam.into(),
        position_confidence: msg.position_confidence,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
        color: msg.color.as_str().into(),
      confidence: msg.confidence,
      xmin: msg.xmin,
      ymin: msg.ymin,
      xmax: msg.xmax,
      ymax: msg.ymax,
        target_uv: msg.target_uv.as_slice().into(),
        received_pos: msg.received_pos.as_slice().into(),
        position: msg.position.as_slice().into(),
        position_projection: msg.position_projection.as_slice().into(),
        position_cam: msg.position_cam.as_slice().into(),
      position_confidence: msg.position_confidence,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      label: msg.label.to_string(),
      color: msg.color.to_string(),
      confidence: msg.confidence,
      xmin: msg.xmin,
      ymin: msg.ymin,
      xmax: msg.xmax,
      ymax: msg.ymax,
      target_uv: msg.target_uv
          .into_iter()
          .collect(),
      received_pos: msg.received_pos
          .into_iter()
          .collect(),
      position: msg.position
          .into_iter()
          .collect(),
      position_projection: msg.position_projection
          .into_iter()
          .collect(),
      position_cam: msg.position_cam
          .into_iter()
          .collect(),
      position_confidence: msg.position_confidence,
    }
  }
}


// Corresponds to vision_interface__msg__Detections

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Detections {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub detected_objects: Vec<super::msg::DetectedObject>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_x: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_y: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub corner_pos: Vec<f32>,

}



impl Default for Detections {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Detections::default())
  }
}

impl rosidl_runtime_rs::Message for Detections {
  type RmwMsg = super::msg::rmw::Detections;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        detected_objects: msg.detected_objects
          .into_iter()
          .map(|elem| super::msg::DetectedObject::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        radar_x: msg.radar_x.into(),
        radar_y: msg.radar_y.into(),
        corner_pos: msg.corner_pos.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        detected_objects: msg.detected_objects
          .iter()
          .map(|elem| super::msg::DetectedObject::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        radar_x: msg.radar_x.as_slice().into(),
        radar_y: msg.radar_y.as_slice().into(),
        corner_pos: msg.corner_pos.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      detected_objects: msg.detected_objects
          .into_iter()
          .map(super::msg::DetectedObject::from_rmw_message)
          .collect(),
      radar_x: msg.radar_x
          .into_iter()
          .collect(),
      radar_y: msg.radar_y
          .into_iter()
          .collect(),
      corner_pos: msg.corner_pos
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to vision_interface__msg__LineSegments

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LineSegments {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coordinates: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub coordinates_uv: Vec<f32>,

}



impl Default for LineSegments {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LineSegments::default())
  }
}

impl rosidl_runtime_rs::Message for LineSegments {
  type RmwMsg = super::msg::rmw::LineSegments;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        coordinates: msg.coordinates.into(),
        coordinates_uv: msg.coordinates_uv.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        coordinates: msg.coordinates.as_slice().into(),
        coordinates_uv: msg.coordinates_uv.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      coordinates: msg.coordinates
          .into_iter()
          .collect(),
      coordinates_uv: msg.coordinates_uv
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to vision_interface__msg__CalParam

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CalParam::default())
  }
}

impl rosidl_runtime_rs::Message for CalParam {
  type RmwMsg = super::msg::rmw::CalParam;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pitch_compensation: msg.pitch_compensation,
        yaw_compensation: msg.yaw_compensation,
        z_compensation: msg.z_compensation,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      pitch_compensation: msg.pitch_compensation,
      yaw_compensation: msg.yaw_compensation,
      z_compensation: msg.z_compensation,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pitch_compensation: msg.pitch_compensation,
      yaw_compensation: msg.yaw_compensation,
      z_compensation: msg.z_compensation,
    }
  }
}


// Corresponds to vision_interface__msg__DetectionsTrack

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectionsTrack {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// "yolo" or "pysot" or "none"
    pub ball_type: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub detected_objects: Vec<super::msg::DetectedObject>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_x: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radar_y: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub corner_pos: Vec<f32>,

}



impl Default for DetectionsTrack {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DetectionsTrack::default())
  }
}

impl rosidl_runtime_rs::Message for DetectionsTrack {
  type RmwMsg = super::msg::rmw::DetectionsTrack;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        ball_type: msg.ball_type.as_str().into(),
        detected_objects: msg.detected_objects
          .into_iter()
          .map(|elem| super::msg::DetectedObject::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        radar_x: msg.radar_x.into(),
        radar_y: msg.radar_y.into(),
        corner_pos: msg.corner_pos.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        ball_type: msg.ball_type.as_str().into(),
        detected_objects: msg.detected_objects
          .iter()
          .map(|elem| super::msg::DetectedObject::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        radar_x: msg.radar_x.as_slice().into(),
        radar_y: msg.radar_y.as_slice().into(),
        corner_pos: msg.corner_pos.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      ball_type: msg.ball_type.to_string(),
      detected_objects: msg.detected_objects
          .into_iter()
          .map(super::msg::DetectedObject::from_rmw_message)
          .collect(),
      radar_x: msg.radar_x
          .into_iter()
          .collect(),
      radar_y: msg.radar_y
          .into_iter()
          .collect(),
      corner_pos: msg.corner_pos
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to vision_interface__msg__SegmentationPoint

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationPoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SegmentationPoint::default())
  }
}

impl rosidl_runtime_rs::Message for SegmentationPoint {
  type RmwMsg = super::msg::rmw::SegmentationPoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
        x: msg.x,
        y: msg.y,
        u: msg.u,
        v: msg.v,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
      x: msg.x,
      y: msg.y,
      u: msg.u,
      v: msg.v,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      label: msg.label.to_string(),
      x: msg.x,
      y: msg.y,
      u: msg.u,
      v: msg.v,
    }
  }
}


// Corresponds to vision_interface__msg__SegmentationLine

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationLine {

    // This member is not documented.
    #[allow(missing_docs)]
    pub label: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SegmentationLine::default())
  }
}

impl rosidl_runtime_rs::Message for SegmentationLine {
  type RmwMsg = super::msg::rmw::SegmentationLine;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
        x1: msg.x1,
        y1: msg.y1,
        x2: msg.x2,
        y2: msg.y2,
        u1: msg.u1,
        v1: msg.v1,
        u2: msg.u2,
        v2: msg.v2,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        label: msg.label.as_str().into(),
      x1: msg.x1,
      y1: msg.y1,
      x2: msg.x2,
      y2: msg.y2,
      u1: msg.u1,
      v1: msg.v1,
      u2: msg.u2,
      v2: msg.v2,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      label: msg.label.to_string(),
      x1: msg.x1,
      y1: msg.y1,
      x2: msg.x2,
      y2: msg.y2,
      u1: msg.u1,
      v1: msg.v1,
      u2: msg.u2,
      v2: msg.v2,
    }
  }
}


// Corresponds to vision_interface__msg__SegmentationResult

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationResult {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lines: Vec<super::msg::SegmentationLine>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub points: Vec<super::msg::SegmentationPoint>,

}



impl Default for SegmentationResult {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SegmentationResult::default())
  }
}

impl rosidl_runtime_rs::Message for SegmentationResult {
  type RmwMsg = super::msg::rmw::SegmentationResult;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        lines: msg.lines
          .into_iter()
          .map(|elem| super::msg::SegmentationLine::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        points: msg.points
          .into_iter()
          .map(|elem| super::msg::SegmentationPoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        lines: msg.lines
          .iter()
          .map(|elem| super::msg::SegmentationLine::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        points: msg.points
          .iter()
          .map(|elem| super::msg::SegmentationPoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      lines: msg.lines
          .into_iter()
          .map(super::msg::SegmentationLine::from_rmw_message)
          .collect(),
      points: msg.points
          .into_iter()
          .map(super::msg::SegmentationPoint::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to vision_interface__msg__ImageHeadPosition

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ImageHeadPosition {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub image: sensor_msgs::msg::Image,

}



impl Default for ImageHeadPosition {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ImageHeadPosition::default())
  }
}

impl rosidl_runtime_rs::Message for ImageHeadPosition {
  type RmwMsg = super::msg::rmw::ImageHeadPosition;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        position: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        image: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Owned(msg.image)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        position: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
        image: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Borrowed(&msg.image)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      position: geometry_msgs::msg::Pose::from_rmw_message(msg.position),
      image: sensor_msgs::msg::Image::from_rmw_message(msg.image),
    }
  }
}


// Corresponds to vision_interface__msg__Ball

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Ball {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Ball::default())
  }
}

impl rosidl_runtime_rs::Message for Ball {
  type RmwMsg = super::msg::rmw::Ball;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        x: msg.x,
        y: msg.y,
        confidence: msg.confidence,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      x: msg.x,
      y: msg.y,
      confidence: msg.confidence,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      x: msg.x,
      y: msg.y,
      confidence: msg.confidence,
    }
  }
}


