use crate::object::Value;

#[derive(Default)]
pub struct ValueStack {
    values: Vec<Value>,
    frames: Vec<Frame>,
}

impl ValueStack {
    pub fn push_frame(&mut self) {
        let top = self.values.len();
        self.frames.push(Frame { top })
    }

    pub fn pop_frame(&mut self) -> Frame {
        let frame = self.frames.pop().expect("No frames to pop.");
        self.values.drain(frame.top..);
        frame
    }

    pub fn push_value(&mut self, value: Value) -> usize {
        let sp = self.values.len();
        self.values.push(value);
        sp
    }

    pub fn set_value(&mut self, sp: usize, value: Value) -> Option<Value> {
        let v = self.values.get_mut(sp)?;
        *v = value;
        Some(v.clone())
    }

    pub fn get_value(&self, sp: usize) -> Option<Value> {
        self.values.get(sp).cloned()
    }
}

pub struct Frame {
    top: usize,
}
