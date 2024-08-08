function FindProxyForURL(url, host) {
  if (dnsDomainIs(host, ".googlevideo.com")) {
    return "SOCKS5 localhost:1080; DIRECT";
  } else {
    return "DIRECT";
  }
}
