mod logics;

use logics::Card;

fn main() {

    let cards : Vec<Card> = logics::get_cards();

    for card in cards {
        println!("{:?}", card);
    }

}

