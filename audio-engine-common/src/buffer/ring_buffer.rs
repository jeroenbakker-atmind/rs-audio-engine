use std::ops::AddAssign;

pub struct RingBuffer<T>
where
    T: Sized,
{
    pub data: Vec<T>,

    /// Start of the ring buffer in the data attribute.
    pub start: usize,

    /// End of the ring buffer in the data attribute.
    pub end: usize,
}

impl<T> Default for RingBuffer<T>
where
    T: Default,
{
    fn default() -> Self {
        RingBuffer::<T> {
            data: Vec::<T>::default(),
            start: 0,
            end: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum PushOperation {
    Replace,
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
        if self.end >= self.start {
            self.end - self.start
        } else {
            self.capacity() - self.start + self.end
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl<T> RingBuffer<T>
where
    T: Sized + Default + Copy + AddAssign,
{
    pub fn with_capacity(capacity: usize) -> RingBuffer<T> {
        let mut result = RingBuffer::<T>::default();
        result.ensure_capacity(capacity);
        result
    }

    fn reserve(&mut self, capacity: usize) {
        let mut new_data = Vec::<T>::with_capacity(capacity);
        let len = self.len();

        for i in self.start..self.data.len() {
            new_data.push(self.data[i]);
        }
        if self.end < self.start {
            for i in 0..self.end {
                new_data.push(self.data[i]);
            }
        }
        for _i in 0..capacity - len {
            new_data.push(T::default());
        }

        self.data = new_data;
        self.start = 0;
        self.end = len;
    }

    pub fn ensure_capacity(&mut self, offset: usize) {
        if self.capacity() < offset {
            self.reserve(offset)
        }
    }

    fn calc_offset(&self, offset: usize) -> usize {
        (self.start + offset) % self.capacity()
    }

    /// Push a value into the ring buffer so that the value will be popped
    pub fn push(&mut self, offset: usize, value: T, operation: PushOperation) {
        assert!(self.capacity() > offset);
        let offset = self.calc_offset(offset);

        operation.perform(value, &mut self.data[offset]);
        self.end = (offset + 1) % self.capacity();
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            let result = self.data[self.start];
            self.data[self.start] = T::default();
            self.skip();
            Some(result)
        }
    }

    pub fn skip(&mut self) {
        if self.start != self.end {
            self.start = (self.start + 1) % self.capacity();
        }
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
        assert_eq!(buffer.start, 0);
        assert_eq!(buffer.end, 0);
        assert_eq!(buffer.len(), 0);

        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.start, 0);
        assert_eq!(buffer.end, 1);
        assert_eq!(buffer.len(), 1);

        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.start, 1);
        assert_eq!(buffer.end, 1);
        assert_eq!(buffer.len(), 0);

        assert_eq!(buffer.pop(), None);
        assert_eq!(buffer.start, 1);
        assert_eq!(buffer.end, 1);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn pop_0_multiple() {
        let mut buffer = RingBuffer::<i32>::with_capacity(5);
        assert_eq!(buffer.start, 0);
        assert_eq!(buffer.end, 0);
        assert_eq!(buffer.len(), 0);

        buffer.ensure_capacity(5);
        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.start, 0);
        assert_eq!(buffer.end, 1);
        assert_eq!(buffer.len(), 1);

        let result = buffer.pop();
        assert_eq!(result, Some(1));
        assert_eq!(buffer.start, 1);
        assert_eq!(buffer.end, 1);
        assert_eq!(buffer.len(), 0);

        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.start, 1);
        assert_eq!(buffer.end, 2);
        assert_eq!(buffer.len(), 1);

        let result = buffer.pop();
        assert_eq!(result, Some(1));
        assert_eq!(buffer.start, 2);
        assert_eq!(buffer.end, 2);
        assert_eq!(buffer.len(), 0);

        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.start, 2);
        assert_eq!(buffer.end, 3);
        assert_eq!(buffer.len(), 1);

        let result = buffer.pop();
        assert_eq!(result, Some(1));
        assert_eq!(buffer.start, 3);
        assert_eq!(buffer.end, 3);
        assert_eq!(buffer.len(), 0);

        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.start, 3);
        assert_eq!(buffer.end, 4);
        assert_eq!(buffer.len(), 1);

        let result = buffer.pop();
        assert_eq!(result, Some(1));
        assert_eq!(buffer.start, 4);
        assert_eq!(buffer.end, 4);
        assert_eq!(buffer.len(), 0);

        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.start, 4);
        assert_eq!(buffer.end, 0);
        assert_eq!(buffer.len(), 1);

        let result = buffer.pop();
        assert_eq!(result, Some(1));
        assert_eq!(buffer.start, 0);
        assert_eq!(buffer.end, 0);
        assert_eq!(buffer.len(), 0);

        buffer.push(0, 1, PushOperation::Add);
        assert_eq!(buffer.start, 0);
        assert_eq!(buffer.end, 1);
        assert_eq!(buffer.len(), 1);

        let result = buffer.pop();
        assert_eq!(result, Some(1));
        assert_eq!(buffer.start, 1);
        assert_eq!(buffer.end, 1);
        assert_eq!(buffer.len(), 0);
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
