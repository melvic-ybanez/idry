pub type Value = f64;

#[derive(Debug)]
pub struct Values {
    pub values: Vec<Value>,
}

impl Values {
    pub fn write_value(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }
}

impl Default for Values {
    fn default() -> Self {
        Self { values: vec![] }
    }
}

pub fn print(value: Value) {
    print!("{}", value)
}