use anyhow::Result;
use std::convert::{Into, TryFrom};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ops::RangeInclusive;

/// Structure, which represent subnet as an address and subnet mask
#[derive(Debug)]
pub struct CIDRNotation {
    pub addr: IpAddr,
    pub net_mask: IpAddr,
    pub host_range: RangeInclusive<IpAddr>,
}

impl CIDRNotation {
    pub fn new(addr: IpAddr, net_mask: IpAddr) -> Result<Self> {
        Ok(Self {
            addr,
            net_mask,
            host_range: Self::get_host_range(addr, net_mask)?,
        })
    }

    fn get_host_range(address: IpAddr, net_mask: IpAddr) -> Result<RangeInclusive<IpAddr>> {
        let host_range: RangeInclusive<IpAddr>;
        match (address, net_mask) {
            (IpAddr::V4(addr), IpAddr::V4(mask)) => {
                let bit_length = u32::from(mask).count_ones();
                let host_bits = 32 - bit_length;
                let start = (u32::from(addr) >> host_bits) << host_bits;
                let end = start | ((1 << host_bits) - 1);
                host_range = IpAddr::V4(start.into())..=IpAddr::V4(end.into());
                Ok(host_range)
            }
            (IpAddr::V6(addr), IpAddr::V6(mask)) => {
                let bit_length = u128::from(mask).count_ones();
                let host_bits = 128 - bit_length;
                let start = (u128::from(addr) >> host_bits) << host_bits;
                let end = start | ((1 << host_bits) - 1);
                host_range = IpAddr::V6(start.into())..=IpAddr::V6(end.into());
                Ok(host_range)
            }
            _ => Err(anyhow::anyhow!("Address and mask has different types")),
        }
    }
}

impl TryFrom<String> for CIDRNotation {
    type Error = anyhow::Error;

    fn try_from(val: String) -> anyhow::Result<Self> {
        match val.rfind('/') {
            Some(pos) => {
                let input_addr = val[..pos].parse()?;
                let bit_length: u8 = val[pos + 1..].parse()?;
                let network_addr: IpAddr;
                let net_mask: IpAddr;

                match input_addr {
                    IpAddr::V4(addr) => {
                        if bit_length < 1 || bit_length > 32 {
                            return Err(anyhow::anyhow!("Invalid number of bits in mask"));
                        }
                        let net_mask_addr = ((!0u32) << (32 - bit_length)).into();
                        net_mask = IpAddr::V4(net_mask_addr);
                        // Determine network address
                        network_addr =
                            IpAddr::V4(Ipv4Addr::from(u32::from(addr) & u32::from(net_mask_addr)));
                    }
                    IpAddr::V6(addr) => {
                        if bit_length < 1 || bit_length > 128 {
                            return Err(anyhow::anyhow!("Invalid number of bits in mask"));
                        }
                        let net_mask_addr = ((!0u128) << (128 - bit_length)).into();
                        net_mask = IpAddr::V6(net_mask_addr);
                        // Determine network address
                        network_addr = IpAddr::V6(Ipv6Addr::from(
                            u128::from(addr) & u128::from(net_mask_addr),
                        ));
                    }
                }
                Ok(Self::new(network_addr, net_mask).unwrap())
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
        let cidr = CIDRNotation::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            IpAddr::V4(Ipv4Addr::new(255, 255, 0, 0)),
        )
        .unwrap();
        assert_eq!(
            cidr.host_range,
            IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0))
                ..=IpAddr::V4(Ipv4Addr::new(192, 168, 255, 255))
        );
        let notation: String = cidr.into();
        assert_eq!(notation, "192.168.0.1/16");
    }

    #[test]
    fn parse_cidr_notation() {
        let input = String::from("192.168.0.1/24");
        let cidr: CIDRNotation = input.try_into().unwrap();
        assert_eq!(cidr.addr, IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0)));
        assert_eq!(cidr.net_mask, IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)));
        assert_eq!(
            cidr.host_range,
            IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0))..=IpAddr::V4(Ipv4Addr::new(192, 168, 0, 255))
        );

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
