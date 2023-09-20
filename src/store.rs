use crate::abi::{self, erc20};
use crate::pb::erc20::types::v1::TransferEvents;

use substreams::Hex;
use substreams::scalar::BigInt;

use substreams_ethereum::rpc::RpcBatch;
use substreams::store::{StoreSetBigInt, StoreSet};
use substreams::store::StoreNew;
use substreams::log;
pub struct TotalSupply{
    pub address: String,
    pub supply: BigInt
}

#[substreams::handlers::store]
fn store_total_supply(transfers: TransferEvents,s: StoreSetBigInt){
    let mut responses = Vec::new();
    let mut array_supply = Vec::new();
    let mut batch = RpcBatch::new();
    let mut i = 0;
    for transfer in transfers.clone().transfers {
        batch = batch.add(
            erc20::functions::TotalSupply {},
            Hex::decode(transfer.address.clone()).unwrap()
        );

        i = i + 1;

        if i % 50 == 0 || i == transfers.clone().transfers.len() - 1 {
            let batch_response = batch.execute().unwrap().responses;
            responses.extend(batch_response);
            batch = RpcBatch::new();
        }
    }
    
    i = 0;
    for rpc_response in responses.clone(){

        let supply =  match RpcBatch::decode::<_, abi::erc20::functions::TotalSupply>(&rpc_response){
              Some(data) => BigInt::from(data),
              None => BigInt::from(0)
          };
          array_supply.push(TotalSupply{address:transfers.transfers[i].address.clone(),supply: supply});
          i = i + 1;
      }


    for supply in array_supply{
        log::info!("supply {}",supply.address);
        s.set(0, supply.address, &supply.supply)
    }


}