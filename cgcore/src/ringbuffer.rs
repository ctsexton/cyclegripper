pub struct Ringbuffer {
    data: Vec<f32>,
    read_position: usize,
    write_position: usize,
}

impl Ringbuffer {
    pub fn new(len: usize) -> Self {
        Self { data: vec![0_f32; len], read_position: 0, write_position: 0 }
    }

    pub fn write(&mut self, value: f32) {
        let write_to = self.data.get_mut(self.write_position).unwrap();
        *write_to = value;
        self.write_position = (self.write_position + 1) % self.data.len();
    }

    pub fn read(&mut self) -> f32 {
        let value = self.data.get(self.read_position).unwrap();
        self.read_position = (self.read_position + 1) % self.data.len();
        *value
    }

    pub fn set_read_offset(&mut self, offset: usize) {
        if offset > self.data.len() {
            return;
        }
        self.read_position = if self.write_position >= offset {
            self.write_position - offset
        } else {
            self.data.len() - (offset - self.write_position)
        };
    }
}
