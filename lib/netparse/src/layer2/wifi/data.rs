use crate::{
    core::parse,
    layer2::arp,
    layer2::datalink::*,
    layer3::ip::{ipv4, ipv6},
};

use custom_debug_derive::*;
use nom::{bytes::complete::take, combinator::map, error::context, number::complete::be_u8};
use serde::{Deserialize, Serialize};

#[derive(CustomDebug, Serialize, Deserialize)]
pub struct DataFrameBody {
    pub llc: Option<LLCHeader>,
    pub snap: Option<SNAPHeader>,
    pub payload: Option<Payload>,
}

impl DataFrameBody {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("802.11 Data Frame: Data frame body", |i| {
            let (i, llc) = LLCHeader::parse(i)?;
            let (i, snap) = SNAPHeader::parse(i)?;
            let (i, payload) = match snap.ether_type {
                Some(EtherType::IPv4) => map(ipv4::Packet::parse, Payload::IPv4)(i)?,
                Some(EtherType::IPv6) => map(ipv6::Packet::parse, Payload::IPv6)(i)?,
                Some(EtherType::ARP) => map(arp::Packet::parse, Payload::ARP)(i)?,
                _ => (i, Payload::Unknown),
            };

            let res = Self {
                llc: Some(llc),
                snap: Some(snap),
                payload: Some(payload),
            };

            Ok((i, res))
        })(i)
    }
}

#[derive(CustomDebug, Serialize, Deserialize)]
pub struct LLCHeader {
    #[debug(format = "0x{:02X}")]
    pub dsap: u8,
    #[debug(format = "0x{:02X}")]
    pub ssap: u8,
    #[debug(format = "0x{:02X}")]
    pub ctrl: u8,
}

impl LLCHeader {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("802.11 LLC Header", |i| {
            let (i, dsap) = be_u8(i)?;
            let (i, ssap) = be_u8(i)?;
            let (i, ctrl) = be_u8(i)?;

            let res = Self { dsap, ssap, ctrl };

            Ok((i, res))
        })(i)
    }
}

#[derive(CustomDebug, Serialize, Deserialize)]
pub struct SNAPHeader {
    pub ether_type: Option<EtherType>,
}

impl SNAPHeader {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("802.11 SNAP Header", |i| {
            let (i, _) = take(3_usize)(i)?;
            let (i, ether_type) = EtherType::parse(i)?;

            let res = Self { ether_type };
            Ok((i, res))
        })(i)
    }
}
