create table
    "board" (
        board_id uuid primary key default gen_random_uuid(),
        name text unique not null
    );

insert into
    board (name)
values
    ('a'),
    ('b'),
    ('c');