# Integration tests

This folder is for integration tests (files in `tests/` are compiled as separate crates).

Guidance:

- Create tests that exercise your public API from the outside (e.g., `use your_crate_name::something;`).
- Example (replace `your_crate_name` with your crate name):

```rust
// tests/integration_example.rs
// use your_crate_name::add;
//
// #[test]
// fn add_works() {
//     assert_eq!(add(2, 3), 5);
// }
```

Notes:
- Integration tests compile as separate crates, so reference your crate by its package name.
- Keep integration tests focused, fast, and deterministic.
