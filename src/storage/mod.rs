use std::error::Error;
use std::collections::BTreeMap;
use crate::selection::subscription::Subscription;
use crate::selection::selector::*;
use crate::router::Router;
use tokio::sync::RwLock;
use std::sync::{Arc};
use sys_info;
use tokio::time::delay_for;

const INNER_ALLOC_PERCENTAGE: f64 = 0.8;

pub struct Store {
    pub length: usize,
    pub max_length: usize,
    pub current: u64,
    btree: BTreeMap<u64, ([u8; 4], Vec<u8>)>
}

impl Store {
    pub fn new() -> Arc<RwLock<Store>> {
        let memory = sys_info::mem_info().unwrap();
        let btree_size = ((memory.avail + memory.swap_free) as f64 * INNER_ALLOC_PERCENTAGE) as usize;
        println!("Total memory: {} MB, Available memory: {} MB, Available swap: {} MB, Percentage for Store: {}, Reserved: {} MB", memory.total / 1024, memory.avail / 1024, memory.swap_free / 1024,  INNER_ALLOC_PERCENTAGE, btree_size / 1024);


        let store = Arc::new(RwLock::new(Store {
            max_length: btree_size * 1024,
            length: 0,
            current: 0,
            btree: BTreeMap::new()
        }));

        tokio::spawn(Self::ensure_sufficient_remaining_memory(store.clone()));

        store
    }
    async fn ensure_sufficient_remaining_memory(store: Arc<RwLock<Self>>) {
        loop {
            delay_for(std::time::Duration::from_secs(2)).await;

            let mut store = store.write().await;
            let memory = sys_info::mem_info().unwrap();
            store.max_length = (((memory.avail + memory.swap_free) as f64 * INNER_ALLOC_PERCENTAGE) as usize) - store.length;
        }
    }
    pub fn append(&mut self, message_id: [u8; 6], message_type: [u8; 4], message: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let mut memory_id_8bit = [0u8; 8];
        &memory_id_8bit[2..8].copy_from_slice(&message_id);
        let message_offset = u64::from_le_bytes(memory_id_8bit);

        self.length += message.len();
        /* Remove lowest-key entries if there is no more space: */
        while self.length > self.max_length {
            let entry = self.btree.first_entry().unwrap();
            let (_, message) = entry.get();
            self.length -= message.len();
        }
        
        /* We add the message to the B-tree: */
        self.btree.insert(message_offset, (message_type, message));

        /* And update the cursor value: */
        self.current = message_offset;

        Ok(())
    }
    pub async fn pipe(&self, offset: u64, subscription: Arc<Subscription>, router: Arc<Router>) {
        /* Discard offsets above current stored: */
        if offset > self.current { return }

        println!("Reading from store");

        /* Create past selector: */
        let mut selector = Selector::new();

        /* Integrate selection into past selector: */
        subscription.integrate(&mut selector);

        /* Process past messages: */
        let registry = router.registry.read().await;
        for (&key, (message_type, message)) in self.btree.range(offset..) {
            println!("Store item: {} -> {:?} {:?}", key, message_type, message);
            let schema = registry.get(&message_type[..]).unwrap();
            selector.distribute(message_type, schema, message);
        }
        
        /* The past selector dies here, along with the subscription integration in it */
    }
}
