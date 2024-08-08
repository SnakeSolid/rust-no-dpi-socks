#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AuthMethod {
    NoAuthenticationRequired,
    GssApi,
    UsernamePassword,
    IanaAssigned { method: u8 },
    PrivateMethod { method: u8 },
    NoAcceptableMethods,
}

impl From<u8> for AuthMethod {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NoAuthenticationRequired,
            0x01 => Self::GssApi,
            0x02 => Self::UsernamePassword,
            0x03..=0x7f => Self::IanaAssigned { method: value },
            0x80..=0xfe => Self::PrivateMethod { method: value },
            0xff => Self::NoAcceptableMethods,
        }
    }
}

impl From<&u8> for AuthMethod {
    fn from(value: &u8) -> Self {
        match value {
            0x00 => Self::NoAuthenticationRequired,
            0x01 => Self::GssApi,
            0x02 => Self::UsernamePassword,
            0x03..=0x7f => Self::IanaAssigned { method: *value },
            0x80..=0xfe => Self::PrivateMethod { method: *value },
            0xff => Self::NoAcceptableMethods,
        }
    }
}

impl Into<u8> for AuthMethod {
    fn into(self) -> u8 {
        match self {
            Self::NoAuthenticationRequired => 0x00,
            Self::GssApi => 0x01,
            Self::UsernamePassword => 0x02,
            Self::IanaAssigned { method } => method,
            Self::PrivateMethod { method } => method,
            Self::NoAcceptableMethods => 0xff,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Command {
    Connect,
    Bind,
    UdpAssociate,
    Invalid { value: u8 },
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::Connect,
            0x02 => Self::Bind,
            0x03 => Self::UdpAssociate,
            _ => Self::Invalid { value },
        }
    }
}

impl From<&u8> for Command {
    fn from(value: &u8) -> Self {
        match value {
            0x01 => Self::Connect,
            0x02 => Self::Bind,
            0x03 => Self::UdpAssociate,
            _ => Self::Invalid { value: *value },
        }
    }
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        match self {
            Self::Connect => 0x01,
            Self::Bind => 0x02,
            Self::UdpAssociate => 0x03,
            Self::Invalid { value } => value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AddressType {
    IpV4Address,
    DomainName,
    IpV6Address,
    Invalid { value: u8 },
}

impl From<u8> for AddressType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::IpV4Address,
            0x03 => Self::DomainName,
            0x04 => Self::IpV6Address,
            _ => Self::Invalid { value },
        }
    }
}

impl From<&u8> for AddressType {
    fn from(value: &u8) -> Self {
        match value {
            0x01 => Self::IpV4Address,
            0x03 => Self::DomainName,
            0x04 => Self::IpV6Address,
            _ => Self::Invalid { value: *value },
        }
    }
}

impl Into<u8> for AddressType {
    fn into(self) -> u8 {
        match self {
            Self::IpV4Address => 0x01,
            Self::DomainName => 0x03,
            Self::IpV6Address => 0x04,
            Self::Invalid { value } => value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CommandReply {
    Success,
    ServerFailure,
    ConnectionNotAllowed,
    NetworkUnreachable,
    HostUnreachable,
    ConnectionRefused,
    TtlExpired,
    CommandNotSupported,
    AddressTypeNotSupported,
    Unknown { value: u8 },
}

impl From<u8> for CommandReply {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Success,
            0x01 => Self::ServerFailure,
            0x02 => Self::ConnectionNotAllowed,
            0x03 => Self::NetworkUnreachable,
            0x04 => Self::HostUnreachable,
            0x05 => Self::ConnectionRefused,
            0x06 => Self::TtlExpired,
            0x07 => Self::CommandNotSupported,
            0x08 => Self::AddressTypeNotSupported,
            _ => Self::Unknown { value },
        }
    }
}

impl From<&u8> for CommandReply {
    fn from(value: &u8) -> Self {
        match value {
            0x00 => Self::Success,
            0x01 => Self::ServerFailure,
            0x02 => Self::ConnectionNotAllowed,
            0x03 => Self::NetworkUnreachable,
            0x04 => Self::HostUnreachable,
            0x05 => Self::ConnectionRefused,
            0x06 => Self::TtlExpired,
            0x07 => Self::CommandNotSupported,
            0x08 => Self::AddressTypeNotSupported,
            _ => Self::Unknown { value: *value },
        }
    }
}

impl Into<u8> for CommandReply {
    fn into(self) -> u8 {
        match self {
            Self::Success => 0x00,
            Self::ServerFailure => 0x01,
            Self::ConnectionNotAllowed => 0x02,
            Self::NetworkUnreachable => 0x03,
            Self::HostUnreachable => 0x04,
            Self::ConnectionRefused => 0x05,
            Self::TtlExpired => 0x06,
            Self::CommandNotSupported => 0x07,
            Self::AddressTypeNotSupported => 0x08,
            Self::Unknown { value } => value,
        }
    }
}
