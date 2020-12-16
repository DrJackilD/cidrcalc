use std::convert::{Into, TryFrom};
use std::net::IpAddr;

/// Structure, which represent subnet as an address and subnet mask
#[derive(Debug)]
pub struct CIDRNotation {
    pub addr: IpAddr,
    pub net_mask: IpAddr,
}

impl TryFrom<String> for CIDRNotation {
    type Error = anyhow::Error;

    fn try_from(val: String) -> anyhow::Result<Self> {
        match val.rfind('/') {
            Some(pos) => {
                let addr = val[..pos].parse()?;
                let bit_length: u8 = val[pos + 1..].parse()?;
                let net_mask: IpAddr;
                match addr {
                    IpAddr::V4(_) => {
                        if bit_length < 1 || bit_length > 32 {
                            return Err(anyhow::anyhow!("Invalid number of bits in mask"));
                        }
                        net_mask = IpAddr::V4(((!0u32) << (32 - bit_length)).into());
                    }
                    IpAddr::V6(_) => {
                        if bit_length < 1 || bit_length > 128 {
                            return Err(anyhow::anyhow!("Invalid number of bits in mask"));
                        }
                        net_mask = IpAddr::V6(((!0u128) << (128 - bit_length)).into());
                    }
                }
                Ok(Self { addr, net_mask })
            }
            None => Err(anyhow::anyhow!("Could not parse CIDR notation")),
        }
    }
}

impl Into<String> for CIDRNotation {
    fn into(self) -> String {
        let bits: u8 = match self.net_mask {
            IpAddr::V4(addr) => u32::from(addr).count_ones() as u8,
            IpAddr::V6(addr) => u128::from(addr).count_ones() as u8,
        };

        let notation = format!("{}/{}", self.addr, bits);
        notation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn cidr_from_struct() {
        let cidr = CIDRNotation {
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            net_mask: IpAddr::V4(Ipv4Addr::new(255, 255, 0, 0)),
        };
        let notation: String = cidr.into();
        assert_eq!(notation, "192.168.0.1/16");
    }

    #[test]
    fn parse_cidr_notation() {
        let input = String::from("192.168.0.1/24");
        let cidr: CIDRNotation = input.try_into().unwrap();
        assert_eq!(cidr.addr, IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(cidr.net_mask, IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)));

        let input = String::from("::1/128");
        let cidr: CIDRNotation = input.try_into().unwrap();
        assert_eq!(cidr.addr, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
        assert_eq!(
            cidr.net_mask,
            IpAddr::V6(Ipv6Addr::new(
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX
            ))
        );
    }
}
