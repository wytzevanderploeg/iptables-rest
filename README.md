# Iptables rest interface

This project was created as a proof-of-concept.

I wanted to create a small restful service on top of iptables.
Using Rust allows for a small footprint and fast startup times making it
possible to run on a router.
I have tested this on my own router with an ARMv7 processor.

IMPORTANT: The project currently does not provide any form of authentication/authorization.

Consider this project to be far from production ready.

## Assumptions
Tables are assumed to be in lowercase. (filter|mangle|nat)
Chains are assumed to be in uppercase. (INPUT|FORWARD|OUTPUT)

## Building for a specific architecture

These are the steps I followed on my router to determine the required architecture to
build for.

### Determine architecture 
`cat /proc/cpuinfo`

```shell
model name	: ARMv7 Processor rev 0 (v7l)
processor	: 0
BogoMIPS	: 1987.37
Features	: half fastmult edsp tls 
CPU implementer	: 0x41
CPU architecture: 7
CPU variant	: 0x3
CPU part	: 0xc09
CPU revision	: 0

model name	: ARMv7 Processor rev 0 (v7l)
processor	: 1
BogoMIPS	: 1993.93
Features	: half fastmult edsp tls 
CPU implementer	: 0x41
CPU architecture: 7
CPU variant	: 0x3
CPU part	: 0xc09
CPU revision	: 0

Hardware	: Northstar Prototype
Revision	: 0000
Serial		: 0000000000000000
```

### Determine hard float
`ls -l`

```shell
rwxrwxrwx    1 root     root            21 Nov  3  2020 functions.sh -> /opt/lib/functions.sh
lrwxrwxrwx    1 root     root             7 Nov  3  2020 ld-musl-arm.so.1 -> libc.so
-rwxr-xr-x    1 root     root         12343 Nov  3  2020 libatomic.so.1
-rwxr-xr-x    1 root     root        467765 Nov  3  2020 libc.so
-rw-r--r--    1 root     root         45431 Nov  3  2020 libgcc_s.so.1
-rw-r--r--    1 root     root        495848 Nov  3  2020 libgcc_s_pic.a
-rwxr-xr-x    1 root     root         25659 Nov  3  2020 libnvram.so
drwxr-xr-x    3 root     root            30 Nov  3  2020 modules
```

### Installing target
see: [Platform support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)

`rustup target add armv7-unknown-linux-musleabi`

### Add cargo config
[Rust forum](https://users.rust-lang.org/t/how-to-install-armv7-unknown-linux-musleabihf/82395)

`.cargo/config.toml`

```toml
[target.armv7-unknown-linux-musleabi]
linker = "rust-lld"
```

### Building
cargo build --release --target armv7-unknown-linux-musleabi

### Router
Some useful commands that can be used on a router flashed with dd-wrt:

Wireless clients: `wl assoclist`
Connected clients: `arp -a`
Full port scan: `nmap -p- <ip>` or `nmap -Pn <ip>`

There is a chain for lan2wan traffic. Add drop rules here.