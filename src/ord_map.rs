//! Ordered map like container.

use crate::OrdVec;

/// Indexed element fo use in [OrdMap].
#[derive(Clone, Debug, Default)]
pub struct Indexed<I, E>(I, E)
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display;

impl<I, E> Indexed<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    /// Create new indexed element.
    pub fn new(index: I, element: E) -> Self {
        Self(index, element)
    }
    /// Return index of element.
    pub fn index(&self) -> &I {
        &self.0
    }
    /// Return element.
    pub fn element(&self) -> &E {
        &self.1
    }
    /// Return mutable element.
    pub fn element_mut(&mut self) -> &mut E {
        &mut self.1
    }
}

impl<I, E> std::fmt::Display for Indexed<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<I, E> PartialEq for Indexed<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, E> PartialOrd for Indexed<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// Map like container where can be iterated in order.
///
/// # Example
///
/// ```
/// use ord_collections::{OrdMap,Indexed,Error};
///
/// let mut index: OrdMap<char, u64> = OrdMap::default();
///
/// // insert `A`, `B`, `C` in wrong order
/// assert!(index.insert(Indexed::new('C', 0)).is_ok());
/// assert!(index.insert(Indexed::new('A', 0)).is_ok());
/// assert!(index.insert(Indexed::new('B', 0)).is_ok());
///
/// // check that we cannot insert `A` again
/// assert!(matches!(
///     index.insert(Indexed::new('A', 0)),
///     Err(Error::Duplicate(_))
/// ));
///
/// // check that iteration is in expected order
/// let mut sorted = String::new();
/// for i in index.iter() {
///     sorted += &i.index().to_string()
/// }
/// assert_eq!(sorted, "ABC");
///
/// assert!(index.insert(Indexed::new('A', 1)).is_err());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OrdMap<I, E>(OrdVec<Indexed<I, E>>)
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display;

impl<I, E> std::ops::Deref for OrdMap<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    type Target = OrdVec<Indexed<I, E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I, E> std::ops::DerefMut for OrdMap<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
