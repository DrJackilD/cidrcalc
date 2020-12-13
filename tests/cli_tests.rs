use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

// `cidrcalc` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("cidrcalc").unwrap().assert().failure();
}

// `cidrcalc parse <CIDR>` should return parsed CIDR notation as address and subnet mask
#[test]
fn cli_parse() {
    Command::cargo_bin("cidrcalc")
        .unwrap()
        .args(&["parse", "192.168.0.1/24"])
        .assert()
        .success()
        .stdout(contains("Address: 192.168.0.1"))
        .stdout(contains("Subnet mask: 255.255.255.0"));
}

// `cidrcalc parse <CIDR>` should return error in case of invalid input
#[test]
fn cli_parse_invalid() {
    Command::cargo_bin("cidrcalc")
        .unwrap()
        .args(&["parse", "192.168.0.1"])
        .assert()
        .failure()
        .stderr(contains("Error: Could not parse CIDR notation"));
}

// `cidrcalc compose <ADDRESS> <MASK>` should return CIDR notation for given address and mask
#[test]
fn cli_compose() {
    Command::cargo_bin("cidrcalc")
        .unwrap()
        .args(&["compose", "192.168.0.1", "255.255.128.0"])
        .assert()
        .success()
        .stdout(contains("192.168.0.1/17"));
}

// `cidrcalc compose <ADDRESS> <MASK>` should return an error in case of invalid input
#[test]
fn cli_compose_invalid() {
    Command::cargo_bin("cidrcalc")
        .unwrap()
        .args(&["compose", "192.168ff", "255.255.0.0"])
        .assert()
        .failure()
        .stderr(contains(
            "error: Invalid value for '<address>': invalid IP address syntax",
        ));
}
