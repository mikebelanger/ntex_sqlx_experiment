# ntex_sqlx_experiment
Just fooling around with ntex and sqlx

## Stack
- ntex
- sqlx
- markup.rs

## To Run

1. Make sure Postgres and latest Rust are installed. (I'm using rustc 1.73.0 (cc66ad468 2023-10-03)). Probably any newish Postgres will work.
2. Install [sqlx_cli](https://crates.io/crates/sqlx-cli):
```sh
cargo install sqlx-cli
```
3. Set the database url env variable, and run migrations:
```sh
DATABASE_URL=postgres://postgres@localhost/path-to-my-db
sqlx migrate
```
4. Boot up a process in this repo's root directory:
```sh
DATABASE_URL=postgres://postgres@localhost/path-to-my-db cargo run
```
5. Visit `localhost:8080` in your browser

## Notes

- Very WIP. Probably doesn't make sense. I'm just posting because I want help with something, and its easier to point to a repo than recreate on some forum.

- You will have to manually create data in the db. First make a user row, then create some Todos based on that user id.