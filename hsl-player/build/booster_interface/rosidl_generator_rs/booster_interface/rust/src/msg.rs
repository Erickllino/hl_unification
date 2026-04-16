#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to booster_interface__msg__BoosterApiReqMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoosterApiReqMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub api_id: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for BoosterApiReqMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BoosterApiReqMsg::default())
  }
}

impl rosidl_runtime_rs::Message for BoosterApiReqMsg {
  type RmwMsg = super::msg::rmw::BoosterApiReqMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        api_id: msg.api_id,
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      api_id: msg.api_id,
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      api_id: msg.api_id,
      body: msg.body.to_string(),
    }
  }
}


// Corresponds to booster_interface__msg__BoosterApiRespMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoosterApiRespMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for BoosterApiRespMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BoosterApiRespMsg::default())
  }
}

impl rosidl_runtime_rs::Message for BoosterApiRespMsg {
  type RmwMsg = super::msg::rmw::BoosterApiRespMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      body: msg.body.to_string(),
    }
  }
}


// Corresponds to booster_interface__msg__ButtonEventMsg
/// constants

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ButtonEventMsg::default())
  }
}

impl rosidl_runtime_rs::Message for ButtonEventMsg {
  type RmwMsg = super::msg::rmw::ButtonEventMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        event: msg.event,
        button: msg.button,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      event: msg.event,
      button: msg.button,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      event: msg.event,
      button: msg.button,
    }
  }
}


// Corresponds to booster_interface__msg__MotorState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorState::default())
  }
}

impl rosidl_runtime_rs::Message for MotorState {
  type RmwMsg = super::msg::rmw::MotorState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        q: msg.q,
        dq: msg.dq,
        ddq: msg.ddq,
        tau_est: msg.tau_est,
        temperature: msg.temperature,
        lost: msg.lost,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      ddq: msg.ddq,
      tau_est: msg.tau_est,
      temperature: msg.temperature,
      lost: msg.lost,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      ddq: msg.ddq,
      tau_est: msg.tau_est,
      temperature: msg.temperature,
      lost: msg.lost,
      reserve: msg.reserve,
    }
  }
}


// Corresponds to booster_interface__msg__MotorCmd

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorCmd::default())
  }
}

impl rosidl_runtime_rs::Message for MotorCmd {
  type RmwMsg = super::msg::rmw::MotorCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        q: msg.q,
        dq: msg.dq,
        tau: msg.tau,
        kp: msg.kp,
        kd: msg.kd,
        weight: msg.weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      tau: msg.tau,
      kp: msg.kp,
      kd: msg.kd,
      weight: msg.weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      tau: msg.tau,
      kp: msg.kp,
      kd: msg.kd,
      weight: msg.weight,
    }
  }
}


// Corresponds to booster_interface__msg__LowCmd
/// constants

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowCmd {
    /// fields
    /// use CMD_TYPE_PARALLEL or CMD_TYPE_SERIAL
    pub cmd_type: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_cmd: Vec<super::msg::MotorCmd>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LowCmd::default())
  }
}

impl rosidl_runtime_rs::Message for LowCmd {
  type RmwMsg = super::msg::rmw::LowCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cmd_type: msg.cmd_type,
        motor_cmd: msg.motor_cmd
          .into_iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cmd_type: msg.cmd_type,
        motor_cmd: msg.motor_cmd
          .iter()
          .map(|elem| super::msg::MotorCmd::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cmd_type: msg.cmd_type,
      motor_cmd: msg.motor_cmd
          .into_iter()
          .map(super::msg::MotorCmd::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to booster_interface__msg__ImuState
/// 欧拉角信息（0 -> roll ,0 -> pitch ,0 -> yaw）

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ImuState::default())
  }
}

impl rosidl_runtime_rs::Message for ImuState {
  type RmwMsg = super::msg::rmw::ImuState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rpy: msg.rpy,
        gyro: msg.gyro,
        acc: msg.acc,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rpy: msg.rpy,
        gyro: msg.gyro,
        acc: msg.acc,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      rpy: msg.rpy,
      gyro: msg.gyro,
      acc: msg.acc,
    }
  }
}


