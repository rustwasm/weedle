# Weedle - A WebIDL Parser

Parses valid WebIDL definitions & produces a data structure starting from
[`Definitions`](https://docs.rs/weedle/struct.Definitions.html).

### Example

```rust
extern crate weedle;

let (_, parsed) = weedle::parse("
    interface Window {
        readonly attribute Storage sessionStorage;
    };
").unwrap();
println!("{:?}", parsed);
```