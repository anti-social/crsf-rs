use crate::{ExtendedPayloadDump, PacketType, Payload};

/// `DevicePing` payload type
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DevicePing;

impl Payload for DevicePing {
    fn len(&self) -> usize {
        0
    }

    fn typ(&self) -> u8 {
        PacketType::DevicePing as u8
    }

    fn encode(&self, _data: &mut [u8]) {}
}

impl ExtendedPayloadDump for DevicePing {}
