function FindProxyForURL(url, host) {
  if (
    dnsDomainIs(host, ".youtube.com") ||
    dnsDomainIs(host, ".ytimg.com") ||
    dnsDomainIs(host, ".ggpht.com") ||
    dnsDomainIs(host, ".googlevideo.com")
  ) {
    return "SOCKS5 localhost:1080; DIRECT";
  } else {
    return "DIRECT";
  }
}
