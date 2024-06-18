
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;
use std::collections::HashMap;
use crate::pb::erc20::supply::types::v1::TotalSupplies;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, supply: TotalSupplies) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in supply.items {
        let address = &event.address;
        let id  = format!("{}-{}", address.clone(), block.clone());
        tables
            .create_row("TotalSupply", id)
            .set("address", address)
            .set_bigint("supply", &event.supply)
            .set_bigint("block_number", &block)
            .set_bigint("timestamp", &timestamp);
    }
    Ok(tables.to_entity_changes())
}


#[substreams::handlers::map]
pub fn db_out(clock: Clock, supply: TotalSupplies) -> Result<DatabaseChanges, Error> {
    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();
    let mut database_changes: DatabaseChanges = Default::default();

    for event in supply.items {
        let address = &event.address;
        let id  = HashMap::from([("address".to_string(),address.clone()),("supply".to_string(), event.supply.clone())]);
        
        database_changes
        .push_change_composite("supply",id, 0, Operation::Create)
        .change("contract", (None, address))
        .change("supply", (None, event.supply))
        .change("block_num", (None,block.clone() ))
        .change("timestamp", (None, timestamp.clone()))
        .change("version",  (None, 1));
    }
    Ok(database_changes)
}


