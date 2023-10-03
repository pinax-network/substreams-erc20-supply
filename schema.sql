create table supply
(
    id          text not null constraint supply_pk primary key,
    address     text,
    supply      text,
    block       bigint,
    timestamp   timestamp
);

create table cursors
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);