
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use crate::pb::erc20::supply::types::v1::TotalSupplies;
use substreams_entity_change::tables::Tables;


#[substreams::handlers::map]
pub fn graph_out(clock: Clock, supply: TotalSupplies) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in supply.total_supplies {
        let id = format!("{}-{}", event.address, block_num);

        tables
            .create_row("TotalSupply", id)
            // contract address
            .set("address", event.address)
            // event payload
            .set("supply", event.supply)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }
    Ok(tables.to_entity_changes())
}
