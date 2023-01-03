use std::{fmt::Display, io::Write, num::IntErrorKind};

use rand::prelude::*;

mod models;
use crate::models::{create_deck, Card, CardColor, CardKind, Deck};

#[derive(Debug, Clone)]
struct Player {
    name: String,
    deck: Deck,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.deck)
    }
}

fn take_cards(from: &mut Deck, number: usize) -> Deck {
    let new_deck: Vec<Card>;
    let size = from.len();

    new_deck = from.drain(size - number..).collect();

    Deck(new_deck)
}
fn check_playability(last_card: &Card, new_card: &Card) -> bool {
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

fn main() {
    let mut deck = crate::models::create_deck();
    deck.shuffle(&mut thread_rng());

    let commands = "[D]eck, [P]layed card, Draw [C]ard, [H]elp, [E]xit";

    println!("Enter your nickname: ");
    let mut pname = String::new();
    std::io::stdin()
        .read_line(&mut pname)
        .expect("Error while reading the line");

    let mut player = Player {
        name: pname.trim().to_string(),
        deck: take_cards(&mut deck, 7),
    };
    let mut pc1 = Player {
        name: "Alice".to_string(),
        deck: take_cards(&mut deck, 7),
    };

    println!("Hi {}! Your deck is: {}", player.name, player.deck);
    println!("In order to play you need to put the number of the card in your deck, you have to match the number, color, symbol, or play a black card.");
    println!(
        "You can also get some information about the state of the game using the following: {}",
        commands
    );

    let mut last_card = take_cards(&mut deck, 1)[0];
    println!("First card: {}", last_card);

    player.deck.push(Card {
        kind: CardKind::ChangeColor(CardColor::Black),
        color: CardColor::Black,
        number: None,
    });
    loop {
        print!("{}: ", player.name);
        std::io::stdout()
            .flush()
            .expect("Error while flushing stdout");
        let mut user_input = String::new();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Error while reading the line");

        let user_input: usize = match user_input.trim().parse() {
            Ok(i) => {
                if i == 0 {
                    1
                } else {
                    i
                }
            }
            Err(err) => match err.kind() {
                IntErrorKind::InvalidDigit => {
                    match user_input.trim().to_lowercase().as_str() {
                        "d" => println!("Your deck is: {}", player.deck),
                        "p" => println!("The last played card is: {}", last_card),
                        "c" => {
                            let card = take_cards(&mut deck, 1)[0];
                            player.deck.push(card);
                            println!("{} drawn!", card);
                        }
                        "h" => println!("{}", commands),
                        "e" => break,
                        _ => println!("Please input a valid number or command"),
                    }
                    continue;
                }
                _ => continue,
            },
        };
        let player_card = match player.deck.get(user_input - 1) {
            Some(c) => c,
            None => {
                println!("You don't have that many cards! Please check your [d]eck");
                continue;
            }
        };
        if !check_playability(&last_card, &player_card) {
            println!("{} can't be played on {}", player_card, last_card);
            continue;
        }

        println!("{} plays {}", player.name, &player_card);

        last_card = *player_card;

        match player_card.kind {
            CardKind::Normal => (),
            CardKind::Draw(i) => pc1.deck.append(&mut take_cards(&mut deck, i as usize)),
            CardKind::ChangeColor(_) => {
                println!("What color would you like to use now? (Red, Blue, Green, Yellow)");
                let mut pick = String::new();
                std::io::stdin()
                    .read_line(&mut pick)
                    .expect("An error has occured while parsing your pick");

                last_card.color = match pick.trim().to_lowercase().as_str() {
                    "red" => CardColor::Red,
                    "blue" => CardColor::Blue,
                    "green" => CardColor::Green,
                    "yellow" => CardColor::Yellow,
                    _ => {
                        println!("{} is not a valid color", pick.trim());
                        continue;
                    }
                };
                println!("Color has been set to {:?}", last_card.color);
            }
            _ => println!("Functionality for this kind of card has not been set yet."),
        }

        player.deck.remove(user_input - 1);

        if player.deck.len() == 0 {
            println!("You have won, congratulations!");
            break;
        }

        let mut has_to_borrow = false;
        for (i, card) in pc1.deck.iter().enumerate() {
            if check_playability(&last_card, &card) {
                match card.kind {
                    CardKind::Normal => (),
                    CardKind::Draw(i) => player.deck.append(&mut take_cards(&mut deck, i as usize)),
                    CardKind::ChangeColor(_) => {
                        let pick = "red";
                        println!("Yppie");
                        last_card.color = match pick.trim().to_lowercase().as_str() {
                            "red" => CardColor::Red,
                            "blue" => CardColor::Blue,
                            "green" => CardColor::Green,
                            "yellow" => CardColor::Yellow,
                            _ => {
                                println!("{} is not a valid color", pick.trim());
                                continue;
                            }
                        };
                        println!("Color has been set to {:?}", last_card.color);
                    }
                    _ => println!("Functionality for this kind of card has not been set yet."),
                }
                println!("({}){} plays {}", pc1.deck.len() - 1, pc1.name, card);
                last_card = *card;
                pc1.deck.remove(i);
                has_to_borrow = false;
                break;
            } else {
                has_to_borrow = true;
            }
        }
        if has_to_borrow {
            pc1.deck.push(take_cards(&mut deck, 1)[0]);
            println!("({}){} draws a card", pc1.deck.len(), pc1.name);
        }

        last_card.color = match last_card.color {
            CardColor::Black => {
                println!("Wild cards are not fully implemented yet, fallback to red.");
                CardColor::Red
            }
            color => color,
        };
        if pc1.deck.len() == 0 {
            println!("You have lost, {} is the winner.", pc1.name);
            break;
        }
        if deck.len() <= 5 {
            println!("One deck has been depleted, reshufling a new one");
            deck = create_deck()
        }
    }
}
