create table todo (
    todo_id uuid primary key default gen_random_uuid(),
    user_id uuid not null references "user"(user_id),
    title text not null,
    content text,
    deadline timestamptz not null,
    created_at timestamptz not null default now()
);

create index on todo(created_at desc);