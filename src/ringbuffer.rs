use std::{alloc::{alloc, Layout, dealloc}, ptr::{self, NonNull}, sync::atomic::{AtomicUsize, Ordering}};


/**
 * RingBuffer implementation
 * 
 * | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
 *       | head                           | tail
 * read from head, and write after tail
 * 
 * SAFTY: Only support exclusive reader thread and exclusive writer thread, otherwise use Mutex instead.
 * You can call push methods to write ringbuffer.
 * You can call pop methods to read ringbuffer.
 */
pub struct RingBuffer<T> {
    buf: *mut T,
    // Buffer size
    // It must be a power of two.
    // When convert head position into index of buffer, it will become much easier.
    // Can't be modified in runtime.
    capacity: usize,
    // head position, read from head
    head: AtomicUsize,
    // tail position, write after tail
    tail: AtomicUsize,
    // Counter of writer and reader, maximum size is 2.
    counter: AtomicUsize,
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
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            counter: AtomicUsize::new(2),
        }
    }

    /// Turn position(head and tail) into index with offset.
    fn position_to_index(cap: usize, position: usize) -> usize {
        position & (cap - 1)
    }

    /// Push new element after tail position.
    /// If it is full, then pop the head element, and push the new element.
    pub fn push(&mut self, value: T) -> bool {
        if self.is_full() {
            return false;
        }

        // Calculate the index to push
        let index = Self::position_to_index(self.capacity, self.tail.load(Ordering::Acquire));
        // println!("push index: {:?}", index);
        unsafe {
            // Push the new element
            ptr::write(self.buf.add(index), value);
        }

        self.tail.fetch_add(1, Ordering::AcqRel);
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let index = Self::position_to_index(self.capacity, self.head.load(Ordering::Acquire));
        // println!("pop index: {:?}", index);
        let res = unsafe {
            Some(ptr::read(self.buf.add(index)))
        };

        self.head.fetch_add(1, Ordering::AcqRel);
        res
    }

    pub fn len(&self) -> usize {
        self.tail.load(Ordering::Acquire).wrapping_sub(self.head.load(Ordering::Acquire))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }

    // /// Turn position(head and tail) into index with offset.
    // /// When offset is [-1, `-len`], get the last elements pushed index.
    // /// When offset is [0, `len - 1`], get the items that were pushed the longest ago.
    // fn position_to_index_with_offset(cap: usize, position: usize, len: usize, offset: isize) -> Option<usize> {
    //     let real_offset: isize = 
    //         if offset >= 0 { offset }
    //         else { len as isize + offset };

    //     if real_offset < 0 || real_offset >= len as isize {
    //         return None;
    //     }
        
    //     // println!("offset: {:?}", real_offset);
    //     Some(Self::position_to_index(cap, position + real_offset as usize))
    // }

    // /// Gets a value relative to the current index.
    // /// -1 and down are the last elements pushed.
    // /// 0 and up are the items that were pushed the longest ago.
    // pub fn get(&self, index: isize) -> Option<&T> {
    //     if self.is_empty() {
    //         return None;
    //     }

    //     let index = Self::position_to_index_with_offset(
    //         self.capacity,
    //         self.head.load(Ordering::Acquire),
    //         self.len(),
    //         index
    //     );
    //     // println!("get: {:?}", index);
    //     index.map(|i| {
    //         unsafe {
    //             // Move pointer to specified element
    //             let p = self.buf.add(i);
    //             &*p
    //         }
    //     })
    // }

    // /// Gets a value relative to the current index mutably.
    // /// -1 and down are the last elements pushed.
    // /// 0 and up are the items that were pushed the longest ago.
    // pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
    //     if self.is_empty() {
    //         return None;
    //     }
    //     let index = Self::position_to_index_with_offset(
    //         self.capacity,
    //         self.head.load(Ordering::Acquire),
    //         self.len(),
    //         index
    //     );
    //     index.map(|i| {
    //         unsafe {
    //             // Move pointer to specified element
    //             let p = self.buf.add(i);
    //             &mut *p
    //         }
    //     })
    // }

    pub fn release(&mut self) {
        if self.counter.fetch_sub(1, Ordering::AcqRel) == 1 {
            loop {
                if self.pop().is_none() {
                    let layout = Layout::array::<T>(self.capacity).unwrap();
                    unsafe { dealloc(self.buf.cast(), layout) };
                    return;
                }
            }
        }
    }
}

/// build Ringbuffer with capacity.
pub fn ringbuffer<T>(capacity: usize) -> (RingbufferWriter<T>, RingbufferReader<T>) {
    let inner: *mut RingBuffer<T> = Box::into_raw(Box::new(RingBuffer::with_capacity(capacity)));
    (
        RingbufferWriter {inner: NonNull::new(inner).unwrap()},
        RingbufferReader {inner: NonNull::new(inner).unwrap()}
    )
}

pub struct RingbufferWriter<T> {
    inner: NonNull<RingBuffer<T>>
}

