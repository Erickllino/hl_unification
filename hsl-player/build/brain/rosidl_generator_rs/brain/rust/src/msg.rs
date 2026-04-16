#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to brain__msg__Kick

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Kick {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Kick::default())
  }
}

impl rosidl_runtime_rs::Message for Kick {
  type RmwMsg = super::msg::rmw::Kick;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        x: msg.x,
        y: msg.y,
        dir: msg.dir,
        goal_x: msg.goal_x,
        goal_y: msg.goal_y,
        robot_theta_to_field: msg.robot_theta_to_field,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      x: msg.x,
      y: msg.y,
      dir: msg.dir,
      goal_x: msg.goal_x,
      goal_y: msg.goal_y,
      robot_theta_to_field: msg.robot_theta_to_field,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      x: msg.x,
      y: msg.y,
      dir: msg.dir,
      goal_x: msg.goal_x,
      goal_y: msg.goal_y,
      robot_theta_to_field: msg.robot_theta_to_field,
    }
  }
}


