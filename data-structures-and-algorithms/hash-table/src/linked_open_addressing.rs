use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};

use crate::traits::HashTable;

// ---------------------------------------------------------------------------------------------
// COMMENTS / RATIONALE:
// ---------------------------------------------------------------------------------------------
//
// 1) Why a Doubly Linked List with an Open-Addressed Table?
//    - Just an open-addressed table gives O(1) average for insert, remove, get(key),
//      but finding the oldest or newest entry would require scanning the entire table (O(n)).
//    - We want O(1) for get_first() and get_last(). Hence, each entry is also in a doubly
//      linked list, so we can move it to the 'tail' on insertion/updates, and remove it in O(1).
//
// 2) Why Store 'Node' in a Separate Array?
//    - Rust doesn't allow self-referential structs with normal references.
//    - Instead, I keep a Vec<Node<K, V>> and store integer indices for .prev and .next.
//
// 3) Why "LinkedOpenAddressing"?
//    - It's "linked" because we track recency via the doubly linked list,
//      and "open addressing" because we store references to nodes in 'slots'
//      (which are probed linearly).
//
// 4) Potential Improvement:
//    - This example doesn't implement node recycling; once we remove a node,
//      that index is effectively "lost." A real system might keep a free list.
//
// ---------------------------------------------------------------------------------------------

/// Each slot in the open-addressed array can be:
///    - Empty: never used
///    - Tombstone: was occupied, then removed; used to allow continued probing
///    - Occupied(i): currently holds an entry at index `i` in the `nodes` array
#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Tombstone,
    Occupied(usize),
}

/// Represents an actual key-value entry in our hash table, plus links to prev/next
/// for the doubly linked list that tracks insertion order.
#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug)]
pub struct LinkedOpenAddressing<K, V, S = RandomState>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    // Open addressing array:
    slots: Vec<Slot>,

    // Stores the actual data and linked list pointers.
    nodes: Vec<Node<K, V>>,

    // Hasher builder (like RandomState).
    hasher_builder: S,

    // Number of active entries (not counting tombstones).
    len: usize,

    // Fixed maximum number of entries allowed.
    capacity: usize,

    // Head/tail for our doubly linked list of "active" entries.
    head: Option<usize>,
    tail: Option<usize>,

    // Next index to assign in `nodes`. For simplicity, not reusing freed slots here.
    next_node_index: usize,
}

impl<K, V> LinkedOpenAddressing<K, V>
where
    K: Eq + Hash,
{
    /// Creates a table with a given capacity using default hashing (RandomState).
    /// Panics if capacity == 0.
    pub fn new(capacity: usize) -> Self {
        Self::with_hasher(capacity, RandomState::default())
    }
}

