#[macro_export]
macro_rules! dispatch_network {
    ($network:expr, $fn:ident $(, $arg:expr)*) => {
        match $network {
            "CanaryV0" => $fn::<crate::types::native::CanaryV0>($($arg),*),
            "TestnetV0" => $fn::<crate::types::native::TestnetV0>($($arg),*),
            "MainnetV0" => $fn::<crate::types::native::MainnetV0>($($arg),*),
            _ => Err(format!("Unsupported network: {}", $network)),
        }
    };
}

#[macro_export]
macro_rules! network_string_id {
  ($network_id:expr) => {
      match $network_id {
        2u16 => Ok("CanaryV0"),
        1u16 => Ok("TestnetV0"),
        0u16 => Ok("MainnetV0"),
        _ => Err(format!("Unsupported network: {:?}", $network_id)),
      }
  };
}

#[macro_export]
macro_rules! dispatch_network_aleo {
    ($network:expr, $fn:ident $(, $arg:expr)*) => {
        match $network {
            "CanaryV0" => $fn::<crate::types::native::CanaryV0, crate::types::native::AleoCanaryV0>($($arg),*),
            "TestnetV0" => $fn::<crate::types::native::TestnetV0, crate::types::native::AleoTestnetV0>($($arg),*),
            "MainnetV0" => $fn::<crate::types::native::MainnetV0, crate::types::native::AleoV0>($($arg),*),
            _ => Err(format!("Unsupported network: {}", $network)),
        }
    };
}

#[macro_export]
macro_rules! dispatch_network_aleo_async {
    ($network:expr, $fn:ident $(, $arg:expr)*) => {
        match $network {
            "CanaryV0" => $fn::<crate::types::native::CanaryV0, crate::types::native::AleoCanaryV0>($($arg),*).await,
            "TestnetV0" => $fn::<crate::types::native::TestnetV0, crate::types::native::AleoTestnetV0>($($arg),*).await,
            "MainnetV0" => $fn::<crate::types::native::MainnetV0, crate::types::native::AleoV0>($($arg),*).await,
            _ => Err(format!("Unsupported network: {}", $network)),
        }
    };
}