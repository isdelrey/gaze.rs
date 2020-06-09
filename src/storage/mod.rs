use sys_info;

const INNER_ALLOC_PERCENTAGE: f64 = 0.80;

pub struct Store {
    inner: Vec<u8>,
    insert_position: isize
}

impl Store {
    pub fn new() {
        let memory = sys_info::mem_info().unwrap();
        let inner_size = memory.free * INNER_ALLOC_PERCENTAGE;
        Store {
            inner: Vec::with_capacity(inner_size),
            insert_position: 0
        }
    }
    pub fn append(&mut self, buf: &[u8]) -> Result<usize> {
        if(self.inner.len() + buf.len() >= self.inner.capacity()) {
            self.inner[insert_position] = message;
        }
        self.inner.push(message);
    }
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if(self.inner.len() == self.inner.capacity()) {
            self.inner[insert_position] = message;
        }
        self.inner.push(message);
    }

}