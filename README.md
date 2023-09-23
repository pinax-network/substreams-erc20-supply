# ERC-20 Token Supply Substreams

> Extends [ERC-20 Balance Changes](https://github.com/streamingfast/substreams-erc20-balance-changes) with Token Supply

## Quickstart

```
$ gh repo clone pinax-network/substreams-erc20-total-supply
$ cd substreams-erc20-total-supply
$ make
$ make gui
```

## Releases

- https://github.com/pinax-network/substreams-erc20-total-supply/releases

## References
- [Ethereum Docs: ERC-20 Token Standard](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)
- [EIPS: ERC-20 Token Standard ](https://eips.ethereum.org/EIPS/eip-20)
- [OpenZeppelin implementation](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/9b3710465583284b8c4c5d2245749246bb2e0094/contracts/token/ERC20/ERC20.sol)
- [ConsenSys implementation](https://github.com/ConsenSys/Tokens/blob/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20/EIP20.sol)

### Mermaid graph

```mermaid
graph TD;
  map_balance_changes[map: map_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_balance_changes;
  map_valid_balance_changes[map: map_valid_balance_changes];
  map_balance_changes --> map_valid_balance_changes;
  map_token_supply[map: map_token_supply];
  map_valid_balance_changes --> map_token_supply;
  balance_changes:map_balance_changes[map: balance_changes:map_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> balance_changes:map_balance_changes;
  balance_changes:map_valid_balance_changes[map: balance_changes:map_valid_balance_changes];
  balance_changes:map_balance_changes --> balance_changes:map_valid_balance_changes;
  balance_changes:map_unknown_balance_changes[map: balance_changes:map_unknown_balance_changes];
  balance_changes:map_balance_changes --> balance_changes:map_unknown_balance_changes;
  balance_changes:db_out[map: balance_changes:db_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> balance_changes:db_out;
  balance_changes:map_balance_changes --> balance_changes:db_out;
  balance_changes:graph_out[map: balance_changes:graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> balance_changes:graph_out;
  balance_changes:map_balance_changes --> balance_changes:graph_out;
  balance_changes:store_valid_balance_changes[store: balance_changes:store_valid_balance_changes];
  balance_changes:map_balance_changes --> balance_changes:store_valid_balance_changes;
  balance_changes:balance_change_stats[map: balance_changes:balance_change_stats];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> balance_changes:balance_change_stats;
  balance_changes:store_valid_balance_changes --> balance_changes:balance_change_stats;
```

### Modules

```yaml
Package name: erc20_supply
Version: v0.1.0
Doc: ERC-20 Token Supply
Modules:
----
Name: map_token_supply
Initial block: 1397553
Kind: map
Output Type: proto:erc20.supply.types.v1.TotalSupplies
Hash: 87ad90dfb4649470424b6259b141339db719eba6
Doc: Extracts ERC20 token total supply
```