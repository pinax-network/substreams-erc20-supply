use crate::abi::{self, erc20};
use crate::pb::erc20::types::v1::{
    ApprovalEvent, Block as Erc20Block, StorageKey, StorageKeys,
    TotalSupplies, TotalSupply, TransferEvent, TransferEvents,EthCallSupplies,EthCallSupply
};
use abi::erc20::{
    events::{Approval, Transfer},
   // functions::Transfer as TransferFun,
//    functions::TransferFrom as TransferFromFun,
};
use substreams::errors::Error;
//use substreams::log;
use substreams::pb::substreams::Clock;
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetProto};
use substreams::Hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;
use substreams::scalar::BigInt;

use substreams_ethereum::rpc::RpcBatch;

#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let (_, transfers) = map_events(&block);
   

    Ok( TransferEvents{
        transfers: transfers,
    }
    )
}

#[substreams::handlers::map]
pub fn map_filter_contract(block: Erc20Block, s: StoreGetBigInt) -> Result<EthCallSupplies, Error> {
    let mut array_transfer = vec![];
    let mut array_address = vec![];
    for transfer in block.transfers {
        if s.get_first(transfer.clone().address).is_some() {
            if !array_address.contains(&transfer.address) {
                array_address.push(transfer.clone().address);
                array_transfer.push(transfer.clone());
            }
        }
    }

     
    let mut responses = Vec::new();
    let mut array_supply = Vec::new();
    let mut batch = RpcBatch::new();
    let mut i = 0;
    for transfer in array_transfer.clone() {
        batch = batch.add(
            erc20::functions::TotalSupply {},
            Hex::decode(transfer.address.clone()).unwrap()
        );

        i = i + 1;

        if i % 50 == 0 || i == array_transfer.clone().len() - 1 {
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
          array_supply.push(EthCallSupply{address:array_transfer[i].address.clone(),supply: supply.to_string()});
          i = i + 1;
      }

    Ok(EthCallSupplies {
        eth_call_supplies: array_supply,
    })
}

#[substreams::handlers::map]
pub fn map_storage_key(
    block: Block,
    store_supply: StoreGetBigInt,
    store_key: StoreGetProto<StorageKey>,
) -> Result<StorageKeys, Error> {
    let mut storage_keys = Vec::new();
    // ETH calls
    for calls in block.calls() {
        // filter only successful calls
        if calls.call.state_reverted {
            continue;
        }

        if store_key
            .get_first(Hex::encode(&calls.call.address))
            .is_none()
        {
            // Storage changes

            for storage_change in &calls.call.storage_changes {

                if store_supply.get_first(Hex::encode(&storage_change.address)).is_some(){
                    if Hex::encode(&storage_change.old_value)
                    == store_supply
                        .get_first(Hex::encode(&storage_change.address))
                        .unwrap()
                        .to_string()    
                {
                    storage_keys.push(StorageKey {
                        address: Hex::encode(&storage_change.address),
                        key: Hex::encode(&storage_change.key),
                        supply: Hex::encode(&storage_change.new_value),
                    })
                }
                }
                
            }
        }
    }

    Ok(StorageKeys {
        storage_keys: storage_keys,
    })
}

#[substreams::handlers::map]
fn map_total_supply(
    clock: Clock,
    block: Block,
    newfound: StorageKeys,
    store_key: StoreGetProto<StorageKey>,
) -> Result<TotalSupplies, Error> {
    let mut array_supply = Vec::new();

    for calls in block.calls() {
        // filter only successful calls
        if calls.call.state_reverted {
            continue;
        }

        for storage_change in &calls.call.storage_changes {
            if store_key
                .get_first(Hex::encode(&storage_change.address))
                .is_some()
            {
                if Hex::encode(&storage_change.key)
                    == store_key
                        .get_first(Hex::encode(&storage_change.address))
                        .unwrap()
                        .key
                {
                    array_supply.push(TotalSupply {
                        address: Hex::encode(&storage_change.address),
                        supply: Hex::encode(&storage_change.new_value),
                        transaction: Hex::encode(&calls.transaction.hash),
                        block_index: clock.number.to_string(),
                    })
                }
            }
        }
    }

    for storage_key in newfound.storage_keys{
        array_supply.push(TotalSupply {
            address: storage_key.address,
            supply: storage_key.supply,
            transaction: Hex::encode(&block.hash),
            block_index: clock.number.to_string(),
        })
    }

    Ok(TotalSupplies {
        total_supplies: array_supply,
    })
}

pub fn map_events(block: &Block) -> (Vec<ApprovalEvent>, Vec<TransferEvent>) {
    let mut approvals = vec![];
    let mut transfers = vec![];

    for log in block.logs() {
        // received logs are only from successful transaction, no need to check
        // filter by type
        if let Some(approval) = Approval::match_and_decode(log.log) {
            approvals.push(decode_approval(approval, log));
            continue;
        }

        if let Some(transfer) = Transfer::match_and_decode(log.log) {
            transfers.push(decode_transfer(transfer, log));
            continue;
        }

        // no data
    }

    (approvals, transfers)
}

fn decode_transfer(event: Transfer, log: LogView) -> TransferEvent {
    TransferEvent {
        // contract address
        address: Hex::encode(log.address()),

        // event payload
        from: Hex::encode(event.from),
        to: Hex::encode(event.to),
        value: event.value.to_string(),

        // trace information
        transaction: Hex::encode(&log.receipt.transaction.hash),
        block_index: log.log.block_index.into(),
    }
}

fn decode_approval(event: Approval, log: LogView) -> ApprovalEvent {
    ApprovalEvent {
        // contract address
        address: Hex::encode(log.address()),

        // event payload
        owner: Hex::encode(event.owner),
        spender: Hex::encode(event.spender),
        value: event.value.to_string(),

        // trace information
        transaction: Hex::encode(&log.receipt.transaction.hash),
        block_index: log.log.block_index.into(),
    }
}

/*pub fn map_balance_of(block: Block) -> Vec<BalanceOfStorageChange> {
    let mut storage_changes = vec![];

    // ETH calls
    for calls in block.calls() {
        // filter only successful calls
        if calls.call.state_reverted {
            continue;
        }

        // filter by calls containing 36 bytes of raw data
        let input = &calls.call.input;
        if input.len() < 36 {
            continue;
        } // skip if not 36 bytes

        // filter by method selector
        if !TransferFun::match_call(calls.call) && !TransferFromFun::match_call(calls.call) {
            continue;
        }

        // Storage changes
        for storage_change in &calls.call.storage_changes {
            storage_changes.push(BalanceOfStorageChange {
                // contract address
                address: Hex::encode(&storage_change.address),
                method: Hex::encode(&input[0..4]),

                // storage changes
                owner: Hex::encode(&calls.call.caller),
                balance: Hex::encode(&storage_change.new_value),

                // trace information
                transaction: Hex::encode(&calls.transaction.hash),
            })
        }
    }

    storage_changes
}*/
