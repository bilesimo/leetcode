# LeetCode in Rust

Personal repository for LeetCode solutions, notes, and reusable Rust templates.

## Structure

- `src/bin/`: one binary per problem
- `src/lib.rs`: shared helpers and common data structures
- `templates/`: starter files for new problems
- `notes/`: patterns, tricks, and study notes

## Suggested Workflow

Create one file per problem inside `src/bin/`.

Examples:

- `src/bin/two_sum.rs`
- `src/bin/binary_tree_level_order_traversal.rs`

Run a single solution with:

```bash
cargo run --bin two_sum
```

Run tests with:

```bash
cargo test
```
