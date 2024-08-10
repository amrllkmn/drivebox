# Drivebox

It's a Dropbox/Google Drive clone, written in Rust.

## How to run

1. Run `cargo run` at the root of the project folder.
2. It should print out `listening on 127:0.0.1:3000`.
3. Verify that it returns `{ "message": "OK" }`, by either opening `localhost:3000` on your browser or making a cURL request.

## Running migrations on local

1. First set your `DATABASE_URL`

```sh
DATABASE_URL= postgres://username:password@db_host:port/db_name
```

2. Echo it to verify

```sh
echo $DATABASE_URL # Should return postgres://username:password@db_host:port/db_name
```

3. Then, install `sqlx-cli`

```sh
cargo install sqlx-cli
```

4. Run `sqlx migrate run`. It should apply all pending migrations.
