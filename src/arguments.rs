use clap_derive::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(
        short = 'a',
        long,
        default_value = "localhost",
        value_name = "ADDRESS",
        help = "bind to address"
    )]
    bind_address: String,

    #[arg(
        short = 'p',
        long,
        default_value_t = 1080,
        value_name = "PORT",
        help = "bind to port"
    )]
    bind_port: u16,

    #[arg(
        short = 'c',
        long,
        default_value_t = 1024,
        value_name = "N",
        help = "number of one-bytes packets"
    )]
    n_bytes: usize,
}

impl Arguments {
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }

    pub fn bind_port(&self) -> u16 {
        self.bind_port
    }

    pub fn n_bytes(&self) -> usize {
        self.n_bytes
    }
}
