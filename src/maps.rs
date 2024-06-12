use crate::abi;
use crate::pb::erc20::types::v1::ValidBalanceChanges;
use crate::pb::erc20::supply::types::v1::{TotalSupplies, TotalSupply};
use substreams::{errors::Error, Hex, scalar::BigInt};
use substreams::pb::sf::substreams::index::v1::Keys;


#[substreams::handlers::map]
fn index_supply(s: TotalSupplies) -> Result<Keys, Error> {
    Ok(match s.items.is_empty() {
        true => Keys::default(),
        false => Keys {
            keys: vec!["supply".to_string()]
        },
    })
}

#[substreams::handlers::map]
pub fn map_token_supply(balance_changes: ValidBalanceChanges) -> Result<TotalSupplies, Error> {
    let mut items = Vec::new();
    let contracts = filter_contracts(balance_changes);

    for address in contracts {
        match get_total_supply(address.clone()) {
            Some(supply) => {
                items.push(TotalSupply { address, supply: supply.to_string() })
            },
            None => {},
        }
    }
    Ok(TotalSupplies { items })
}

// ETH Call to retrieve total supply
pub fn get_total_supply(address: String) -> Option<BigInt> {
    let call = abi::erc20::functions::TotalSupply{};
    let hex = Hex::decode(address).unwrap();
    call.call(hex)
}

// filter balance changes contracts into unique list
pub fn filter_contracts(balance_changes: ValidBalanceChanges) -> Vec<String> {
    let mut contracts = Vec::new();
    for balance_change in balance_changes.valid_balance_changes {
        contracts.push(balance_change.contract)
    }
    contracts.sort();
    contracts.dedup();
    contracts
}
