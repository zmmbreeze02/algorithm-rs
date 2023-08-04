use std::alloc::{alloc, Layout};


pub struct RingBuffer<T> {
    buf: *mut T,
    capacity: usize,
    head: usize,
    tail: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(cap: usize) -> Self {
        assert_ne!(cap, 0, "Capacity must be greater than 0");
        assert!(cap.is_power_of_two(), "Capacity must be a power of two");

        
        let layout = Layout::array::<T>(cap).unwrap();
        let buf = unsafe { alloc(layout) as *mut T };

        Self {
            buf,
            capacity: cap,
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, value: T) {

    }

    pub fn enqueue(&mut self, value: T) {
        self.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        Option::None
    }

    /// Empties the buffer entirely. Sets the length to 0 but keeps the capacity allocated.
    pub fn clear(&mut self) {

    }

    /// Gets a value relative to the current index. 0 is the next index to be written to with push.
    /// -1 and down are the last elements pushed and 0 and up are the items that were pushed the longest ago.
    pub fn get(&self, index: isize) -> Option<&T> {
        Option::None
    }

    /// Gets a value relative to the current index mutably. 0 is the next index to be written to with push.
    /// -1 and down are the last elements pushed and 0 and up are the items that were pushed the longest ago.
    #[inline]
    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        // Safety: self is a RingBuffer
        unsafe { Self::ptr_get_mut(self, index).map(|i| &mut *i) }
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }


}

