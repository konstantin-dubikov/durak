mod logics;

use logics::{
    Card,
    CardSuit,
};

fn main() {

    let cards : Vec<Card> = logics::get_random_card_deck();
    let trump_suit : logics::CardSuit = logics::get_trump_suit(); 
    
    for card in &cards {
        println!("{:?}", card);
    }

    println!("The trump suit is: {:?}", trump_suit);

    {
    println!("{}", cards[34].can_beat(cards[35].clone(), trump_suit));
    }
}

