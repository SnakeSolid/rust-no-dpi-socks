use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use bytes::BufMut;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::data::{AuthMethod, CommandReply};

#[derive(Debug)]
pub struct AuthMethodResponse {
    version: u8,
    method: AuthMethod,
}

impl AuthMethodResponse {
    pub fn create(version: u8, method: AuthMethod) -> Self {
        Self { version, method }
    }

    pub async fn send(&self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        stream
            .write_all(&[self.version, self.method.into()])
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct CommandResponse {
    version: u8,
    reply: CommandReply,
    address: SocketAddr,
}

impl CommandResponse {
    pub fn success(version: u8, address: &SocketAddr) -> Self {
        Self {
            version,
            reply: CommandReply::Success,
            address: address.clone(),
        }
    }

    pub fn command_not_supported(version: u8) -> Self {
        Self {
            version,
            reply: CommandReply::CommandNotSupported,
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        }
    }

    pub fn host_unreachable(version: u8) -> Self {
        Self {
            version,
            reply: CommandReply::HostUnreachable,
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        }
    }

    pub async fn send(&self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer: Vec<u8> = Vec::with_capacity(96);
        buffer.put_u8(self.version);
        buffer.put_u8(self.reply.into());
        buffer.put_u8(0);

        match self.address {
            SocketAddr::V4(address) => {
                buffer.put_u8(1);
                buffer.extend(address.ip().octets());
            }
            SocketAddr::V6(address) => {
                buffer.put_u8(4);
                buffer.extend(address.ip().octets());
            }
        }

        buffer.put_u16_ne(self.address.port());

        stream.write_all(&buffer).await?;

        Ok(())
    }
}
