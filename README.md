# Weedle - A WebIDL Parser

Parses valid WebIDL definitions & produces a data structure starting from
[`Definitions`](struct.Definitions.html).

### Example

```
extern crate weedle;

let (_, parsed) = weedle::parse("
    interface Window {
        readonly attribute Storage sessionStorage;
    };
").unwrap();
println!("{:?}", parsed);
```