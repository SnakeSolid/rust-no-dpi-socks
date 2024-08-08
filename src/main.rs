mod arguments;
mod context;
mod data;
mod request;
mod response;

use std::error::Error;
use std::fmt::Display;

use arguments::Arguments;
use clap::Parser;
use context::Context;
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

use data::AuthMethod;
use request::AuthMethodsRequest;
use request::CommandRequest;

use crate::data::Command;
use crate::response::AuthMethodResponse;
use crate::response::CommandResponse;

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

    let arguments = Arguments::parse();
    let runtime = Builder::new_multi_thread().enable_io().build()?;
    let context = Context::create(arguments, runtime);

    context.runtime().block_on(async {
        let listener = log_error!(
            TcpListener::bind((context.bind_address(), context.bind_port())).await,
            "Failed to bind address"
        );

        if let Ok(bind_address) = listener.local_addr() {
            info!("Listening on {}", bind_address);
        } else {
            info!("Listening...");
        }

        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    let cntxt = context.clone();

                    context.runtime().spawn(async move {
                        match handle_client(cntxt, socket).await {
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

async fn handle_client(context: Context, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let source_address = stream.peer_addr()?;

    info!("Connection from {}", source_address);

    let auth_request = AuthMethodsRequest::read(&mut stream).await?;

    if !auth_request
        .methods()
        .contains(&AuthMethod::NoAuthenticationRequired)
    {
        return Ok(());
    }

    let response = AuthMethodResponse::create(SOCKS5_VERSION, AuthMethod::NoAuthenticationRequired);
    response.send(&mut stream).await?;

    let command_request = CommandRequest::read(&mut stream).await?;

    if command_request.command() != Command::Connect {
        let response = CommandResponse::command_not_supported(SOCKS5_VERSION);
        response.send(&mut stream).await?;
    } else if let Some(destination_address) = command_request.destination() {
        let destination = TcpStream::connect(destination_address).await?;
        destination.set_nodelay(true)?;
        stream.set_nodelay(true)?;

        let response = CommandResponse::success(SOCKS5_VERSION, &destination_address);
        response.send(&mut stream).await?;

        let (src_read, src_write) = split(stream);
        let (dst_read, dst_write) = split(destination);

        info!(
            "{} -> {}: start transfer",
            source_address, destination_address
        );

        let n_bytes = context.n_bytes();
        let copy_to = context.runtime().spawn(async move {
            copy_data(src_read, dst_write, n_bytes)
                .await
                .map_err(|error| error.to_string())
        });
        let copy_from = context.runtime().spawn(async move {
            copy_data(dst_read, src_write, 0)
                .await
                .map_err(|error| error.to_string())
        });

        let (in_result, out_result) = join!(copy_to, copy_from);

        info!(
            "{} -> {}: {}, {}",
            source_address,
            destination_address,
            bytes_string(in_result?, "in bytes: ", "error: "),
            bytes_string(out_result?, "out bytes: ", "error: ")
        );
    } else {
        let response = CommandResponse::host_unreachable(SOCKS5_VERSION);
        response.send(&mut stream).await?;
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
