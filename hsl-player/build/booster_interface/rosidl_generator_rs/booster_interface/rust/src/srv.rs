#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to booster_interface__srv__RpcService_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcService_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: super::msg::BoosterApiReqMsg,

}



impl Default for RpcService_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RpcService_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RpcService_Request {
  type RmwMsg = super::srv::rmw::RpcService_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        msg: super::msg::BoosterApiReqMsg::into_rmw_message(std::borrow::Cow::Owned(msg.msg)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        msg: super::msg::BoosterApiReqMsg::into_rmw_message(std::borrow::Cow::Borrowed(&msg.msg)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      msg: super::msg::BoosterApiReqMsg::from_rmw_message(msg.msg),
    }
  }
}


// Corresponds to booster_interface__srv__RpcService_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RpcService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub msg: super::msg::BoosterApiRespMsg,

}



impl Default for RpcService_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RpcService_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RpcService_Response {
  type RmwMsg = super::srv::rmw::RpcService_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        msg: super::msg::BoosterApiRespMsg::into_rmw_message(std::borrow::Cow::Owned(msg.msg)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        msg: super::msg::BoosterApiRespMsg::into_rmw_message(std::borrow::Cow::Borrowed(&msg.msg)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      msg: super::msg::BoosterApiRespMsg::from_rmw_message(msg.msg),
    }
  }
}


// Corresponds to booster_interface__srv__AgentService_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgentService_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for AgentService_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AgentService_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AgentService_Request {
  type RmwMsg = super::srv::rmw::AgentService_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      body: msg.body.to_string(),
    }
  }
}


// Corresponds to booster_interface__srv__AgentService_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgentService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub body: std::string::String,

}



impl Default for AgentService_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AgentService_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AgentService_Response {
  type RmwMsg = super::srv::rmw::AgentService_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body: msg.body.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        body: msg.body.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      body: msg.body.to_string(),
    }
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