impl<K, V, S> LinkedOpenAddressing<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    /// Creates a table with given capacity and a custom hasher.
    /// Panics if capacity == 0.
    pub fn with_hasher(capacity: usize, hasher_builder: S) -> Self {
        assert!(capacity > 0, "Cannot create a 0-capacity hash table.");

        Self {
            slots: vec![Slot::Empty; capacity],
            nodes: Vec::with_capacity(capacity),
            hasher_builder,
            len: 0,
            capacity,
            head: None,
            tail: None,
            next_node_index: 0,
        }
    }

    /// Returns the current number of (active) entries.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the table has no active entries.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Hashes the key and maps it to a slot index.
    fn index_for(&self, key: &K) -> usize {
        (self.hasher_builder.hash_one(key) % self.capacity as u64) as usize
    }

    /// Finds the slot for `key` or an insertion slot (first tombstone or empty).
    /// Returns `Ok(i)` if the key is found at slot i, or `Err(i)` if not found
    /// but i is a suitable insertion position.
    fn probe(&self, key: &K) -> Result<usize, usize> {
        let mut idx = self.index_for(key);
        let mut first_tombstone = None;
        let start_idx = idx; // Remember start

        loop {
            match &self.slots[idx] {
                Slot::Empty => {
                    // If we see an empty slot, the key isn't in the table.
                    // We'll return any earlier tombstone for insertion, otherwise this empty slot.
                    return Err(first_tombstone.unwrap_or(idx));
                }
                Slot::Tombstone => {
                    // Remember the first tombstone for insertion if the key isn't found later.
                    if first_tombstone.is_none() {
                        first_tombstone = Some(idx);
                    }
                }
                Slot::Occupied(node_idx) => {
                    // We need to actually check if the key matches
                    if &self.nodes[*node_idx].key == key {
                        return Ok(idx);
                    }
                }
            }
            idx = (idx + 1) % self.capacity;

            // If we've gone full circle, the table is full of occupied slots and tombstones
            if idx == start_idx {
                // Either return a tombstone or panic if there are none
                return Err(first_tombstone.expect("Hash table is completely full!"));
            }
        }
    }

    /// Allocates a new node in `nodes` at index `next_node_index`.
    /// Real code might reuse freed slots, but I'm keeping it straightforward for the assignment.
    fn allocate_node(&mut self, key: K, value: V) -> usize {
        if self.next_node_index >= self.capacity {
            panic!("No more space to allocate new nodes!");
        }

        let idx = self.next_node_index;
        self.nodes.push(Node {
            key,
            value,
            prev: None,
            next: None,
        });

        self.next_node_index += 1;
        idx
    }

    /// Unlinks a node from the doubly linked list in O(1).
    fn unlink_node(&mut self, node_idx: usize) {
        let (prev_idx, next_idx) = {
            let node = &self.nodes[node_idx];
            (node.prev, node.next)
        };

        // Fix the previous node's `next` pointer
        if let Some(p) = prev_idx {
            self.nodes[p].next = next_idx;
        } else {
            // node_idx was the head
            self.head = next_idx;
        }

        // Fix the next node's `prev` pointer
        if let Some(n) = next_idx {
            self.nodes[n].prev = prev_idx;
        } else {
            // node_idx was the tail
            self.tail = prev_idx;
        }
    }

    /// Links a newly added or updated node at the tail (newest) of the list.
    fn link_at_tail(&mut self, node_idx: usize) {
        let old_tail = self.tail;
        self.tail = Some(node_idx);

        if let Some(t) = old_tail {
            self.nodes[t].next = Some(node_idx);
            self.nodes[node_idx].prev = Some(t);
        } else {
            // The list was empty, so this node is also the head
            self.head = Some(node_idx);
            self.nodes[node_idx].prev = None;
        }

        self.nodes[node_idx].next = None;
    }
}

