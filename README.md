# Tally Votes - Rust Code Test

This is an exercise to practice Serde.

# Build

```shell
# Clone the repository, then:
cd tally_votes_exercise
cargo build --release
```

# Run

The program requires two files as arguments: contest JSON and votes JSON.
There are several sample files in the test folder.
To run with the provided sample after building the release:

```shell
cd tally_votes_exercise
target/release/tally_votes_exercise tests/contest.json tests/votes.json
```

Or using cargo:

```shell
cargo run -- tests/contest.json  tests/votes.json
```
