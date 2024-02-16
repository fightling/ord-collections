//! Ordered map like container.

use crate::*;

#[test]
pub fn ord_map() {
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

    // check that iteration is in expected order
    let mut sorted = String::new();
    for i in index.iter() {
        sorted += &i.index().to_string()
    }
    assert_eq!(sorted, "ABC");

    assert!(index.insert(Indexed::new('A', 1)).is_err());
}
