## Publishing

```bash
$ cd <project>
# Regenerate readme
$ cargo readme > README.md
# Run tests
$ cargo test --all-features
# Try to publich
$ cargo publish --dry-run
# Publish
$ cargo publish
```

## Notes

```bash
# Builds the ENCODE_TABLE.
$ cargo run --example parse
# Builds the DECODE_TABLE.
$ cargo run --example flatten
```
