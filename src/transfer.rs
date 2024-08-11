use std::error::Error;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct TransferError {
    count: usize,
    message: String,
}

impl TransferError {
    pub fn new<E>(count: usize, error: E) -> Self
    where
        E: Error,
    {
        let message = error.to_string();

        Self { count, message }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub async fn copy_data(
    mut read: ReadHalf<TcpStream>,
    mut write: WriteHalf<TcpStream>,
    slow_bytes: usize,
) -> Result<usize, TransferError> {
    let mut buffer = [0; 8192];
    let mut count = 0;

    for _ in 0..slow_bytes {
        match read
            .read(&mut buffer[..1])
            .await
            .map_err(|error| TransferError::new(count, error))?
        {
            0 => return Ok(count),
            length => {
                write
                    .write_all(&buffer[..length])
                    .await
                    .map_err(|error| TransferError::new(count, error))?;

                count += length;
            }
        }

        write
            .flush()
            .await
            .map_err(|error| TransferError::new(count, error))?;
    }

    loop {
        match read
            .read(&mut buffer)
            .await
            .map_err(|error| TransferError::new(count, error))?
        {
            0 => return Ok(count),
            length => {
                write
                    .write_all(&buffer[..length])
                    .await
                    .map_err(|error| TransferError::new(count, error))?;

                count += length;
            }
        }

        write
            .flush()
            .await
            .map_err(|error| TransferError::new(count, error))?;
    }
}
