//! Ordered map like container.

use crate::{OrdVec, Result};

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
#[derive(Debug)]
pub struct OrdMap<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    vec: OrdVec<Indexed<I, E>>,
}

impl<I, E> Default for OrdMap<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    fn default() -> Self {
        Self {
            vec: OrdVec::default(),
        }
    }
}

impl<I, E> OrdMap<I, E>
where
    I: PartialEq + PartialOrd + std::fmt::Display,
    E: std::fmt::Display,
{
    /// insert element into map with given index
    /// # Arguments
    /// - `index`: index of element
    /// - `element`: element to insert
    /// # Returns
    /// - `Ok(())` if element was inserted
    /// - `Err(Error::Duplicate)` if index already exists
    pub fn insert(&mut self, indexed: Indexed<I, E>) -> Result<()> {
        self.vec.insert(indexed)
    }
    ///
    pub fn get_mut(&mut self, index: &I) -> Option<&mut E> {
        self.vec
            .iter_mut()
            .find(|i| &i.0 == index)
            .map(|i| &mut i.1)
    }
    /// Returns iterator over elements in map.
    pub fn iter(&self) -> impl Iterator<Item = &Indexed<I, E>> {
        self.vec.iter()
    }
    /// Returns iterator over elements in map.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&I, &mut E)> {
        self.vec.iter_mut().map(|i| (&i.0, &mut i.1))
    }
    /// Return length of map.
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    /// Return `true` if map is empty.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    /// Return `true` if map contains element with given index.
    pub fn contains(&self, index: I) -> bool {
        self.vec.iter().any(|i| i.0 == index)
    }
}
