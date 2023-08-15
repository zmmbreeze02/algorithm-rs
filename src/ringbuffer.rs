use std::{alloc::{alloc, Layout}, ptr};


/**
 * RingBuffer implementation
 * 
 * | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
 *       | head                           | tail
 * read from head, and write after tail
 */
pub struct RingBuffer<T> {
    buf: *mut T,
    // Buffer size
    // It must be a power of two.
    // When convert head position into index of buffer, it will become much easier.
    // Can't be modified in runtime.
    capacity: usize,
    // head position, read from head
    head: usize,
    // tail position, write after tail
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

    /// Turn position(head and tail) into index with offset.
    fn position_to_index(cap: usize, position: usize) -> usize {
        position & (cap - 1)
    }

    /// Push new element after tail position.
    /// If it is full, then pop the head element, and push the new element.
    pub fn push(&mut self, value: T) {
        if self.is_full() {
            // Manually drop the head element
            unsafe {
                let index = Self::position_to_index(self.capacity, self.head);
                // println!("pop index: {:?}", index);
                let old_value = ptr::read(self.buf.add(index));
                drop(old_value);
            }

            self.head += 1;
        }

        // Calculate the index to push
        let index = Self::position_to_index(self.capacity, self.tail);
        // println!("push index: {:?}", index);
        unsafe {
            ptr::write(self.buf.add(index), value);
        }

        self.tail += 1;
    }

    pub fn enqueue(&mut self, value: T) {
        self.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let index = Self::position_to_index(self.capacity, self.head);
        // println!("pop index: {:?}", index);
        let p = unsafe {
            self.buf.add(index)
        };
        self.head += 1;

        unsafe {
            Some(ptr::read(p))
        }
    }

    /// Empties the buffer entirely. Sets the length to 0 but keeps the capacity allocated.
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
    }

    /// Turn position(head and tail) into index with offset.
    /// When offset is [-1, `-len`], get the last elements pushed index.
    /// When offset is [0, `len - 1`], get the items that were pushed the longest ago.
    fn position_to_index_with_offset(cap: usize, position: usize, len: usize, offset: isize) -> Option<usize> {
        let real_offset: isize = 
            if offset >= 0 { offset }
            else { len as isize + offset };

        if real_offset < 0 || real_offset >= len as isize {
            return None;
        }
        
        // println!("offset: {:?}", real_offset);
        Some(Self::position_to_index(cap, position + real_offset as usize))
    }

    /// Gets a value relative to the current index.
    /// -1 and down are the last elements pushed.
    /// 0 and up are the items that were pushed the longest ago.
    pub fn get(&self, index: isize) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let index = Self::position_to_index_with_offset(
            self.capacity,
            self.head,
            self.len(),
            index
        );
        // println!("get: {:?}", index);
        index.map(|i| {
            unsafe {
                // Move pointer to specified element
                let p = self.buf.add(i);
                &*p
            }
        })
    }

    /// Gets a value relative to the current index mutably.
    /// -1 and down are the last elements pushed.
    /// 0 and up are the items that were pushed the longest ago.
    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }
        let index = Self::position_to_index_with_offset(
            self.capacity,
            self.head,
            self.len(),
            index
        );
        index.map(|i| {
            unsafe {
                // Move pointer to specified element
                let p = self.buf.add(i);
                &mut *p
            }
        })
    }

    pub fn len(&self) -> usize {
        self.tail.wrapping_sub(self.head)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }
}


#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn test_ring_buffer() {
        let mut rb: RingBuffer<u32> = RingBuffer::with_capacity(8);
        assert_eq!(rb.is_empty(), true);
        rb.push(0);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        rb.push(4);
        rb.push(5);
        rb.push(6);
        assert_eq!(rb.is_full(), false);
        rb.push(7);
        // |start 0 1 2 3 4 5 6 7

        assert_eq!(rb.len(), 8);
        assert_eq!(rb.is_full(), true);
        assert_eq!(rb.is_empty(), false);
        assert_eq!(rb.get(0), Some(&0));
        assert_eq!(rb.get(7), Some(&7));
        assert_eq!(rb.get(8), None);
        assert_eq!(rb.get(-1), Some(&7));
        assert_eq!(rb.get(-8), Some(&0));

        assert_eq!(rb.dequeue(), Some(0));
        assert_eq!(rb.is_full(), false);
        assert_eq!(rb.dequeue(), Some(1));
        assert_eq!(rb.dequeue(), Some(2));
        assert_eq!(rb.dequeue(), Some(3));
        assert_eq!(rb.dequeue(), Some(4));
        assert_eq!(rb.dequeue(), Some(5));
        assert_eq!(rb.dequeue(), Some(6));
        assert_eq!(rb.dequeue(), Some(7));
        assert_eq!(rb.dequeue(), None);

        rb.push(0);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        rb.push(4);
        rb.push(5);
        rb.push(6);
        assert_eq!(rb.is_full(), false);
        rb.push(7);
        rb.push(8);
        rb.push(9);
        // 8 9 |start 2 3 4 5 6 7
        
        assert_eq!(rb.len(), 8);
        assert_eq!(rb.is_full(), true);
        assert_eq!(rb.is_empty(), false);
        assert_eq!(rb.get(0), Some(&2));
        assert_eq!(rb.get(1), Some(&3));
        assert_eq!(rb.get(7), Some(&9));
        assert_eq!(rb.get(8), None);
        assert_eq!(rb.get(-1), Some(&9));
        assert_eq!(rb.get(-2), Some(&8));
        assert_eq!(rb.get(-8), Some(&2));

        rb.clear();
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.is_full(), false);
        assert_eq!(rb.is_empty(), true);
    }

}