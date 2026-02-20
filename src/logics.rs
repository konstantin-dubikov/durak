use rand::prelude::*;

#[derive(Debug, Clone)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Debug, Clone)]
pub enum CardValue {
    Six, 
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Debug)]
pub struct Card {
    suit: CardSuit,
    value: CardValue,
}

const CARD_SUITS : [CardSuit; 4] = [CardSuit::Hearts,
                                 CardSuit::Diamonds,
                                 CardSuit::Spades,
                                 CardSuit::Clubs];

const CARD_VALUES : [CardValue; 9] = [CardValue::Six,
                                   CardValue::Seven,
                                   CardValue::Eight,
                                   CardValue::Nine,
                                   CardValue::Ten,
                                   CardValue::Jack,
                                   CardValue::Queen,
                                   CardValue::King,
                                   CardValue::Ace];

pub fn get_card_deck() -> Vec<Card> {
    let mut card_deck : Vec<Card> = Vec::new();

    for card_suit in CARD_SUITS {
        for card_value in CARD_VALUES {
            card_deck.push(Card {
                suit : card_suit.clone(),
                value : card_value.clone(),
            });
        } 
    }
    card_deck
}

pub fn get_random_card_deck() -> Vec<Card> {
    let mut random_card_deck : Vec<Card> = get_card_deck();
    let mut rng = rand::rng();
    
    random_card_deck.shuffle(&mut rng);

    random_card_deck
}

pub fn get_trump_suit() -> CardSuit {
    let mut rng = rand::rng();

    CARD_SUITS[rng.random_range(0..=3)].clone()
}
