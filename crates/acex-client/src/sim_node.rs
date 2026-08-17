// region: Imports

use crate::{client::UdsClient, ClientError};
use acex_core::Vec;
use acex_sim::node::SimNode;

// endregion: Imports

// region: SimNode for UdsClient

impl<
        const PENDING: usize,
        const SIM_MAX_FRAME: usize,
        const SIM_MAX_OUTBOX: usize,
        const MAX_EVENTS: usize,
        const PERIODIC_DIDS: usize,
        const MAX_DATA: usize,
    > SimNode<SIM_MAX_FRAME, SIM_MAX_OUTBOX>
    for UdsClient<PENDING, SIM_MAX_FRAME, SIM_MAX_OUTBOX, MAX_EVENTS, PERIODIC_DIDS, MAX_DATA>
{
    type Error = ClientError;

    fn address(&self) -> &acex_sim::io::NodeAddress {
        UdsClient::address(self)
    }

    fn handle(
        &mut self,
        src: &acex_sim::io::NodeAddress,
        data: &[u8],
        now: acex_sim::clock::Instant,
    ) -> Result<(), Self::Error> {
        UdsClient::handle(self, src, data, now)
    }

    fn tick(&mut self, now: acex_sim::clock::Instant) -> Result<(), Self::Error> {
        UdsClient::tick(self, now)
    }

    fn drain_outbox(
        &mut self,
        out: &mut Vec<(acex_sim::io::NodeAddress, Vec<u8, SIM_MAX_FRAME>), SIM_MAX_OUTBOX>,
    ) -> usize {
        UdsClient::drain_outbox(self, out)
    }
}

// endregion: SimNode for UdsClient
