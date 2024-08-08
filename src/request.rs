use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddr;

use dns_lookup::lookup_host;
use log::info;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::data::AddressType;
use crate::data::AuthMethod;
use crate::data::Command;

#[derive(Debug)]
pub struct AuthMethodsRequest {
    version: u8,
    methods: Vec<AuthMethod>,
}

impl AuthMethodsRequest {
    pub async fn read(stream: &mut TcpStream) -> Result<AuthMethodsRequest, Box<dyn Error>> {
        let version = stream.read_u8().await?;
        let n_methods = stream.read_u8().await? as usize;
        let mut methods = [0; 256];
        stream.read_exact(&mut methods[..n_methods]).await?;

        Ok(AuthMethodsRequest {
            version,
            methods: methods[..n_methods].iter().map(AuthMethod::from).collect(),
        })
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn methods(&self) -> &[AuthMethod] {
        &self.methods
    }
}

#[derive(Debug)]
pub struct CommandRequest {
    version: u8,
    command: Command,
    address_type: AddressType,
    destination: Option<SocketAddr>,
}

impl CommandRequest {
    pub async fn read(stream: &mut TcpStream) -> Result<CommandRequest, Box<dyn Error>> {
        let version = stream.read_u8().await?;
        let command = stream.read_u8().await?.into();
        let _reserved = stream.read_u8().await?;
        let address_type = stream.read_u8().await?.into();
        let mut destination_address = None;

        match address_type {
            AddressType::IpV4Address => {
                let mut buffer = [0; 4];
                stream.read_exact(&mut buffer).await?;

                destination_address = Some(IpAddr::V4(Ipv4Addr::from(buffer)));
            }
            AddressType::DomainName => {
                let mut buffer = [0; 256];
                let n_chars = stream.read_u8().await? as usize;
                stream.read_exact(&mut buffer[..n_chars]).await?;
                let domain_name = unsafe { std::str::from_utf8_unchecked(&buffer[..n_chars]) };
                let addresses = lookup_host(domain_name)?;

                info!("Resolve domain name `{}` into: {:?}", domain_name, addresses);

                if let Some(address) = addresses.get(0) {
                    destination_address = Some(*address);
                }
            }
            AddressType::IpV6Address => {
                let mut buffer = [0; 16];
                stream.read_exact(&mut buffer).await?;

                destination_address = Some(IpAddr::V6(Ipv6Addr::from(buffer)));
            }
            AddressType::Invalid { value } => unimplemented!("Invalid address type: {}", value),
        }

        let destination_port = stream.read_u16().await?.into();

        if let Some(destination_address) = destination_address {
            let destination = SocketAddr::new(destination_address, destination_port);

            Ok(CommandRequest {
                version,
                command,
                address_type,
                destination: Some(destination),
            })
        } else {
            Ok(CommandRequest {
                version,
                command,
                address_type,
                destination: None,
            })
        }
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn address_type(&self) -> &AddressType {
        &self.address_type
    }

    pub fn destination(&self) -> Option<&SocketAddr> {
        self.destination.as_ref()
    }
}
