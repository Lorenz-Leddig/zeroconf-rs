use crate::NetworkInterface;

pub const WINDOWS_INTERFACE_UNSPECIFIED: u32 = 0;

/// Converts the specified [`NetworkInterface`] to the Windows expected value.
pub fn interface_index(interface: NetworkInterface) -> u32 {
    match interface {
        NetworkInterface::Unspec => WINDOWS_INTERFACE_UNSPECIFIED,
        NetworkInterface::AtIndex(index) => index,
    }
}

/// Converts the specified Windows interface index to a [`NetworkInterface`].
pub fn interface_from_index(index: u32) -> NetworkInterface {
    match index {
        WINDOWS_INTERFACE_UNSPECIFIED => NetworkInterface::Unspec,
        index => NetworkInterface::AtIndex(index),
    }
}
