# ord-collections

[![Status](https://github.com/fightling/ord-collections/actions/workflows/rust.yml/badge.svg)](https://github.com/fightling/ord-collections/actions)
[![Crates.io](https://img.shields.io/crates/v/ord-collections.svg)](https://crates.io/crates/ord-collections)
[![Documentation](https://docs.rs/ord-collections/badge.svg)](https://docs.rs/ord-collections/)
[![Dependency status](https://deps.rs/repo/github/fightling/ord-collections/status.svg)](https://deps.rs/repo/github/fightling/ord-collections)

## Examples

### OrdVec

```rs
use ord_collections:: {Error,OrdVec};

let mut index: OrdVec<u64> = OrdVec::default();

// insert `1`, `2`, `3` in wrong order
assert!(index.insert(1).is_ok());
assert!(index.insert(3).is_ok());
assert!(index.insert(2).is_ok());

// check that we cannot push `1` again
assert!(matches!(
    index.insert(1),
    Err(Error::Duplicate(_))
));

// check that iteration is in expected order
let mut sorted = String::new();
for i in index.iter() {
    sorted += &i.to_string()
}
assert_eq!(sorted, "123");

assert!(index.insert(1).is_err());
```

### OrdMap

```rs
use ord_collections::{OrdMap,Indexed,Error};

let mut index: OrdMap<char, u64> = OrdMap::default();

// insert `A`, `B`, `C` in wrong order
assert!(index.insert(Indexed::new('C', 0)).is_ok());
assert!(index.insert(Indexed::new('A', 0)).is_ok());
assert!(index.insert(Indexed::new('B', 0)).is_ok());

// check that we cannot insert `A` again
assert!(matches!(
    index.insert(Indexed::new('A', 0)),
    Err(Error::Duplicate(_))
));

check that iteration is in expected order
let mut sorted = String::new();
for i in index.iter() {
    sorted += &i.index().to_string()
}
assert_eq!(sorted, "ABC");

assert!(index.insert(Indexed::new('A', 1)).is_err());
```
