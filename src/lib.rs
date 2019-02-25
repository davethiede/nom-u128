// #![feature(repeat_generic_slice)]

//#[macro_use]
//extern crate nom;

use nom::*;
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};

/// A Communication record
#[derive(PartialEq,Debug)]
pub struct Comm {
    saddr: IpAddr,
    daddr: IpAddr,
}

named_args!(pub parse_comm(e: Endianness)<Comm>,
    do_parse!(
        saddr: u32!(e) >>
        daddr: u128!(e) >>
        (Comm {
            saddr: IpAddr::V4(<Ipv4Addr>::from(saddr)),
            daddr: IpAddr::V6(<Ipv6Addr>::from(daddr)),
        })
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        // let i = b"\x00".repeat(20);
        let i = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let s = Comm {
            saddr: "0.0.0.0".parse().unwrap(),
            daddr: "::0".parse().unwrap(),
        };
        assert_eq!(parse_comm(&i[..],Endianness::Big), Ok((&[][..], s)));
    }
}
