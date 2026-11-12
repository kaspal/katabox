struct Parser<'a> {
    text: &'a [u8],
    pos: usize,
}

impl Parser<'_> {
    fn peek(&mut self) -> u8 {
        while self.pos < self.text.len() && self.text[self.pos] == b' ' {
            self.pos += 1;
        }
        if self.pos < self.text.len() { self.text[self.pos] } else { 0 }
    }

    fn expr(&mut self) -> i64 {
        let mut v = self.term();
        loop {
            let c = self.peek();
            if c != b'+' && c != b'-' {
                return v;
            }
            self.pos += 1;
            let w = self.term();
            v = if c == b'+' { v + w } else { v - w };
        }
    }

    fn term(&mut self) -> i64 {
        let mut v = self.factor();
        while self.peek() == b'*' {
            self.pos += 1;
            v *= self.factor();
        }
        v
    }

    fn factor(&mut self) -> i64 {
        match self.peek() {
            b'-' => {
                self.pos += 1;
                -self.factor()
            }
            b'(' => {
                self.pos += 1;
                let v = self.expr();
                self.peek();
                self.pos += 1; // ')'
                v
            }
            _ => {
                let mut v = 0i64;
                while self.pos < self.text.len() && self.text[self.pos].is_ascii_digit() {
                    v = v * 10 + (self.text[self.pos] - b'0') as i64;
                    self.pos += 1;
                }
                v
            }
        }
    }
}

pub fn evaluate(expression: String) -> i64 {
    Parser { text: expression.as_bytes(), pos: 0 }.expr()
}
