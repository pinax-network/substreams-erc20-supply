
use crate::pb::erc20::supply::types::v1::TotalSupplies;
use substreams::store::{StoreSet, StoreSetString};
use substreams::store::StoreNew;

#[substreams::handlers::store]
fn store_supply(supply: TotalSupplies,s: StoreSetString)
{
     for item in supply.items {
          s.set(0, &item.address, &item.supply);
     }
}
