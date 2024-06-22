#[macro_export]
macro_rules! dispatch_network {
    ($network:expr, $fn:ident $(, $arg:expr)*) => {
        match $network {
            "TestnetV0" => $fn::<crate::types::native::TestnetV0>($($arg),*),
            "MainnetV0" => $fn::<crate::types::native::MainnetV0>($($arg),*),
            _ => Err(format!("Unsupported network: {}", $network)),
        }
    };
}

#[macro_export]
macro_rules! dispatch_network_self {
    ($network:expr, $fn:ident, $self:ident $(, $arg:expr)*) => {
        match $network {
            "TestnetV0" => $self.$fn::<crate::types::native::TestnetV0>($($arg),*),
            "MainnetV0" => $self.$fn::<crate::types::native::MainnetV0>($($arg),*),
            _ => Err(format!("Unsupported network: {}", $network)),
        }
    };
}

#[macro_export]
macro_rules! dispatch_network_aleo {
    ($network:expr, $fn:ident $(, $arg:expr)*) => {
        match $network {
            "TestnetV0" => $fn::<crate::types::native::TestnetV0, crate::types::native::AleoTestnetV0>($($arg),*),
            "MainnetV0" => $fn::<crate::types::native::MainnetV0, crate::types::native::AleoV0>($($arg),*),
            _ => Err(format!("Unsupported network: {}", $network)),
        }
    };
}