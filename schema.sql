CREATE TABLE IF NOT EXISTS supply  (
    contract FixedString(40),
    supply       UInt256,
    block_num    UInt32(),
    timestamp    DateTime64(3, 'UTC'),
    version      UInt32()
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (contract,supply);

-- Indexes for block_number and chain --
ALTER TABLE supply ADD INDEX supply_block_number_index block_num TYPE minmax;

-- MV for contract --
CREATE MATERIALIZED VIEW mv_supply_contract
ENGINE = MergeTree()
ORDER BY (contract)
POPULATE
AS SELECT * FROM supply;
