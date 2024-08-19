//! Ordered map like container.

use crate::{Error, Result};

/// Vec of items sorted by key.
///
/// # Example
///
/// ```
/// use ord_collections:: {Error,OrdVec};
///
/// let mut index: OrdVec<u64> = OrdVec::default();
///
/// // insert `1`, `2`, `3` in wrong order
/// assert!(index.insert(1).is_ok());
/// assert!(index.insert(3).is_ok());
/// assert!(index.insert(2).is_ok());
///
/// // check that we cannot push `1` again
/// assert!(matches!(
///     index.insert(1),
///     Err(Error::Duplicate(_))
/// ));
///
/// // check that iteration is in expected order
/// let mut sorted = String::new();
/// for i in index.iter() {
///     sorted += &i.to_string()
/// }
/// assert_eq!(sorted, "123");
///
/// assert!(index.insert(1).is_err());
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OrdVec<E>(Vec<E>)
where
    E: PartialEq + PartialOrd + std::fmt::Display;

impl<E> OrdVec<E>
where
    E: PartialEq + PartialOrd + std::fmt::Display,
{
    /// Create new empty map.
    /// # Returns
    /// New empty map.
    pub const fn new() -> Self {
        Self(Vec::new())
    }
    /// Insert new element.
    /// # Arguments
    /// - `element`: element to insert
    /// # Returns
    /// - `Ok(())` if element was inserted
    /// - `Err(Error::IndexCollision)` if element with same key already exists
    pub fn insert(&mut self, element: E) -> Result<()> {
        for (n, e) in self.0.iter().enumerate() {
            if e == &element {
                return Err(Error::Duplicate(element.to_string()));
            } else if e > &element {
                self.0.insert(n, element);
                return Ok(());
            }
        }
        self.0.push(element);
        Ok(())
    }
    /// Append other to self.
    /// # Arguments
    /// - `other`: other to append
    pub fn append(&mut self, other: &mut Self) {
        for other in other.0.drain(..) {
            self.insert(other).unwrap();
        }
    }
    /// Return `true` if self contains the given element.
    /// # Arguments
    /// - `what`: comparison value
    /// # Returns
    /// - `true` if self contains the given element
    /// - `false` if self does not contain the given element
    pub fn contains<T>(&self, what: T) -> bool
    where
        E: PartialEq<T>,
    {
        self.0.iter().any(|e| *e == what)
    }
    /// Return iterator over items.
    pub fn iter(&self) -> std::slice::Iter<'_, E> {
        self.0.iter()
    }
    /// Return iterator over mutable items.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, E> {
        self.0.iter_mut()
    }
    /// Return length of map.
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Return `true` if map is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Get first element if any.
    pub fn first(&self) -> Option<&E> {
        self.0.first()
    }
    /// Get last element if any.
    pub fn last(&self) -> Option<&E> {
        self.0.last()
    }
    /// Get value by element.
    pub fn get<T>(&self, what: T) -> Option<&E>
    where
        E: PartialEq<T>,
    {
        self.0.iter().find(|e| *e == &what)
    }
    /// Get mutable value by element.
    /// # Arguments
    /// - `element`: element to search for
    /// # Returns
    /// - `Some(value)`: if element was found
    /// - `None`: if element was not found
    pub fn get_mut<T>(&mut self, what: T) -> Option<&mut E>
    where
        E: PartialEq<T>,
    {
        self.0.iter_mut().find(|e| *e == &what)
    }
    /// Join elements with given separator.
    /// # Arguments
    /// - `separator`: separator to join elements with
    /// # Returns
    /// Joined elements in a string.
    pub fn join(&self, separator: &str) -> String {
        self.0
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(separator)
    }
    /// Extend self with other.
    /// # Arguments
    /// - `other`: other to extend with
    /// # Returns
    /// - `Ok(())` if successful
    pub fn extend<I: IntoIterator<Item = E>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<E> TryFrom<Vec<E>> for OrdVec<E>
where
    E: PartialEq + PartialOrd + std::fmt::Display,
{
    type Error = Error;

    fn try_from(value: Vec<E>) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut result = Self::new();
        for i in value {
            result.insert(i)?;
        }
        Ok(result)
    }
}

impl std::fmt::Display for OrdVec<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.join(","))
    }
}
