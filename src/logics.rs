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

static card_suits : [CardSuit; 4] = [CardSuit::Hearts,
                                 CardSuit::Diamonds,
                                 CardSuit::Spades,
                                 CardSuit::Clubs];

static card_values : [CardValue; 9] = [CardValue::Six,
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

    for card_suit in card_suits.clone() {
        for card_value in card_values.clone() {
            cards.push(Card {
                suit : card_suit.clone(),
                value : card_value.clone(),
            });
        } 
    }
    cards
}
