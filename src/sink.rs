
use substreams::{errors::Error, pb::substreams::Clock, store::{StoreGet, StoreGetString}};
use std::collections::HashMap;
use crate::pb::erc20::supply::types::v1::TotalSupplies;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};



#[substreams::handlers::map]
pub fn db_out(clock: Clock, supply: TotalSupplies,s: StoreGetString) -> Result<DatabaseChanges, Error> {
    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();
    let mut database_changes: DatabaseChanges = Default::default();

    for event in supply.items {
        let address = &event.address;

        if s.get_at(0, address).unwrap() == event.supply {
            let id  = HashMap::from([("address".to_string(),address.clone()),("block".to_string(), block.clone())]);
        
        database_changes
        .push_change_composite("supply",id, 0, Operation::Create)
        .change("contract", (None, address))
        .change("supply", (None, event.supply))
        .change("block_num", (None,block.clone() ))
        .change("timestamp", (None, timestamp.clone()));
    }
    Ok(database_changes)
}


