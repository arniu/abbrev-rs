# abbrev

[![Build Status](https://travis-ci.org/arniu/abbrev-rs.svg?branch=master)](https://travis-ci.org/arniu/abbrev-rs)
[![Crates.io](https://img.shields.io/crates/v/abbrev)](https://crates.io/crates/abbrev)

Like Ruby's Abbrev module

## Example

```rust
use abbrev::abbrev;

fn main() {
    let xs = vec!["foo", "fool", "folding", "flop"];
    let map = abbrev(&xs);

    println!("{:#?}", map);
}
```

And it will print:

```json
{
    "fl": "flop",
    "flo": "flop",
    "flop": "flop",
    "fol": "folding",
    "fold": "folding",
    "foldi": "folding",
    "foldin": "folding",
    "folding": "folding",
    "foo": "foo",
    "fool": "fool"
}
```

## License

[MIT](LICENSE)
