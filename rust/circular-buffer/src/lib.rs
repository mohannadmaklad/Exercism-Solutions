pub struct CircularBuffer<T> {
    read_index: usize,
    write_index: usize,
    elements: Vec<Option<T>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            read_index: 0,
            write_index: 0,
            elements: (0..capacity).map(|_| None).collect(),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.elements[self.write_index].is_some() {
            return Err(Error::FullBuffer);
        } else {
            self.elements[self.write_index] = Some(element);
            self.write_index = (self.write_index + 1) % self.elements.len();
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.elements[self.read_index].is_none() {
            return Err(Error::EmptyBuffer);
        } else {
            let element = self.elements[self.read_index]
                .take()
                .ok_or(Error::EmptyBuffer);
            self.read_index = (self.read_index + 1) % self.elements.len();
            element
        }
    }

    pub fn clear(&mut self) {
        self.elements = (0..self.elements.len()).map(|_| None).collect();
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.elements[self.write_index].is_some() {
            self.elements[self.write_index] = Some(element);
            self.read_index = (self.read_index + 1) % self.elements.len();
        } else {
            let _ = self.write(element);
        }
    }
}
