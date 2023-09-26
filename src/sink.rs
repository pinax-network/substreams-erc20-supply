
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;
use crate::pb::erc20::supply::types::v1::TotalSupplies;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, supply: TotalSupplies) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in supply.total_supplies {
        let address = &event.address;
        tables
            .create_row("TotalSupply", address)
            .set("address", address)
            .set_bigint("supply", &event.supply)
            .set_bigint("block", &block)
            .set_bigint("timestamp", &timestamp);
    }
    Ok(tables.to_entity_changes())
}