/// Implement trait that requires O(1) for insert, remove, get, get_first, get_last.
impl<K, V, S> HashTable<K, V> for LinkedOpenAddressing<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.len == self.capacity {
            // We are at max capacity: assignment says "fixed size" -> panic.
            panic!("Hash table is full, cannot insert new key!");
        }

        match self.probe(&key) {
            Ok(slot_idx) => {
                // Key is already in the table
                if let Slot::Occupied(node_idx) = self.slots[slot_idx] {
                    // Unlink from the list (we'll move it to the 'tail' as newest).
                    self.unlink_node(node_idx);

                    // Update the value, store the old one to return
                    let old_value = std::mem::replace(&mut self.nodes[node_idx].value, value);

                    // Now re-link at tail
                    self.link_at_tail(node_idx);

                    Some(old_value)
                } else {
                    // Shouldn't happen
                    unreachable!("Found key but slot isn't occupied?")
                }
            }
            Err(slot_idx) => {
                // Key not found: we can insert at slot_idx
                self.len += 1;

                // Create a new Node in our `nodes`
                let node_idx = self.allocate_node(key, value);

                // Link it at the tail of the list
                self.link_at_tail(node_idx);

                // Occupy this slot with a pointer to that node index
                self.slots[slot_idx] = Slot::Occupied(node_idx);

                None
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        match self.probe(key) {
            Ok(slot_idx) => {
                // Key found at slot_idx
                if let Slot::Occupied(node_idx) = self.slots[slot_idx] {
                    // Unlink from the doubly linked list
                    self.unlink_node(node_idx);

                    // Mark slot as tombstone
                    self.slots[slot_idx] = Slot::Tombstone;

                    self.len -= 1;

                    // Take ownership of the value to return
                    let value = std::mem::replace(&mut self.nodes[node_idx].value, unsafe {
                        std::mem::zeroed()
                    });

                    // This isn't ideal but works for this example - in production code we might
                    // have a free list to reuse these nodes
                    Some(value)
                } else {
                    None
                }
            }
            Err(_) => None, // Key not found
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        match self.probe(key) {
            Ok(slot_idx) => {
                if let Slot::Occupied(node_idx) = &self.slots[slot_idx] {
                    Some(&self.nodes[*node_idx].value)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn get_first(&self) -> Option<(&K, &V)> {
        // The "oldest" node is at self.head
        self.head.map(|head_idx| {
            let node = &self.nodes[head_idx];
            (&node.key, &node.value)
        })
    }

    fn get_last(&self) -> Option<(&K, &V)> {
        // The "newest" node is at self.tail
        self.tail.map(|tail_idx| {
            let node = &self.nodes[tail_idx];
            (&node.key, &node.value)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut table = LinkedOpenAddressing::new(5);

        // Insert a key-value pair
        assert_eq!(table.insert("TravelersGuide", 42), None);

        // Now retrieve it
        assert_eq!(table.get(&"TravelersGuide"), Some(&42));

        // Confirm length is correct
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn test_update_existing_key() {
        let mut table = LinkedOpenAddressing::new(5);

        table.insert("Injective", 55);
        assert_eq!(table.insert("Injective", 120), Some(55));
        assert_eq!(table.get(&"Injective"), Some(&120));
    }

    #[test]
    fn test_remove() {
        let mut table = LinkedOpenAddressing::new(5);

        table.insert("Bitcoin", 125000);
        table.insert("Ethereum", 12728);

        assert_eq!(table.remove(&"Bitcoin"), Some(125000));
        assert_eq!(table.get(&"Bitcoin"), None);
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn test_ordering() {
        let mut table = LinkedOpenAddressing::new(5);

        table.insert("Celestia", 25);
        table.insert("Casper", 2);
        table.insert("Akash", 15);

        assert_eq!(table.get_first(), Some((&"Celestia", &25)));
        assert_eq!(table.get_last(), Some((&"Akash", &15)));

        // Update Celestia key
        table.insert("Celestia", 35);

        // Now "Celestia" should be the most recent
        assert_eq!(table.get_first(), Some((&"Casper", &2)));
        assert_eq!(table.get_last(), Some((&"Celestia", &35)));
    }

    #[test]
    fn test_collisions() {
        use std::hash::Hasher;

        // Use a custom hasher that always returns the same hash
        struct AlwaysCollideHasher;
        impl Hasher for AlwaysCollideHasher {
            fn finish(&self) -> u64 {
                0
            }
            fn write(&mut self, _: &[u8]) {}
        }

        struct AlwaysCollideState;
        impl BuildHasher for AlwaysCollideState {
            type Hasher = AlwaysCollideHasher;
            fn build_hasher(&self) -> AlwaysCollideHasher {
                AlwaysCollideHasher
            }
        }

        let mut table = LinkedOpenAddressing::with_hasher(5, AlwaysCollideState);

        // All these keys will hash to the same slot (0)
        table.insert("Ripple", 1);
        table.insert("Stellar", 2);
        table.insert("Hedera", 3);

        assert_eq!(table.get(&"Ripple"), Some(&1));
        assert_eq!(table.get(&"Stellar"), Some(&2));
        assert_eq!(table.get(&"Hedera"), Some(&3));
    }
}
