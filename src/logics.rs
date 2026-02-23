//TODO 
//
//Remove cloning from here

use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Card {
    suit: CardSuit,
    value: CardValue,
}

impl Card {
    fn new(suit : CardSuit, value : CardValue) -> Self {
        Self {
            suit, 
            value, 
        }
    }

    pub fn can_beat(&self, compare_to : &Card, trump_suit : &CardSuit) -> bool {
        if self.is_trump_suit(trump_suit) {
            if compare_to.is_trump_suit(trump_suit) {
                return self.has_higher_weight(compare_to);
            }

            if !compare_to.is_trump_suit(trump_suit) {
                return true;
            }
        }

        if !self.is_trump_suit(trump_suit) {
            if self.is_same_suit(compare_to) {
                return self.has_higher_weight(compare_to);
            }
            if !self.is_same_suit(compare_to) {
                return false;
            }
        }
        false
    }

    fn has_higher_weight(&self, compare_to : &Card) -> bool {
        let card_weight : HashMap<CardValue, u8> = get_card_weight();

        card_weight.get(&self.value).copied().unwrap() > card_weight.get(&compare_to.value).copied().unwrap()

    }

    fn is_trump_suit(&self, trump_suit : &CardSuit) -> bool {
        self.suit == *trump_suit
    }


    fn is_same_suit(&self, compare_to : &Card) -> bool {
        self.suit == compare_to.suit
    }
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

fn get_card_weight() -> HashMap<CardValue, u8> {
    let mut card_weight : HashMap<CardValue, u8> = HashMap::new();

    let mut weight : u8 = 1;
    for card_value in CARD_VALUES {
        card_weight.insert(card_value.clone(), weight);
        weight += 1;
    }

    card_weight
}

pub fn get_card_deck() -> Vec<Card> {
    let mut card_deck : Vec<Card> = Vec::new();

    for card_suit in CARD_SUITS {
        for card_value in CARD_VALUES {
            card_deck.push(Card::new(card_suit.clone(), card_value.clone()));
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

pub fn get_random_trump_suit() -> CardSuit {
    let mut rng = rand::rng();

    CARD_SUITS[rng.random_range(0..=3)].clone()
}
