# No DPI SOCKS

A simple SOCKS5 proxy designed to bypass YouTube slowdown. The server works by
analyzing packets individually. To circumvent this, the server sends the first
1024 bytes of the stream one byte per packet. This prevents DPI from
identifying the target host name.

## Build

To build `no-dpi-socks` from source code use following command:

```sh
cargo build --release
```

To start `no-dpi-socks` use following command:

```sh
./target/release/no-dpi-socks
```

SOCKS5 server will run on `localhost:1080`, it can be used in any browser. To avoid transfer all data through proxy
use `proxu.pac` file to configure browser to transfer only `*.googlevideo.com` traffic through this proxy.

## License
[license]: #license

Source code is primarily distributed under the terms of the MIT license. See LICENSE for details.
