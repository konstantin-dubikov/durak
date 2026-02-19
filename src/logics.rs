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

pub fn get_cards() -> Vec<Card> {
    let mut cards : Vec<Card> = Vec::new();

    for card_suit in CARD_SUITS {
        for card_value in CARD_VALUES {
            cards.push(Card {
                suit : card_suit.clone(),
                value : card_value.clone(),
            });
        } 
    }
    cards
}