// Corresponds to booster_interface__msg__LowState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::msg::ImuState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state_parallel: Vec<super::msg::MotorState>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state_serial: Vec<super::msg::MotorState>,

}



impl Default for LowState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LowState::default())
  }
}

impl rosidl_runtime_rs::Message for LowState {
  type RmwMsg = super::msg::rmw::LowState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        imu_state: super::msg::ImuState::into_rmw_message(std::borrow::Cow::Owned(msg.imu_state)).into_owned(),
        motor_state_parallel: msg.motor_state_parallel
          .into_iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        motor_state_serial: msg.motor_state_serial
          .into_iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        imu_state: super::msg::ImuState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.imu_state)).into_owned(),
        motor_state_parallel: msg.motor_state_parallel
          .iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        motor_state_serial: msg.motor_state_serial
          .iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      imu_state: super::msg::ImuState::from_rmw_message(msg.imu_state),
      motor_state_parallel: msg.motor_state_parallel
          .into_iter()
          .map(super::msg::MotorState::from_rmw_message)
          .collect(),
      motor_state_serial: msg.motor_state_serial
          .into_iter()
          .map(super::msg::MotorState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to booster_interface__msg__RawBytesMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawBytesMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: Vec<u8>,

}



impl Default for RawBytesMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RawBytesMsg::default())
  }
}

impl rosidl_runtime_rs::Message for RawBytesMsg {
  type RmwMsg = super::msg::rmw::RawBytesMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        msg: msg.msg.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        msg: msg.msg.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      msg: msg.msg
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to booster_interface__msg__RawBytesStamped

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawBytesStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: Vec<u8>,

}



impl Default for RawBytesStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RawBytesStamped::default())
  }
}

impl rosidl_runtime_rs::Message for RawBytesStamped {
  type RmwMsg = super::msg::rmw::RawBytesStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        msg: msg.msg.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        msg: msg.msg.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      msg: msg.msg
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to booster_interface__msg__Odometer

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Odometer::default())
  }
}

impl rosidl_runtime_rs::Message for Odometer {
  type RmwMsg = super::msg::rmw::Odometer;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        theta: msg.theta,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      theta: msg.theta,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      theta: msg.theta,
    }
  }
}


// Corresponds to booster_interface__msg__FallDownState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FallDownState::default())
  }
}

impl rosidl_runtime_rs::Message for FallDownState {
  type RmwMsg = super::msg::rmw::FallDownState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        fall_down_state: msg.fall_down_state,
        is_recovery_available: msg.is_recovery_available,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      fall_down_state: msg.fall_down_state,
      is_recovery_available: msg.is_recovery_available,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      fall_down_state: msg.fall_down_state,
      is_recovery_available: msg.is_recovery_available,
    }
  }
}


// Corresponds to booster_interface__msg__RemoteControllerState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RemoteControllerState::default())
  }
}

