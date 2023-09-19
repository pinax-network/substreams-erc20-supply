use crate::abi::{self};
use crate::pb::erc20::types::v1::{
    ApprovalEvent, BalanceOfStorageChange, Block as Erc20Block, TransferEvent,TransferEvents,StorageKeys,StorageKey
};
use abi::erc20::{
    events::{Approval, Transfer},
    functions::Transfer as TransferFun,
    functions::TransferFrom as TransferFromFun,
};
use substreams::errors::Error;
use substreams::Hex;
use substreams::store::{StoreGetBigInt, StoreGet, Deltas, DeltaBigInt};
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Erc20Block, Error> {
    let (approvals, transfers) = map_events(&block);
    let storage_changes = map_balance_of(block);

    Ok(Erc20Block {
        approvals,
        transfers,
        storage_changes,
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

pub fn map_filter_contract(block: Erc20Block,s: StoreGetBigInt)-> Result<TransferEvents,Error>
{
    let mut array_transfer = vec![];
    for transfer in block.transfers {

        if s.get_first(transfer.clone().address).is_some() {
            array_transfer.push(transfer.clone());
        }
    }

    Ok(TransferEvents{
        transfers: array_transfer
    })
}


pub fn map_storage_change(deltas: Deltas<DeltaBigInt>)-> Result<StorageKeys,Error>
{
   let mut storage_array = Vec::new();

   for delta in deltas.deltas{
    storage_array.push(StorageKey {
        address: delta.key,
        key: "".to_string(),
        supply: delta.new_value.to_string()
    })
   }
   Ok(StorageKeys { storage_keys: storage_array })

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

pub fn map_balance_of(block: Block) -> Vec<BalanceOfStorageChange> {
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
}
