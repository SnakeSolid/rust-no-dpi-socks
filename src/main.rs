mod data;
mod request;

use std::error::Error;
use std::fmt::Display;
use std::sync::Arc;

use log::error;
use log::info;
use tokio::io::split;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::join;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

use data::AuthMethod;
use request::AuthMethodsRequest;
use request::CommandRequest;

const SOCKS5_VERSION: u8 = 0x05;

macro_rules! log_error {
    ($e:expr, $message:expr) => {{
        match $e {
            Ok(value) => value,
            Err(error) => {
                error!("{}: {}", $message, error);

                return;
            }
        }
    }};
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let rt = Builder::new_multi_thread().enable_io().build()?;
    let rt = Arc::new(rt);

    rt.block_on(async {
        let listener = log_error!(
            TcpListener::bind("127.0.0.1:1080").await,
            "Failed to bind address"
        );

        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    let runtime = rt.clone();

                    rt.spawn(async move {
                        match handle_client(runtime, socket).await {
                            Ok(()) => {}
                            Err(error) => eprintln!("{}", error),
                        }
                    });
                }
                Err(error) => eprintln!("{}", error),
            }
        }
    });

    Ok(())
}

async fn handle_client(rt: Arc<Runtime>, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let source_address = stream.peer_addr()?;

    info!("Connection from {}", source_address);

    let auth_request = AuthMethodsRequest::read(&mut stream).await?;

    if !auth_request
        .methods()
        .contains(&AuthMethod::NoAuthenticationRequired)
    {
        return Ok(());
    }

    stream
        .write_all(&[SOCKS5_VERSION, AuthMethod::NoAuthenticationRequired.into()])
        .await?;

    let command_request = CommandRequest::read(&mut stream).await?;

    if let Some(destination_address) = command_request.destination() {
        let destination = TcpStream::connect(destination_address).await?;
        destination.set_nodelay(true)?;
        stream.set_nodelay(true)?;

        stream
            .write_all(&[
                SOCKS5_VERSION,
                0x00,
                0x00,
                0x01,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ])
            .await?;

        let (src_read, src_write) = split(stream);
        let (dst_read, dst_write) = split(destination);

        info!(
            "{} -> {}: start transfer",
            source_address, destination_address
        );

        let copy_to = rt.spawn(async move {
            copy_data(src_read, dst_write, 1024)
                .await
                .map_err(|error| error.to_string())
        });
        let copy_from = rt.spawn(async move {
            copy_data(dst_read, src_write, 0)
                .await
                .map_err(|error| error.to_string())
        });

        let (in_result, out_result) = join!(copy_to, copy_from);

        info!(
            "{} -> {}: {:?}, {:?}",
            source_address,
            destination_address,
            bytes_string(in_result?, "in bytes: ", "error: "),
            bytes_string(out_result?, "out bytes: ", "error: ")
        );
    } else {
        stream
            .write_all(&[SOCKS5_VERSION, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00])
            .await?;
    }

    Ok(())
}

fn bytes_string<T, E>(result: Result<T, E>, success: &str, failed: &str) -> String
where
    T: Display,
    E: Display,
{
    match result {
        Ok(value) => format!("{}{}", success, value),
        Err(error) => format!("{}{}", failed, error),
    }
}

async fn copy_data(
    mut read: ReadHalf<TcpStream>,
    mut write: WriteHalf<TcpStream>,
    slow_bytes: usize,
) -> Result<usize, Box<dyn Error>> {
    let mut buffer = [0; 8192];
    let mut count = 0;

    for _ in 0..slow_bytes {
        match read.read(&mut buffer[..1]).await? {
            0 => return Ok(count),
            length => {
                write.write_all(&buffer[..length]).await?;

                count += length;
            }
        }

        write.flush().await?;
    }

    loop {
        match read.read(&mut buffer).await? {
            0 => return Ok(count),
            length => {
                write.write_all(&buffer[..length]).await?;

                count += length;
            }
        }

        write.flush().await?;
    }
}
