/// CLI tool to calculate CIDR notation for subnetwork from IP and Subnet mask
use cidrcalc::cidr::CIDRNotation;
use clap::{AppSettings, Clap};
use std::convert::{TryFrom, TryInto};
use std::net::IpAddr;

/// cidrcalc CLI takes CIDR notation and return network and subnet mask for it
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Yevhen Dubovskoy <edubovskoy@gmail.com>",
    setting = AppSettings::ArgRequiredElseHelp
)]
struct Opts {
    /// Subnet specs in CIDR notation
    #[clap(short, long)]
    cidr: Option<String>,
    /// Network address
    #[clap(short, long)]
    network: Option<IpAddr>,
    /// Subnetwork mask
    #[clap(short, long)]
    mask: Option<IpAddr>,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    if let Some(cidr) = opts.cidr {
        let subnet: CIDRNotation = CIDRNotation::try_from(cidr)?;
        println!("Address: {}", subnet.addr);
        println!("Subnet mask: {}", subnet.net_mask)
    } else if let Some(network) = opts.network {
        if let Some(mask) = opts.mask {
            let cidr = CIDRNotation {
                addr: network,
                net_mask: mask,
            };
            let sn: String = cidr.try_into()?;
            println!("{}", sn);
        }
    }
    Ok(())
}
