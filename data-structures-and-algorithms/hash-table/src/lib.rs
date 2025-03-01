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
`use hash_table::LinkedOpenAddressing`.
*/

mod traits;
mod linked_open_addressing;

// Re-export the HashTable trait so consumers can do `use hash_table::HashTable;`.
pub use traits::HashTable;

// Re-export our linked open addressing table with doubly linked list tracking.
pub use linked_open_addressing::LinkedOpenAddressing;

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        // Just a quick check to ensure tests are running
        assert_eq!(3.0 + 0.14, 3.14);
    }
}
