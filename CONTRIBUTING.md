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
# Build ENCODE_TABLE
$ cargo run --example parse
# Build DECODE_TABLE
$ cargo run --example flatten
# Generate documentation (./target/doc/httlib_huffman)
$ cargo doc --all-features
```
