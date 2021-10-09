#[cfg(test)]
mod opus_test;

use crate::{
    error::{Error, Result},
    packetizer::{Depacketizer, Payloader},
};

use bytes::Bytes;

#[derive(Default, Debug, Copy, Clone)]
pub struct OpusPayloader;

impl Payloader for OpusPayloader {
    fn payload(&mut self, mtu: usize, payload: &Bytes) -> Result<Vec<Bytes>> {
        if payload.is_empty() || mtu == 0 {
            return Ok(vec![]);
        }

        Ok(vec![payload.clone()])
    }

    fn clone_to(&self) -> Box<dyn Payloader + Send + Sync> {
        Box::new(*self)
    }
}

/// OpusPacket represents the Opus header that is stored in the payload of an RTP Packet
#[derive(PartialEq, Debug, Default, Clone)]
pub struct OpusPacket {
    pub payload: Bytes,
}

impl Depacketizer for OpusPacket {
    fn depacketize(&mut self, packet: &Bytes) -> Result<()> {
        if packet.is_empty() {
            Err(Error::ErrShortPacket)
        } else {
            self.payload = packet.clone();
            Ok(())
        }
    }

    fn is_partition_head(&self, _payload: &Bytes) -> bool {
        true
    }

    fn is_partition_tail(&self, _marker: bool, _payload: &Bytes) -> bool {
        true
    }
}
