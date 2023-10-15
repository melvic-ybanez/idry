pub type Value = f64;

#[derive(Debug)]
pub struct Values {
    values: Vec<Value>,
}

impl Values {
    pub fn write(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn values(&self) -> &Vec<Value> {
        &self.values
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