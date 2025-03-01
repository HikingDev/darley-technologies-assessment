use std::hash::Hash;

/// A generic trait for hash-table-like data structures.
/// 
/// # Type Parameters
/// - `K`: The key type (must implement `Eq` and `Hash`).
/// - `V`: The value type.
pub trait HashTable<K, V>
where
    K: Eq + Hash,
{
    /// Inserts or updates a key-value pair in the hash table.
    /// 
    /// Returns `Some(old_value)` if the key existed and its value was replaced,
    /// otherwise returns `None` if the key was newly inserted.
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Removes a key-value pair from the hash table.
    ///
    /// Returns `Some(value)` if the key existed (and is removed),
    /// or `None` if the key wasn’t found.
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Retrieves a reference to the value for the given `key`, if it exists.
    fn get(&self, key: &K) -> Option<&V>;

    /// Returns a reference to the most recent key-value pair
    /// that was either inserted or updated (and still present).
    fn get_last(&self) -> Option<(&K, &V)>;

    /// Returns a reference to the least recent key-value pair
    /// that was inserted or updated (and still present).
    fn get_first(&self) -> Option<(&K, &V)>;
}
