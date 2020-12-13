# CIDR notation calculator

## Usage
```shell
USAGE:
    cidrcalc [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cidr <cidr>          Subnet specs in CIDR notation
    -m, --mask <mask>          Subnetwork mask
    -n, --network <network>    Network address
```

## Examples
### Get network address and subnet mask from CIDR notation
```shell
$ cidrcalc -c 192.168.0.1/32
Address: 192.168.0.1
Subnet mask: 255.255.255.255
```

### Get CIDR notation from network address and subnet mask
```shell
$ cidrcalc -n 192.168.0.1 -m 255.255.0.0
192.168.0.1/16
```
