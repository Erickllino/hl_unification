#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to booster_msgs__msg__BinaryData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BinaryData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<u8>,

}



impl Default for BinaryData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BinaryData::default())
  }
}

impl rosidl_runtime_rs::Message for BinaryData {
  type RmwMsg = super::msg::rmw::BinaryData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to booster_msgs__msg__RpcReqMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcReqMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for RpcReqMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RpcReqMsg::default())
  }
}

impl rosidl_runtime_rs::Message for RpcReqMsg {
  type RmwMsg = super::msg::rmw::RpcReqMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        header: msg.header.as_str().into(),
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        header: msg.header.as_str().into(),
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      uuid: msg.uuid.to_string(),
      header: msg.header.to_string(),
      body: msg.body.to_string(),
    }
  }
}


// Corresponds to booster_msgs__msg__RpcRespMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcRespMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for RpcRespMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RpcRespMsg::default())
  }
}

impl rosidl_runtime_rs::Message for RpcRespMsg {
  type RmwMsg = super::msg::rmw::RpcRespMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        header: msg.header.as_str().into(),
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        uuid: msg.uuid.as_str().into(),
        header: msg.header.as_str().into(),
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      uuid: msg.uuid.to_string(),
      header: msg.header.to_string(),
      body: msg.body.to_string(),
    }
  }
}


