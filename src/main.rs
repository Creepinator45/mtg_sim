use std::vec;

#[derive(Clone, Debug)]
struct MtgCard {
    value: u8,
}

#[derive(Debug)]
struct GameState {
    deck: Vec<MtgCard>,
    hand: Vec<MtgCard>,
    board: Vec<MtgCard>
}
impl GameState {
    fn draw(&mut self, num_cards: u8) {
        for _ in 0..num_cards {
            if let Some(value) = self.deck.last() {
                self.hand.push(value.clone());
            } else {
                todo!("deck out loses game")
            }
            self.deck.pop();
        }
    }
    fn shuffle(&mut self) {
        fastrand::shuffle(&mut self.deck)
    }

}
fn main() {
    let mut game = GameState {
        deck: vec![
            MtgCard { value: 1 },
            MtgCard { value: 2 },
            MtgCard { value: 3 },
            MtgCard { value: 4 },
            MtgCard { value: 5 },
            MtgCard { value: 6 },
            MtgCard { value: 7 },
            MtgCard { value: 8 },
            MtgCard { value: 9 },
            MtgCard { value: 10 },
            MtgCard { value: 11 },
            MtgCard { value: 12 },
            MtgCard { value: 13 },
            MtgCard { value: 14 },
            MtgCard { value: 15 },
            MtgCard { value: 16 },
            MtgCard { value: 17 },
            MtgCard { value: 18 },
            MtgCard { value: 19 },
            MtgCard { value: 20 },
            MtgCard { value: 21 },
            MtgCard { value: 22 },
            MtgCard { value: 23 },
            MtgCard { value: 24 },
            MtgCard { value: 25 },
            MtgCard { value: 26 },
            MtgCard { value: 27 },
            MtgCard { value: 28 },
            MtgCard { value: 29 },
            MtgCard { value: 30 },
            MtgCard { value: 31 },
            MtgCard { value: 32 },
            MtgCard { value: 33 },
            MtgCard { value: 34 },
            MtgCard { value: 35 },
            MtgCard { value: 36 },
            MtgCard { value: 37 },
            MtgCard { value: 38 },
            MtgCard { value: 39 },
            MtgCard { value: 40 },
            MtgCard { value: 41 },
            MtgCard { value: 42 },
            MtgCard { value: 43 },
            MtgCard { value: 44 },
            MtgCard { value: 45 },
            MtgCard { value: 46 },
            MtgCard { value: 47 },
            MtgCard { value: 48 },
            MtgCard { value: 49 },
            MtgCard { value: 50 },
            MtgCard { value: 51 },
            MtgCard { value: 52 },
            MtgCard { value: 53 },
            MtgCard { value: 54 },
            MtgCard { value: 55 },
            MtgCard { value: 56 },
            MtgCard { value: 57 },
            MtgCard { value: 58 },
            MtgCard { value: 59 },
            MtgCard { value: 60 },
        ],
        hand: Vec::new(),
        board: Vec::new(),
    };

    game.shuffle();
    game.draw(7);
    
    println!("{:?}", game)
}
