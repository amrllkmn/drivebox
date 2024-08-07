# Drivebox

It's a Dropbox/Google Drive clone, written in Rust.

## How to run

1. Run `cargo run` at the root of the project folder.
2. It should print out `listening on 127:0.0.1:3000`.
3. Verify that it returns `{ "message": "OK" }`, by either opening `localhost:3000` on your browser or making a cURL request.
