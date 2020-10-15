See it in action:

```bash
# Builds the ENCODE_TABLE.
$ cargo run --exeample parse
# Builds the DECODE_TABLE.
$ cargo run --exeample flatten
# Run all tests.
$ cargo test --all-features
```

Publish this crate:

```bash
$ cd <project>
$ cargo publish --dry-run
$ cargo publish
```