impl rosidl_runtime_rs::Message for RemoteControllerState {
  type RmwMsg = super::msg::rmw::RemoteControllerState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        event: msg.event,
        lx: msg.lx,
        ly: msg.ly,
        rx: msg.rx,
        ry: msg.ry,
        a: msg.a,
        b: msg.b,
        x: msg.x,
        y: msg.y,
        lb: msg.lb,
        rb: msg.rb,
        lt: msg.lt,
        rt: msg.rt,
        ls: msg.ls,
        rs: msg.rs,
        back: msg.back,
        start: msg.start,
        hat_c: msg.hat_c,
        hat_u: msg.hat_u,
        hat_d: msg.hat_d,
        hat_l: msg.hat_l,
        hat_r: msg.hat_r,
        hat_lu: msg.hat_lu,
        hat_ld: msg.hat_ld,
        hat_ru: msg.hat_ru,
        hat_rd: msg.hat_rd,
        hat_pos: msg.hat_pos,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      event: msg.event,
      lx: msg.lx,
      ly: msg.ly,
      rx: msg.rx,
      ry: msg.ry,
      a: msg.a,
      b: msg.b,
      x: msg.x,
      y: msg.y,
      lb: msg.lb,
      rb: msg.rb,
      lt: msg.lt,
      rt: msg.rt,
      ls: msg.ls,
      rs: msg.rs,
      back: msg.back,
      start: msg.start,
      hat_c: msg.hat_c,
      hat_u: msg.hat_u,
      hat_d: msg.hat_d,
      hat_l: msg.hat_l,
      hat_r: msg.hat_r,
      hat_lu: msg.hat_lu,
      hat_ld: msg.hat_ld,
      hat_ru: msg.hat_ru,
      hat_rd: msg.hat_rd,
      hat_pos: msg.hat_pos,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      event: msg.event,
      lx: msg.lx,
      ly: msg.ly,
      rx: msg.rx,
      ry: msg.ry,
      a: msg.a,
      b: msg.b,
      x: msg.x,
      y: msg.y,
      lb: msg.lb,
      rb: msg.rb,
      lt: msg.lt,
      rt: msg.rt,
      ls: msg.ls,
      rs: msg.rs,
      back: msg.back,
      start: msg.start,
      hat_c: msg.hat_c,
      hat_u: msg.hat_u,
      hat_d: msg.hat_d,
      hat_l: msg.hat_l,
      hat_r: msg.hat_r,
      hat_lu: msg.hat_lu,
      hat_ld: msg.hat_ld,
      hat_ru: msg.hat_ru,
      hat_rd: msg.hat_rd,
      hat_pos: msg.hat_pos,
    }
  }
}


// Corresponds to booster_interface__msg__HandCommand

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hand_param: Vec<super::msg::HandParam>,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HandCommand::default())
  }
}

impl rosidl_runtime_rs::Message for HandCommand {
  type RmwMsg = super::msg::rmw::HandCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hand_param: msg.hand_param
          .into_iter()
          .map(|elem| super::msg::HandParam::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        force_mode: msg.force_mode,
        hand_index: msg.hand_index,
        hand_type: msg.hand_type,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hand_param: msg.hand_param
          .iter()
          .map(|elem| super::msg::HandParam::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      force_mode: msg.force_mode,
      hand_index: msg.hand_index,
      hand_type: msg.hand_type,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      hand_param: msg.hand_param
          .into_iter()
          .map(super::msg::HandParam::from_rmw_message)
          .collect(),
      force_mode: msg.force_mode,
      hand_index: msg.hand_index,
      hand_type: msg.hand_type,
    }
  }
}


// Corresponds to booster_interface__msg__HandDdsMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandDdsMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hands_vec: Vec<super::msg::HandCommand>,

}



impl Default for HandDdsMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HandDdsMsg::default())
  }
}

impl rosidl_runtime_rs::Message for HandDdsMsg {
  type RmwMsg = super::msg::rmw::HandDdsMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hands_vec: msg.hands_vec
          .into_iter()
          .map(|elem| super::msg::HandCommand::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hands_vec: msg.hands_vec
          .iter()
          .map(|elem| super::msg::HandCommand::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      hands_vec: msg.hands_vec
          .into_iter()
          .map(super::msg::HandCommand::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to booster_interface__msg__HandParam

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HandParam::default())
  }
}

impl rosidl_runtime_rs::Message for HandParam {
  type RmwMsg = super::msg::rmw::HandParam;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        angle: msg.angle,
        force: msg.force,
        speed: msg.speed,
        seq: msg.seq,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      angle: msg.angle,
      force: msg.force,
      speed: msg.speed,
      seq: msg.seq,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      angle: msg.angle,
      force: msg.force,
      speed: msg.speed,
      seq: msg.seq,
    }
  }
}


