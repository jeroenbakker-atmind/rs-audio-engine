//! RingBuffer
//! 
//! Rust has a ring buffer (#std::collections::VecDeque). Our implementation adds
//! methods to that are specific to audio processing.
//! 
//! When pushing a new value to the buffer an operation can be specified how the
//! value is applied to the existing value in the buffer.
use std::{collections::VecDeque, ops::AddAssign};

#[derive(Debug, Clone)]
pub struct RingBuffer<T>
where
    T: Sized,
{
    pub data: VecDeque<T>,
}

impl<T> Default for RingBuffer<T>
where
    T: Default,
{
    fn default() -> Self {
        RingBuffer::<T> {
            data: VecDeque::<T>::default(),
        }
    }
}

/// Operation to perform when pushing a new value to a ring buffer.
#[derive(Copy, Clone)]
pub enum PushOperation {
    /// Value will overwrite the current value in the buffer.
    Replace,
    /// Value will be added to the current value in the buffer. 
    Add,
}

impl PushOperation {
    fn perform<T: Copy + AddAssign>(&self, value: T, r_value: &mut T) {
        match self {
            PushOperation::Replace => *r_value = value,
            PushOperation::Add => *r_value += value,
        }
    }
}

impl<T> RingBuffer<T>
where
    T: Sized,
{
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl<T> RingBuffer<T>
where
    T: Sized + Default + Copy + AddAssign,
{
    pub fn with_capacity(capacity: usize) -> RingBuffer<T> {
        RingBuffer::<T> {
            data: VecDeque::<T>::with_capacity(capacity),
        }
    }

    pub fn ensure_size(&mut self, size: usize) {
        if self.data.len() < size {
            self.data.resize(size, T::default());
        }
    }

    /// Push a value into the ring buffer so that the value will be popped
    pub fn push(&mut self, offset: usize, value: T, operation: PushOperation) {
        self.ensure_size(offset + 1);
        operation.perform(value, self.data.get_mut(offset).unwrap());
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn pop_or_default(&mut self) -> T {
        match self.pop() {
            Some(value) => value,
            None => T::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{PushOperation, RingBuffer};

    #[test]
    fn pop_0() {
        let mut buffer = RingBuffer::<i32>::with_capacity(5);

        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn pop_0_multiple() {
        let mut buffer = RingBuffer::<i32>::with_capacity(5);
        buffer.push(0, 1, PushOperation::Add);
        let result = buffer.pop();
        assert_eq!(result, Some(1));
        buffer.push(0, 1, PushOperation::Add);
        let result = buffer.pop();
        assert_eq!(result, Some(1));
        buffer.push(0, 1, PushOperation::Add);
        let result = buffer.pop();
        assert_eq!(result, Some(1));
        buffer.push(0, 1, PushOperation::Add);
        let result = buffer.pop();
        assert_eq!(result, Some(1));
        buffer.push(0, 1, PushOperation::Add);
        let result = buffer.pop();
        assert_eq!(result, Some(1));
        buffer.push(0, 1, PushOperation::Add);
        let result = buffer.pop();
        assert_eq!(result, Some(1));
    }

    #[test]
    fn pop_3() {
        let mut buffer = RingBuffer::<i32>::with_capacity(5);
        buffer.push(3, 1, PushOperation::Add);
        assert_eq!(buffer.pop(), Some(0));
        assert_eq!(buffer.pop(), Some(0));
        assert_eq!(buffer.pop(), Some(0));
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn pop_or_default() {
        let mut buffer = RingBuffer::<i32>::with_capacity(5);
        buffer.push(3, 1, PushOperation::Add);
        assert_eq!(buffer.pop_or_default(), 0);
        assert_eq!(buffer.pop_or_default(), 0);
        assert_eq!(buffer.pop_or_default(), 0);
        assert_eq!(buffer.pop_or_default(), 1);
        assert_eq!(buffer.pop_or_default(), 0);
    }
}
