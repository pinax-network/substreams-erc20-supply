
use substreams::{errors::Error, pb::substreams::Clock};
use std::collections::HashMap;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams::store::{Deltas, DeltaString};


#[substreams::handlers::map]
pub fn db_out(clock: Clock,s: Deltas<DeltaString>) -> Result<DatabaseChanges, Error> {
    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();
    let mut database_changes: DatabaseChanges = Default::default();

    for event in s.deltas{
        let address = &event.key;
        
        if event.new_value != event.old_value {
            let id  = HashMap::from([("address".to_string(),address.clone()),("block".to_string(), block.clone())]);
        
        database_changes
        .push_change_composite("supply",id, 0, Operation::Create)
        .change("contract", (None, address))
        .change("supply", (None, event.new_value))
        .change("block_num", (None,block.clone() ))
        .change("timestamp", (None, timestamp.clone()));
    }
}
    Ok(database_changes)
}


