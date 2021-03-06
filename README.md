# CIDR notation calculator

## Installation
```shell
$ cargo build --release
$ ./target/release/cidrcalc <SUBCOMMAND>
```

## Usage
```shell
USAGE:
    cidrcalc <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    compose    Combine address and mask and create CIDR notation
    help       Prints this message or the help of the given subcommand(s)
    parse      Parse CIDR notation and return address and mask
```

## Examples
### Get network address and subnet mask from CIDR notation
```shell
$ cidrcalc parse 192.168.0.1/32
Address: 192.168.0.1
Subnet mask: 255.255.255.255
Hosts range: 192.168.0.1 - 192.168.0.1
```

### Get CIDR notation from network address and subnet mask
```shell
$ cidrcalc compose 192.168.0.1 255.255.0.0
192.168.0.1/16
```
