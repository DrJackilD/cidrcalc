/// CLI tool to calculate CIDR notation for subnetwork from IP and Subnet mask
use cidrcalc::cidr::CIDRNotation;
use clap::{AppSettings, Clap};
use std::convert::{TryFrom, TryInto};
use std::net::IpAddr;

/// cidrcalc CLI takes CIDR notation and return network and subnet mask for it
#[derive(Clap)]
#[clap(
    version = "2.0.0",
    author = "Yevhen Dubovskoy <edubovskoy@gmail.com>",
    setting = AppSettings::ArgRequiredElseHelp
)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Parse CIDR notation and return address and mask
    #[clap(name = "parse")]
    Parse(ParseCommand),
    /// Combine address and mask and create CIDR notation
    #[clap(name = "compose")]
    Compose(ComposeCommand),
}

#[derive(Clap)]
struct ParseCommand {
    cidr: String,
}

impl ParseCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let subnet: CIDRNotation = CIDRNotation::try_from(self.cidr.clone())?;
        println!("Address: {}", subnet.addr);
        println!("Subnet mask: {}", subnet.net_mask);
        Ok(())
    }
}

#[derive(Clap)]
struct ComposeCommand {
    /// Network address
    address: IpAddr,
    /// Subnetwork mask
    mask: IpAddr,
}

impl ComposeCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let cidr = CIDRNotation {
            addr: self.address,
            net_mask: self.mask,
        };
        let sn: String = cidr.try_into()?;
        println!("{}", sn);
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.subcommand {
        SubCommand::Parse(cmd) => cmd.run()?,
        SubCommand::Compose(cmd) => cmd.run()?,
    }
    Ok(())
}
