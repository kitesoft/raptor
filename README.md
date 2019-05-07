# Raptor
High-speed trading library written in rust

## Usage
Put this in your `Cargo.toml`:
```
raptor = { path = "/path/to/raptor" }
```

Then put this in your crate root:
```rust
extern crate raptor;
```

## Example
```
$ cp examples/.default.yml examples/config.yml
$ vim examples/config.yml
$ cargo run --example main
```
