struct Card {
    rank: String,
    suit: String,
}

struct FrenchDeck {
    ranks: Vec<String>,
    suits: Vec<String>,
    cards: Vec<Card>,
}

impl FrenchDeck {
    fn new() -> FrenchDeck {
        let ranks_list = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"].map(String::from).to_vec();
        let suits_list = ["♣", "♠", "♥", "♦"].map(String::from).to_vec();
        let mut cards_list = vec![];
        for each_suit in &suits_list {
            for each_rank in &ranks_list {
                cards_list.push(Card {
                    rank: each_rank.to_string(),
                    suit: each_suit.to_string(),
                });
            }
        }
        FrenchDeck {
            ranks: ranks_list,
            suits: suits_list,
            cards: cards_list,
        }
    }
}

fn main() {
    let a_deck = FrenchDeck::new();
    let a_len = a_deck.cards.len();
    println!("Length of deck: {}", a_len);
}