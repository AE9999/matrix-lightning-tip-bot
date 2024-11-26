
use std::collections::HashSet;
use std::sync::{RwLock};

pub struct TransactionIDCache {
    array: Vec<String>,
    array_ptr: usize,
    hash: HashSet<String>,
    lock: RwLock<()>,
}

impl TransactionIDCache {
    // Constructor
    pub fn new(size: usize) -> Self {
        TransactionIDCache {
            array: vec![String::new(); size],
            array_ptr: 0,
            hash: HashSet::new(),
            lock: RwLock::new(()),
        }
    }

    // Check if a transaction ID has already been processed
    pub fn is_processed(&self, txn_id: &str) -> bool {
        let _read_guard = self.lock.read().unwrap();
        self.hash.contains(txn_id)
    }

    // Mark a transaction ID as processed with internal locking
    pub fn mark_processed(&mut self, txn_id: String) {
        let _write_guard = self.lock.write().unwrap(); // Lock for writing

        // Insert the new transaction ID into the hash
        self.hash.insert(txn_id.clone());

        // Clear older entries in the circular buffer and hash if array is full
        if !self.array[self.array_ptr].is_empty() {
            for i in 0..self.array.len() / 8 {
                if let Some(id) = self.array.get(self.array_ptr + i) {
                    self.hash.remove(id);
                    self.array[self.array_ptr + i].clear();
                }
            }
        }

        // Update the circular pointer and insert the new transaction ID
        self.array[self.array_ptr] = txn_id;
        self.array_ptr = (self.array_ptr + 1) % self.array.len();
    }
}