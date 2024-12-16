
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum XMASLetter {
    X, M, A, S
}

impl XMASLetter {
    pub fn from(value: char) -> Option<XMASLetter> {
        match value {
            'X' => Some(XMASLetter::X),
            'M' => Some(XMASLetter::M),
            'A' => Some(XMASLetter::A),
            'S' => Some(XMASLetter::S),
            _ => None,
        }
    }
}

impl std::fmt::Display for XMASLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            XMASLetter::X => 'X',
            XMASLetter::M => 'M',
            XMASLetter::A => 'A',
            XMASLetter::S => 'S',
        })
    }
}
