# Fediboard (poc)

This is a proof of concept for a simple image board. The aim is to connect it to the Fediverse using the *ActivityPub* protocol. Local state will be stored as documents in a *PostgreSQL* database.

The RESTful API is built using [axum](https://github.com/tokio-rs/axum) and rust.

# Development

After setting up the database and environment, the app can be run using `cargo run` .

## Database

Fediboard expects a fully migrated database to be present for application runs and compilation.

Install *sqlx cli* using `cargo install sqlx-cli` and obtain a *Postgres* image. The following example uses *podman* to run the container:

```bash
podman run --replace  -d --name fedi-db \
    -e POSTGRES_PASSWORD=mysecretpassword \
    -e POSTGRES_DB=fediboard \
    -p 5432:5432 \
    docker.io/postgres:17
```

```bash
cargo sqlx migrate run # runs everything in ./migrations
```

To compile in offline mode, *sqlx* query metadata needs to be generated and checked into git. Add a pre-commit hook to generate *sqlx* query metadata automatically:

```bash
cargo sqlx prepare > /dev/null 2>&1; git add .sqlx > /dev/null
```

The `supabase.postgrestools` language server is used for editing *Postgres* files. Point your plugin at [postgrestools.jsonc](./postgrestools.jsonc).

## Environment

Fediboard expects the following environment variables to be set. Usage of `.env` files is supported using [dotenvy](https://github.com/allan2/dotenvy).

| NAME           | EXAMPLE                                                     |
| -------------- | ----------------------------------------------------------- |
| DATABASE_URL   | psql://postgres:mysecretpassword@localhost:5432/fediboard   |

## License

See [LICENSE](LICENSE).
