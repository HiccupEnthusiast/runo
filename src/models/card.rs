use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub kind: CardKind,
    pub color: CardColor,
    pub number: Option<u8>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardKind {
    Normal,
    Draw(u8),
    ChangeColor(CardColor),
    Reverse,
    Skip,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardColor {
    Red,
    Blue,
    Green,
    Yellow,
    Black,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:?}{}card",
            match self.kind {
                CardKind::Normal => "".to_string(),
                CardKind::Draw(i) => format!("+{} ", i),
                CardKind::ChangeColor(_) => "Wild ".to_string(),
                CardKind::Reverse => "Reverse ".to_string(),
                CardKind::Skip => "Skip ".to_string(),
            },
            self.color,
            match self.number {
                Some(i) => format!(" {} ", i),
                None => " ".to_string(),
            }
        )
    }
}
