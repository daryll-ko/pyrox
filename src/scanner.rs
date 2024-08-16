pub struct Scanner<'a> {
    pub code: &'a str,
}

impl Scanner<'_> {
    pub fn new(code: &str) -> Scanner {
        Scanner { code }
    }
}
