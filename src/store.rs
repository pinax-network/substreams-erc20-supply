use std::str::FromStr;


use crate::pb::erc20::types::v1::{StorageKeys, EthCallSupplies,StorageKey};

use substreams::scalar::BigInt;

use substreams::store::{StoreSetBigInt, StoreSet, StoreSetProto};
use substreams::store::StoreNew;
use substreams::log;


#[substreams::handlers::store]
fn store_total_supply(array_supply: EthCallSupplies,s: StoreSetBigInt){
   

    for supply in array_supply.eth_call_supplies{
        log::info!("supply {}",supply.address);
        s.set(0, supply.address, &BigInt::from_str(&supply.supply).unwrap())
    }


}

#[substreams::handlers::store]
fn store_key(storage_keys: StorageKeys,s: StoreSetProto<StorageKey>){
    for element in storage_keys.storage_keys{
        s.set(0, &element.address, &element)
    }
}

