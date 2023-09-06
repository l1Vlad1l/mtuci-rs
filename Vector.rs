struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector {
            data: Vec::new(),
        }
    }
    
    fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
        }
    }
    
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    
    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    
    fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, Default::default());
    }
}