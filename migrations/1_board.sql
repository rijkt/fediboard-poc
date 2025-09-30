create table "board" (
    board_id       uuid primary key default gen_random_uuid(),
    name           text unique not null
);