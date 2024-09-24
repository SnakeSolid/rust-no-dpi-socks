# No DPI SOCKS

A simple SOCKS5 proxy designed to bypass YouTube slowdown. The server sends
first package byte by byte, making it difficult for Deep Packet Inspection
(DPI) to detect the nature of the connection.

## Build

To build `no-dpi-socks` from source code use following command:

```sh
cargo build --release
```

To start `no-dpi-socks` use following command:

```sh
./target/release/no-dpi-socks
```

SOCKS5 server will run on `localhost:1080`, it can be used in any browser. To
avoid transfer all data through proxy use `proxy.pac` file to configure
browser to transfer only YouTube related traffic through this proxy.

### Command Line Parameters

- `-a, --address <address>`: Bind to address (default: localhost)
- `-p, --port <port>`: Bind to port (default: 1080)

## License
[license]: #license

Source code is primarily distributed under the terms of the MIT license.
See LICENSE for details.
