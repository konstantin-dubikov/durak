mod logics;

use logics::Card;

fn main() {

    let cards : Vec<Card> = logics::get_random_card_deck();

    for card in cards {
        println!("{:?}", card);
    }

}

