# Fediboard (poc)

 This is a proof of concept for a simple image board. The aim is to connect it to the Fediverse using the ActivityPub protocol. Local state will be stored as documents in a PostgreSQL database.

 The RESTful API is built using [axum](https://github.com/tokio-rs/axum) and rust.

 ## Development

Run the service with:
```bash
 podman run --replace --name fedi-db -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_DB=fediboard -p 5432:5432 -d docker.io/postgres:17


DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/fediboard cargo run
```

Add a pre-commit hook to generate sqlx query metadata automatically:
```bash
cargo sqlx prepare > /dev/null 2>&1; git add .sqlx > /dev/null
```

 ## License
 See [LICENSE](LICENSE).