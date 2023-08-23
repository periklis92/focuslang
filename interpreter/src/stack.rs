#[derive(Default)]
pub struct ValueStack {
    values: Vec<u8>,
    frames: Vec<Frame>,
}

impl ValueStack {
    pub fn new() -> Self {
        Self {
            values: Vec::default(),
            frames: Vec::default(),
        }
    }

    pub fn push_frame(&mut self) {
        let top = self.values.len();
        self.frames.push(Frame { top })
    }

    pub fn pop_frame(&mut self) -> Frame {
        let frame = self.frames.pop().expect("No frames to pop.");
        self.values.drain(frame.top..);
        frame
    }

    #[inline]
    fn extend(&mut self, bytes: &[u8]) -> usize {
        let sp = self.values.len();
        self.values.extend_from_slice(bytes);
        sp
    }

    pub fn push_char(&mut self, c: char) -> usize {
        let sp = self.values.len();
        for b in c
            .encode_utf8(&mut [0; 4])
            .as_bytes()
            .iter()
            .chain(&[0; 4])
            .take(4)
        {
            self.values.push(*b);
        }
        sp
    }

    pub fn push_bool(&mut self, b: bool) -> usize {
        self.extend(&[b as u8])
    }

    pub fn push_int(&mut self, i: i64) -> usize {
        self.extend(&i.to_be_bytes())
    }

    pub fn push_float(&mut self, f: f64) -> usize {
        self.extend(&f.to_be_bytes())
    }

    pub fn get_char(&self, sp: usize) -> Option<char> {
        let bytes = self.values.get(sp..sp + std::mem::size_of::<char>())?;
        std::str::from_utf8(bytes)
            .ok()
            .map(|s| s.chars().next())
            .flatten()
    }

    pub fn get_bool(&self, sp: usize) -> Option<bool> {
        self.values.get(sp).map(|b| *b != 0)
    }

    pub fn get_int(&self, sp: usize) -> Option<i64> {
        let bytes = self.values.get(sp..sp + std::mem::size_of::<i64>())?;
        Some(i64::from_be_bytes(bytes.try_into().ok()?))
    }

    pub fn get_float(&self, sp: usize) -> Option<f64> {
        let bytes = self.values.get(sp..sp + std::mem::size_of::<f64>())?;
        Some(f64::from_be_bytes(bytes.try_into().ok()?))
    }

    pub fn set_char(&mut self, sp: usize, c: char) -> Option<char> {
        if self.values.len() <= sp {
            None
        } else {
            for (i, b) in c
                .encode_utf8(&mut [0; 4])
                .as_bytes()
                .iter()
                .chain(&[0; 4])
                .take(4)
                .enumerate()
            {
                self.values[sp + i] = *b;
            }
            Some(c)
        }
    }

    pub fn set_bool(&mut self, sp: usize, b: bool) -> Option<bool> {
        if self.values.len() <= sp {
            None
        } else {
            self.values[sp] = b as u8;
            Some(b)
        }
    }

    pub fn set_int(&mut self, sp: usize, i: i64) -> Option<i64> {
        if self.values.len() <= sp {
            None
        } else {
            self.values[sp..sp + std::mem::size_of::<i64>()].copy_from_slice(&i.to_be_bytes());
            Some(i)
        }
    }

    pub fn set_float(&mut self, sp: usize, f: f64) -> Option<f64> {
        if self.values.len() <= sp {
            None
        } else {
            self.values[sp..sp + std::mem::size_of::<f64>()].copy_from_slice(&f.to_be_bytes());
            Some(f)
        }
    }
}

pub struct Frame {
    top: usize,
}

impl Frame {
    pub fn new(top: usize) -> Self {
        Self { top }
    }
}

#[cfg(test)]
mod tests {
    use super::ValueStack;

    #[test]
    fn char() {
        let mut stack = ValueStack::new();

        let sp = stack.push_char('a');
        let val = stack.get_char(sp);
        assert!(val.is_some_and(|v| v == 'a'));

        let sp = stack.push_char('ά');
        let val = stack.get_char(sp);
        assert!(val.is_some_and(|v| v == 'ά'));
    }

    #[test]
    fn set_char() {
        let mut stack = ValueStack::new();

        let sp = stack.push_char('a');
        stack.set_char(sp, 'b');
        let val = stack.get_char(sp);
        assert!(val.is_some_and(|v| v == 'b'));

        let sp = stack.push_char('ά');
        stack.set_char(sp, 'ί');
        let val = stack.get_char(sp);
        assert!(val.is_some_and(|v| v == 'ί'));
    }

    #[test]
    fn bool() {
        let mut stack = ValueStack::new();

        let sp = stack.push_bool(true);
        let val = stack.get_bool(sp);
        assert!(val.is_some_and(|v| v == true));

        let sp = stack.push_bool(false);
        let val = stack.get_bool(sp);
        assert!(val.is_some_and(|v| v == false));
    }

    #[test]
    fn set_bool() {
        let mut stack = ValueStack::new();

        let sp = stack.push_bool(true);
        stack.set_bool(sp, false);
        let val = stack.get_bool(sp);
        assert!(val.is_some_and(|v| v == false));

        let sp = stack.push_bool(false);
        stack.set_bool(sp, true);
        let val = stack.get_bool(sp);
        assert!(val.is_some_and(|v| v == true));
    }

    #[test]
    fn int() {
        let mut stack = ValueStack::new();

        let sp = stack.push_int(1);
        let val = stack.get_int(sp);
        assert!(val.is_some_and(|v| v == 1));

        let sp = stack.push_int(78846);
        let val = stack.get_int(sp);
        assert!(val.is_some_and(|v| v == 78846));
    }

    #[test]
    fn set_int() {
        let mut stack = ValueStack::new();

        let sp = stack.push_int(1);
        stack.set_int(sp, 2);
        let val = stack.get_int(sp);
        assert!(val.is_some_and(|v| v == 2));

        let sp = stack.push_int(-100000005);
        stack.set_int(sp, 1000000058);
        let val = stack.get_int(sp);
        assert!(val.is_some_and(|v| v == 1000000058));
    }

    #[test]
    fn float() {
        let mut stack = ValueStack::new();

        let sp = stack.push_float(1.4);
        let val = stack.get_float(sp);
        assert!(val.is_some_and(|v| v == 1.4));

        let sp = stack.push_float(-82.54);
        let val = stack.get_float(sp);
        assert!(val.is_some_and(|v| v == -82.54));
    }

    #[test]
    fn set_float() {
        let mut stack = ValueStack::new();

        let sp = stack.push_float(1.4);
        stack.set_float(sp, -6.5);
        let val = stack.get_float(sp);
        assert!(val.is_some_and(|v| v == -6.5));

        let sp = stack.push_float(-7987.234);
        stack.set_float(sp, 79827.234);
        let val = stack.get_float(sp);
        assert!(val.is_some_and(|v| v == 79827.234));
    }
}
