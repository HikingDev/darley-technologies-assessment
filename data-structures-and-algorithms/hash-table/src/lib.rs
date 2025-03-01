/*!
Minimal Hash Table Crate
========================

This crate defines:
  - A `HashTable` trait (in `traits.rs`) for a uniform interface.
  - A `LinkedOpenAddressing` implementation (in `linked_open_addressing.rs`)
    that uses open addressing *plus* a doubly linked list, enabling O(1) for
    get_first() and get_last().

Reasoning :
  - The `HashTable` trait makes it easy to swap in different collision strategies.
  - `LinkedOpenAddressing` meets the fixed-size requirement with O(1) insert, remove, and get,
    while also providing O(1) get_first and get_last by linking entries.

Note: Re-export trait and struct here, so users can simply `use hash_table::HashTable` or
`use hash_table::LinkedHashTable`.
*/

mod linked_open_addressing;
mod traits;

// Re-export the HashTable trait so consumers can do `use hash_table::HashTable;`.
pub use traits::HashTable;

// Re-export our linked open addressing table with doubly linked list tracking.
pub use linked_open_addressing::LinkedOpenAddressing as LinkedHashTable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage_example() {
        let mut table = LinkedHashTable::new(5);
        table.insert("PI", 314);
        assert_eq!(table.get(&"PI"), Some(&314));
    }
}