impl<T> RingbufferWriter<T> {
    pub fn push(&mut self, value: T) -> bool {
        unsafe {
            self.inner.as_mut().push(value)
        }
    }
    
    pub fn len(&self) -> usize {
        unsafe {
            self.inner.as_ref().len()
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            self.inner.as_ref().is_empty()
        }
    }

    pub fn is_full(&self) -> bool {
        unsafe {
            self.inner.as_ref().is_full()
        }
    }
}

impl<T> Drop for RingbufferWriter<T> {
    fn drop(&mut self) {
        unsafe {
            self.inner.as_mut().release();
        }
    }
}

pub struct RingbufferReader<T> {
    inner: NonNull<RingBuffer<T>>
}

impl<T> RingbufferReader<T> {
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            self.inner.as_mut().pop()
        }
    }
    
    pub fn len(&self) -> usize {
        unsafe {
            self.inner.as_ref().len()
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            self.inner.as_ref().is_empty()
        }
    }

    pub fn is_full(&self) -> bool {
        unsafe {
            self.inner.as_ref().is_full()
        }
    }
}

impl<T> Drop for RingbufferReader<T> {
    fn drop(&mut self) {
        unsafe {
            self.inner.as_mut().release();
        }
    }
}

// unsafe impl<T: Sync> Sync for RingBuffer<T> {}
// unsafe impl<T: Send> Send for RingBuffer<T> {}

unsafe impl<T: Sync> Sync for RingbufferWriter<T> {}
unsafe impl<T: Sync> Sync for RingbufferReader<T> {}
unsafe impl<T: Send> Send for RingbufferWriter<T> {}
unsafe impl<T: Send> Send for RingbufferReader<T> {}


#[cfg(test)]
mod tests {
    use std::{sync::Mutex, thread};
    use crate::ringbuffer::ringbuffer;

    #[test]
    fn test_ring_buffer() {
        let (mut writer, mut reader) = ringbuffer(8);
        assert_eq!(writer.is_empty(), true);
        writer.push(0);
        writer.push(1);
        writer.push(2);
        writer.push(3);
        writer.push(4);
        writer.push(5);
        writer.push(6);
        assert_eq!(writer.is_full(), false);
        writer.push(7);
        // |start 0 1 2 3 4 5 6 7

        assert_eq!(reader.len(), 8);
        assert_eq!(reader.is_full(), true);
        assert_eq!(reader.is_empty(), false);

        assert_eq!(reader.pop(), Some(0));
        assert_eq!(reader.is_full(), false);
        assert_eq!(reader.pop(), Some(1));
        assert_eq!(reader.pop(), Some(2));
        assert_eq!(reader.pop(), Some(3));
        assert_eq!(reader.pop(), Some(4));
        assert_eq!(reader.pop(), Some(5));
        assert_eq!(reader.pop(), Some(6));
        assert_eq!(reader.pop(), Some(7));
        assert_eq!(reader.pop(), None);

        writer.push(0);
        writer.push(1);
        writer.push(2);
        writer.push(3);
        writer.push(4);
        writer.push(5);
        writer.push(6);
        assert_eq!(writer.is_full(), false);
        writer.push(7);
        assert_eq!(writer.push(8), false);
        assert_eq!(writer.push(9), false);
        // 8 9 |start 2 3 4 5 6 7
        
        assert_eq!(reader.len(), 8);
        assert_eq!(reader.is_full(), true);
        assert_eq!(reader.is_empty(), false);
    }

    #[test]
    fn test_async_ringbuffer() {
        let (mut writer, mut reader) = ringbuffer(1024);
        let a = thread::spawn(move || {
            for i in 0..1025 {
                writer.push(i);
            }
        });

        let b = thread::spawn(move || {
            let mut i = 0;
            loop {
                if let Some(value) = reader.pop() {
                    assert_eq!(value, i);
                    assert_eq!(value < 1024, true);
                    i += 1;
                } else {
                    break;
                }
            }
        });

        a.join().expect("Couldn't join on the associated thread");
        b.join().expect("Couldn't join on the associated thread");
    }

    static A_DROPPED: Mutex<usize> = Mutex::new(0);
    #[derive(Debug)]
    struct A(u32, String);
    impl Drop for A {
        fn drop(&mut self) {
            if let Ok(mut a) = A_DROPPED.lock() {
                // println!("A dropped {a}");
                *a += 1;
            }
        }
    }
    impl PartialEq for A {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }

    #[test]
    fn test_drop() {
        let (mut writer, mut reader) = ringbuffer(1024);
        writer.push(A(0, String::from("0")));
        writer.push(A(1, String::from("1")));
        writer.push(A(2, String::from("2")));
        assert_eq!(reader.pop().unwrap().0, 0);
        // println!("after reader");
        drop(writer);
        drop(reader);
        // println!("after drop");
        assert_eq!(*A_DROPPED.lock().unwrap(), 3);
    }
}