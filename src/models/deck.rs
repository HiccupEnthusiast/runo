use crate::{Card, CardColor, CardKind};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Deck(pub Vec<Card>);

pub fn create_deck() -> Deck {
    let mut created_card: Card;
    let mut deck = vec![];

    for n_color in 0..=3 {
        let mut kind: CardKind = CardKind::Normal;
        let color: CardColor;
        match n_color {
            0 => color = CardColor::Red,
            1 => color = CardColor::Blue,
            2 => color = CardColor::Green,
            3 => color = CardColor::Yellow,
            _ => color = CardColor::Black,
        };
        for number in 0..=9 {
            created_card = Card {
                kind,
                color,
                number: Some(number),
            };
            deck.push(created_card);
            deck.push(created_card);
        }
        kind = CardKind::Reverse;
        created_card = Card {
            kind,
            color,
            number: None,
        };
        deck.push(created_card);
        deck.push(created_card);
        created_card.kind = CardKind::Draw(2);
        deck.push(created_card);
        deck.push(created_card);
        created_card.kind = CardKind::Skip;
        deck.push(created_card);
        deck.push(created_card);
    }
    created_card = Card {
        color: CardColor::Black,
        kind: CardKind::ChangeColor(CardColor::Black),
        number: None,
    };
    for _ in 0..4 {
        created_card.kind = CardKind::ChangeColor(CardColor::Black);
        deck.push(created_card);
        created_card.kind = CardKind::Draw(4);
        deck.push(created_card)
    }

    Deck(deck)
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc: u8 = 0;
        self.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| {
                acc += 1;
                write!(f, "[{}]{}, ", acc, card)
            })
        })
    }
}
impl std::ops::Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
