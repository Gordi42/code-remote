#[derive(Debug, Default, PartialEq)]
pub struct Counter {
    value: u32,
    length: u32,
}

impl Counter {
    pub fn new(length: u32) -> Self {
        Counter { value: 0, length }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn update_length(&mut self, length: u32) {
        self.length = length;
        if self.value >= self.length {
            self.value = self.length - 1;
        }
    }

    pub fn increment(&mut self) {
        self.value += 1;
        if self.value >= self.length {
            self.value = 0;
        }
    }

    pub fn decrement(&mut self) {
        if self.value == 0 {
            self.value = self.length;
        }
        self.value -= 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let counter = Counter::new(10);
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_get_value() {
        let mut counter = Counter::new(10);
        counter.increment();
        assert_eq!(counter.get_value(), 1);
    }

    #[test]
    fn test_increment() {
        let mut counter = Counter::new(3);
        counter.increment();
        assert_eq!(counter.value, 1);
        counter.increment();
        assert_eq!(counter.value, 2);
        counter.increment();
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_decrement() {
        let mut counter = Counter::new(3);
        counter.decrement();
        assert_eq!(counter.value, 2);
        counter.decrement();
        assert_eq!(counter.value, 1);
        counter.decrement();
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_update_length() {
        let mut counter = Counter::new(3);
        counter.increment();
        counter.increment();
        assert_eq!(counter.value, 2);
        counter.update_length(2);
        assert_eq!(counter.length, 2);
        assert_eq!(counter.value, 1);
    }
}

