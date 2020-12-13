use std::convert::{TryFrom, TryInto};
use std::net::IpAddr;

/// Structure, which represent subnet as
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
                        let mut bit_mask: [u8; 4] = [0; 4];
                        let full_bytes = (bit_length / 8) as u8;
                        for i in 0..full_bytes {
                            bit_mask[i as usize] = u8::MAX;
                        }
                        let rem = bit_length - (full_bytes * 8);
                        if rem > 0 {
                            bit_mask[full_bytes as usize] =
                                u8::MAX - (2u8.pow((8 - rem).into()) - 1);
                        }
                        net_mask = IpAddr::from(bit_mask);
                    }
                    IpAddr::V6(_) => {
                        if bit_length < 1 || bit_length > 128 {
                            return Err(anyhow::anyhow!("Invalid number of bits in mask"));
                        }
                        let mut bit_mask: [u16; 8] = [0; 8];
                        let full_hexs: usize = (bit_length / 16) as usize;
                        for i in 0..full_hexs {
                            bit_mask[i] = u16::MAX;
                        }
                        let rem: u16 = (bit_length - (full_hexs as u8 * 16)).into();
                        if rem > 0 {
                            bit_mask[full_hexs] =
                                u16::MAX - (2u32.pow((16 - rem).into()) - 1) as u16
                        };
                        net_mask = IpAddr::from(bit_mask);
                    }
                }
                Ok(Self { addr, net_mask })
            }
            None => Err(anyhow::anyhow!("Could not parse CIDR notation")),
        }
    }
}

impl TryInto<String> for CIDRNotation {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<String> {
        let bits: u8 = match self.net_mask {
            IpAddr::V4(addr) => addr.octets().iter().map(|i| i.count_ones() as u8).sum(),
            IpAddr::V6(addr) => addr.segments().iter().map(|i| i.count_ones() as u8).sum(),
        };

        let notation = format!("{}/{}", self.addr, bits);
        Ok(notation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn cidr_from_struct() {
        let cidr = CIDRNotation {
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            net_mask: IpAddr::V4(Ipv4Addr::new(255, 255, 0, 0)),
        };
        let notation: String = cidr.try_into().unwrap();
        assert_eq!(notation, "192.168.0.1/16");
    }

    #[test]
    fn parse_cidr_notation() {
        let input = String::from("192.168.0.1/24");
        let cidr: CIDRNotation = input.try_into().unwrap();
        assert_eq!(cidr.addr, IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(cidr.net_mask, IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)));
    }
}
