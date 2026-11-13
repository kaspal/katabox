    // ---- JSON (ADR 0008, ADR 0026) ------------------------------------------------------
    // The harness's JSON reader and writer. Injected into the rendered harness (kb-judge
    // codegen) and `include!`d by kb-judge's reader tests, so the tested code is the shipped
    // code. Plain Rust, no template syntax, standard library only.
    //
    // Speed matters: Rust problems send requests of several MB, and the reader used to cost
    // more CPU than the solutions. Integers are parsed once into `Int`; any other number is
    // kept as written; strings copy unescaped runs in bulk; nesting is capped so a broken
    // request fails cleanly (exit 70) instead of overflowing the stack.

    #[derive(Clone)]
    pub enum Json {
        Null,
        Bool(bool),
        /// A number written as a plain integer that fits an i64 (`-12`, `007`).
        Int(i64),
        /// Any other number, as written (`2.5`, `1e3`, `+4`), converted on demand.
        Num(String),
        Str(String),
        Arr(Vec<Json>),
        Obj(Vec<(String, Json)>),
    }

    /// The same text the derived Debug of the earlier `Num(String)`-only enum printed, so
    /// error messages that show a value do not change.
    impl std::fmt::Debug for Json {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Json::Null => f.write_str("Null"),
                Json::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
                Json::Int(n) => f.debug_tuple("Num").field(&n.to_string()).finish(),
                Json::Num(n) => f.debug_tuple("Num").field(n).finish(),
                Json::Str(s) => f.debug_tuple("Str").field(s).finish(),
                Json::Arr(items) => f.debug_tuple("Arr").field(items).finish(),
                Json::Obj(fields) => f.debug_tuple("Obj").field(fields).finish(),
            }
        }
    }

    impl Json {
        pub fn get(&self, key: &str) -> Option<&Json> {
            match self {
                Json::Obj(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
                _ => None,
            }
        }
    }

    /// Deeper nesting than this is refused. Requests nest a few levels at most; the cap
    /// only turns an absurd request into a clean error (exit 70) instead of a stack
    /// overflow, and is far below what the reading thread's 64 MB stack can hold.
    pub const MAX_DEPTH: usize = 10_000;

    pub struct Parser<'a> {
        s: &'a [u8],
        i: usize,
        depth: usize,
    }

    impl<'a> Parser<'a> {
        pub fn new(s: &'a str) -> Self {
            Parser { s: s.as_bytes(), i: 0, depth: 0 }
        }
    }

    impl Parser<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
                self.i += 1;
            }
        }

        fn eat(&mut self, lit: &str) -> Result<(), String> {
            if self.s[self.i..].starts_with(lit.as_bytes()) {
                self.i += lit.len();
                Ok(())
            } else {
                Err(format!("bad JSON at byte {}", self.i))
            }
        }

        fn value(&mut self) -> Result<Json, String> {
            self.ws();
            let c = *self.s.get(self.i).ok_or("unexpected end of JSON")?;
            let v = match c {
                b'n' => self.eat("null").map(|_| Json::Null)?,
                b't' => self.eat("true").map(|_| Json::Bool(true))?,
                b'f' => self.eat("false").map(|_| Json::Bool(false))?,
                b'"' => Json::Str(self.string()?),
                b'[' => {
                    self.enter()?;
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&b']') {
                        self.i += 1;
                    } else {
                        loop {
                            items.push(self.value()?);
                            self.ws();
                            match self.s.get(self.i) {
                                Some(b',') => self.i += 1,
                                Some(b']') => {
                                    self.i += 1;
                                    break;
                                }
                                _ => return Err(format!("bad JSON array at byte {}", self.i)),
                            }
                        }
                    }
                    self.depth -= 1;
                    Json::Arr(items)
                }
                b'{' => {
                    self.enter()?;
                    self.i += 1;
                    let mut fields = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&b'}') {
                        self.i += 1;
                    } else {
                        loop {
                            self.ws();
                            let k = self.string()?;
                            self.ws();
                            self.eat(":")?;
                            fields.push((k, self.value()?));
                            self.ws();
                            match self.s.get(self.i) {
                                Some(b',') => self.i += 1,
                                Some(b'}') => {
                                    self.i += 1;
                                    break;
                                }
                                _ => return Err(format!("bad JSON object at byte {}", self.i)),
                            }
                        }
                    }
                    self.depth -= 1;
                    Json::Obj(fields)
                }
                _ => self.number()?,
            };
            Ok(v)
        }

        fn enter(&mut self) -> Result<(), String> {
            self.depth += 1;
            if self.depth > MAX_DEPTH {
                return Err(format!("JSON nested deeper than {MAX_DEPTH} levels"));
            }
            Ok(())
        }

        /// A run of number characters. A plain integer (optional `-`, then digits) that
        /// fits becomes `Int`; anything else is kept as written, as before.
        fn number(&mut self) -> Result<Json, String> {
            let start = self.i;
            while self.i < self.s.len()
                && matches!(self.s[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
            {
                self.i += 1;
            }
            if start == self.i {
                return Err(format!("bad JSON at byte {start}"));
            }
            let text = &self.s[start..self.i];
            let (negative, digits) = match text.split_first() {
                Some((b'-', rest)) => (true, rest),
                _ => (false, text),
            };
            if !digits.is_empty() && digits.len() <= 18 && digits.iter().all(u8::is_ascii_digit) {
                // At most 18 digits cannot overflow an i64.
                let mut n: i64 = 0;
                for &d in digits {
                    n = n * 10 + (d - b'0') as i64;
                }
                return Ok(Json::Int(if negative { -n } else { n }));
            }
            Ok(Json::Num(String::from_utf8_lossy(text).into_owned()))
        }

        fn hex4(&mut self) -> Result<u32, String> {
            let h = self.s.get(self.i..self.i + 4).ok_or("bad \\u escape")?;
            self.i += 4;
            u32::from_str_radix(std::str::from_utf8(h).map_err(|e| e.to_string())?, 16)
                .map_err(|e| e.to_string())
        }

        fn string(&mut self) -> Result<String, String> {
            self.eat("\"")?;
            let mut out = Vec::new();
            loop {
                // Copy the run up to the next quote or backslash in one go.
                let run = self.s[self.i..]
                    .iter()
                    .position(|&b| b == b'"' || b == b'\\')
                    .ok_or("unterminated JSON string")?;
                out.extend_from_slice(&self.s[self.i..self.i + run]);
                self.i += run;
                let c = self.s[self.i];
                self.i += 1;
                if c == b'"' {
                    break;
                }
                let e = *self.s.get(self.i).ok_or("bad escape")?;
                self.i += 1;
                let ch = match e {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{8}',
                    b'f' => '\u{c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'u' => {
                        let hi = self.hex4()?;
                        let code = if (0xD800..0xDC00).contains(&hi) {
                            self.eat("\\u")?;
                            let lo = self.hex4()?;
                            0x10000 + ((hi - 0xD800) << 10) + (lo.wrapping_sub(0xDC00) & 0x3FF)
                        } else {
                            hi
                        };
                        char::from_u32(code).unwrap_or('\u{FFFD}')
                    }
                    _ => return Err("bad escape".into()),
                };
                let mut buf = [0u8; 4];
                out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
            }
            String::from_utf8(out).map_err(|e| e.to_string())
        }
    }

    // ---- Typed reading (the fast path) -------------------------------------------------
    // Test values are read straight into the solution's argument types, with no `Json`
    // tree in between. Whenever a value does not fit its type, the caller rewinds to the
    // start of the test and takes the generic path (`parse` + `FromJson`), so messages and
    // acceptance are exactly the generic path's: this is only a shortcut for the common case.

    pub enum ReadError {
        /// The value is valid JSON so far but does not fit the type: rewind, go generic.
        Mismatch,
        /// The request is not valid JSON.
        Syntax(String),
    }

    impl From<String> for ReadError {
        fn from(e: String) -> Self {
            ReadError::Syntax(e)
        }
    }

    impl From<&str> for ReadError {
        fn from(e: &str) -> Self {
            ReadError::Syntax(e.to_string())
        }
    }

    pub trait ReadJson: Sized {
        fn read(p: &mut Parser<'_>) -> Result<Self, ReadError>;
    }

    impl Parser<'_> {
        /// The next significant byte, after whitespace.
        fn peek(&mut self) -> Option<u8> {
            self.ws();
            self.s.get(self.i).copied()
        }

        /// Reads a number for a typed slot; anything else is a mismatch.
        fn typed_number(&mut self) -> Result<Json, ReadError> {
            match self.peek() {
                Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') => Ok(self.number()?),
                _ => Err(ReadError::Mismatch),
            }
        }
    }

    impl ReadJson for i64 {
        fn read(p: &mut Parser<'_>) -> Result<Self, ReadError> {
            match p.typed_number()? {
                Json::Int(n) => Ok(n),
                other => i64::from_json(&other).map_err(|_| ReadError::Mismatch),
            }
        }
    }

    impl ReadJson for f64 {
        fn read(p: &mut Parser<'_>) -> Result<Self, ReadError> {
            let v = p.typed_number()?;
            f64::from_json(&v).map_err(|_| ReadError::Mismatch)
        }
    }

    impl ReadJson for bool {
        fn read(p: &mut Parser<'_>) -> Result<Self, ReadError> {
            match p.peek() {
                Some(b't') => p.eat("true").map(|_| true).map_err(ReadError::Syntax),
                Some(b'f') => p.eat("false").map(|_| false).map_err(ReadError::Syntax),
                _ => Err(ReadError::Mismatch),
            }
        }
    }

    impl ReadJson for String {
        fn read(p: &mut Parser<'_>) -> Result<Self, ReadError> {
            match p.peek() {
                Some(b'"') => Ok(p.string()?),
                _ => Err(ReadError::Mismatch),
            }
        }
    }

    impl<T: ReadJson> ReadJson for Vec<T> {
        fn read(p: &mut Parser<'_>) -> Result<Self, ReadError> {
            if p.peek() != Some(b'[') {
                return Err(ReadError::Mismatch);
            }
            p.enter()?;
            p.i += 1;
            let mut items = Vec::new();
            if p.peek() == Some(b']') {
                p.i += 1;
            } else {
                loop {
                    items.push(T::read(p)?);
                    match p.peek() {
                        Some(b',') => p.i += 1,
                        Some(b']') => {
                            p.i += 1;
                            break;
                        }
                        _ => return Err(ReadError::Syntax(format!("bad JSON array at byte {}", p.i))),
                    }
                }
            }
            p.depth -= 1;
            Ok(items)
        }
    }

    /// Reads one test value as `T` with the fast path, falling back to the generic path on
    /// a mismatch. `Err` is a syntax error (the whole request is broken: exit 70); the inner
    /// result is the value or the generic path's message.
    pub fn read_value<T: ReadJson + FromJson>(p: &mut Parser<'_>) -> Result<Result<T, String>, String> {
        let (start, depth) = (p.i, p.depth);
        match T::read(p) {
            Ok(v) => Ok(Ok(v)),
            Err(ReadError::Syntax(e)) => Err(e),
            Err(ReadError::Mismatch) => {
                p.i = start;
                p.depth = depth;
                let v = p.value()?;
                Ok(T::from_json(&v))
            }
        }
    }

    pub fn parse(s: &str) -> Result<Json, String> {
        let mut p = Parser { s: s.as_bytes(), i: 0, depth: 0 };
        let v = p.value()?;
        p.ws();
        if p.i != p.s.len() {
            return Err("trailing characters after JSON".into());
        }
        Ok(v)
    }

    pub fn write_str(out: &mut String, s: &str) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }

    pub trait FromJson: Sized {
        fn from_json(v: &Json) -> Result<Self, String>;
    }

    impl FromJson for i64 {
        fn from_json(v: &Json) -> Result<Self, String> {
            match v {
                Json::Int(n) => Ok(*n),
                Json::Num(n) => n
                    .parse::<i64>()
                    .or_else(|_| {
                        n.parse::<f64>()
                            .ok()
                            .filter(|f| f.fract() == 0.0 && f.abs() < 9.2e18)
                            .map(|f| f as i64)
                            .ok_or(())
                    })
                    .map_err(|_| format!("{n} is not an i64")),
                _ => Err(format!("expected an integer, got {v:?}")),
            }
        }
    }

    impl FromJson for f64 {
        fn from_json(v: &Json) -> Result<Self, String> {
            match v {
                // Exact for every integer a test can hold (within ±(2^53 − 1)), and for
                // larger ones the same correctly rounded value `parse` gives.
                Json::Int(n) => Ok(*n as f64),
                Json::Num(n) => n.parse().map_err(|_| format!("{n} is not an f64")),
                _ => Err(format!("expected a number, got {v:?}")),
            }
        }
    }

    impl FromJson for bool {
        fn from_json(v: &Json) -> Result<Self, String> {
            match v {
                Json::Bool(b) => Ok(*b),
                _ => Err(format!("expected a bool, got {v:?}")),
            }
        }
    }

    impl FromJson for String {
        fn from_json(v: &Json) -> Result<Self, String> {
            match v {
                Json::Str(s) => Ok(s.clone()),
                _ => Err(format!("expected a string, got {v:?}")),
            }
        }
    }

    impl<T: FromJson> FromJson for Vec<T> {
        fn from_json(v: &Json) -> Result<Self, String> {
            match v {
                Json::Arr(items) => items.iter().map(T::from_json).collect(),
                _ => Err(format!("expected an array, got {v:?}")),
            }
        }
    }

    pub trait ToJson {
        /// Appends this value as JSON; fails for values JSON cannot represent.
        fn write_json(&self, out: &mut String) -> Result<(), String>;
    }

    impl ToJson for i64 {
        fn write_json(&self, out: &mut String) -> Result<(), String> {
            out.push_str(&self.to_string());
            Ok(())
        }
    }

    impl ToJson for f64 {
        fn write_json(&self, out: &mut String) -> Result<(), String> {
            if !self.is_finite() {
                return Err(format!("{self} is not a finite number"));
            }
            // Debug keeps a fractional part (2.0, not 2) and uses JSON-compatible exponents.
            out.push_str(&format!("{self:?}"));
            Ok(())
        }
    }

    impl ToJson for bool {
        fn write_json(&self, out: &mut String) -> Result<(), String> {
            out.push_str(if *self { "true" } else { "false" });
            Ok(())
        }
    }

    impl ToJson for String {
        fn write_json(&self, out: &mut String) -> Result<(), String> {
            write_str(out, self);
            Ok(())
        }
    }

    impl<T: ToJson> ToJson for Vec<T> {
        fn write_json(&self, out: &mut String) -> Result<(), String> {
            out.push('[');
            for (i, v) in self.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                v.write_json(out)?;
            }
            out.push(']');
            Ok(())
        }
    }
