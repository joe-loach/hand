pub struct Cursor<'t> {
    text: &'t str,
    chars: std::iter::Peekable<std::str::CharIndices<'t>>,
    pos: usize,
}

impl<'t> Cursor<'t> {
    #[inline]
    pub(crate) fn new(text: &'t str) -> Self {
        assert!(!text.is_empty());

        Cursor {
            text,
            chars: text.char_indices().peekable(),
            pos: 0_usize,
        }
    }

    #[inline]
    pub(crate) fn finish(self) -> (usize, &'t str) {
        let pos = self.pos + 1;
        (pos, &self.text[pos..])
    }

    #[inline]
    pub fn eat(&mut self) -> Option<char> {
        self.eat_inner().map(|(_pos, char)| char)
    }

    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.peek_inner().map(|(_pos, char)| char)
    }

    pub fn eat_while(&mut self, pred: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if pred(c) {
                self.eat();
            } else {
                break;
            }
        }
    }

    fn eat_inner(&mut self) -> Option<(usize, char)> {
        let Some((pos, c)) = self.chars.next() else {
            return None;
        };
        self.pos = pos;
        Some((pos, c))
    }

    fn peek_inner(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }
}
