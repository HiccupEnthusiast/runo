use colored::Colorize;
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

impl Card {
    pub fn check_playability(last_card: &Card, new_card: &Card) -> bool {
        if new_card.color == last_card.color || new_card.color == CardColor::Black {
            true
        } else if new_card.number == last_card.number {
            match new_card.number {
                Some(_) => true,
                None => {
                    if new_card.kind == last_card.kind {
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let number = match self.number {
            Some(i) => format!(" {} ", i).cyan(),
            None => " ".white(),
        };
        let base = format!(
            "{}{}{}card",
            match self.kind {
                CardKind::Normal => "".to_string(),
                CardKind::Draw(i) => format!("+{} ", i),
                CardKind::ChangeColor(_) => "Wild ".to_string(),
                CardKind::Reverse => "Reverse ".to_string(),
                CardKind::Skip => "Skip ".to_string(),
            },
            self.color,
            number,
        );
        let base = match self.color {
            CardColor::Red => base.red(),
            CardColor::Blue => base.blue(),
            CardColor::Green => base.green(),
            CardColor::Yellow => base.yellow(),
            CardColor::Black => base.white(),
        };

        write!(f, "{}", base)

        // write!(
        // f,
        // "{}{}{}card",
        // match self.kind {
        // CardKind::Normal => "".to_string(),
        // CardKind::Draw(i) => format!("+{} ", i),
        // CardKind::ChangeColor(_) => "Wild ".to_string(),
        // CardKind::Reverse => "Reverse ".to_string(),
        // CardKind::Skip => "Skip ".to_string(),
        // },
        // self.color,
        // match self.number {
        // Some(i) => format!(" {} ", i).cyan(),
        // None => " ".white(),
        // },
        // )
    }
}
impl Display for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Red => "Red".red(),
            Self::Blue => "Blue".blue(),
            Self::Green => "Green".green(),
            Self::Yellow => "Yellow".yellow(),
            Self::Black => "Black".white(),
        };
        write!(f, "{}", out)
    }
}
