CREATE TABLE IF NOT EXISTS supply  (
    contract FixedString(40),
    supply       UInt256,
    block_num    UInt32(),
    timestamp    DateTime64(3, 'UTC'),
)
ENGINE = ReplacingMergeTree()
ORDER BY (contract,block_num);
