use std::vec;

#[derive(Clone, Debug, PartialEq)]
struct BaseCard {
    value: u8,
}
#[derive(Clone, Debug, PartialEq)]
struct BasicLand {
    color: Color,
}

#[derive(Clone, Debug, PartialEq)]
enum Color {
    W,
    U,
    B,
    R,
    G
}
#[derive(Clone, Debug, PartialEq)]
enum MtgCard {
    BaseCard(BaseCard),
    BasicLand(BasicLand),
}
#[derive(Debug)]
struct GameState {
    deck: Vec<MtgCard>,
    hand: Vec<MtgCard>,
    board: Vec<MtgCard>,
    has_played_land: bool,
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
    fn play_selector(&self) -> Option<MtgCard> {
        let selected_card = self
            .hand
            .iter()
            .max_by_key(|card| card.play_prio_calc(self))
            .unwrap()
            .clone();
        if selected_card.play_prio_calc(self) > 0 {
            return Some(selected_card);
        } else {
            None
        }
    }
    fn play_selected_card(&mut self, selected_card: MtgCard) {
        let hand_index = self
                    .hand
                    .iter()
                    .position(|x| *x == selected_card)
                    .expect("Card missing from hand");
        selected_card.play_card_fn(self, hand_index);
    }
}

impl MtgCard {
    fn play_prio_calc(&self, game_state: &GameState) -> i8 {
        match self {
            MtgCard::BaseCard(card) => {
                if card.value >= 30 {
                    return 1
                }
                else {
                    return 0
                }
            }
            MtgCard::BasicLand(_) => {
                if game_state.has_played_land {
                    return 0
                }
                else {
                    return 127
                }
            }
        }
    }

    fn play_card_fn(&self, game_state: &mut GameState, hand_index: usize) {
        if game_state.hand[hand_index] != *self {
            panic!("Card not at expected position in hand")
        }
        
        match self {
            MtgCard::BaseCard(_) => {
                game_state.hand.swap_remove(hand_index);
                game_state.board.push(self.clone());
            }
            MtgCard::BasicLand(card) => {
                game_state.hand.swap_remove(hand_index);
                game_state.board.push(self.clone());
                game_state.has_played_land = true;
            }
        }
    }
}
fn main() {
    let mut game = GameState {
        deck: vec![
            MtgCard::BaseCard(BaseCard { value: 01 }),
            MtgCard::BaseCard(BaseCard { value: 02 }),
            MtgCard::BaseCard(BaseCard { value: 03 }),
            MtgCard::BaseCard(BaseCard { value: 04 }),
            MtgCard::BaseCard(BaseCard { value: 05 }),
            MtgCard::BaseCard(BaseCard { value: 06 }),
            MtgCard::BaseCard(BaseCard { value: 07 }),
            MtgCard::BaseCard(BaseCard { value: 08 }),
            MtgCard::BaseCard(BaseCard { value: 09 }),
            MtgCard::BaseCard(BaseCard { value: 10 }),
            MtgCard::BaseCard(BaseCard { value: 11 }),
            MtgCard::BaseCard(BaseCard { value: 12 }),
            MtgCard::BaseCard(BaseCard { value: 13 }),
            MtgCard::BaseCard(BaseCard { value: 14 }),
            MtgCard::BaseCard(BaseCard { value: 15 }),
            MtgCard::BaseCard(BaseCard { value: 16 }),
            MtgCard::BaseCard(BaseCard { value: 17 }),
            MtgCard::BaseCard(BaseCard { value: 18 }),
            MtgCard::BaseCard(BaseCard { value: 19 }),
            MtgCard::BaseCard(BaseCard { value: 20 }),
            MtgCard::BaseCard(BaseCard { value: 21 }),
            MtgCard::BaseCard(BaseCard { value: 22 }),
            MtgCard::BaseCard(BaseCard { value: 23 }),
            MtgCard::BaseCard(BaseCard { value: 24 }),
            MtgCard::BaseCard(BaseCard { value: 25 }),
            MtgCard::BaseCard(BaseCard { value: 26 }),
            MtgCard::BaseCard(BaseCard { value: 27 }),
            MtgCard::BaseCard(BaseCard { value: 28 }),
            MtgCard::BaseCard(BaseCard { value: 29 }),
            MtgCard::BaseCard(BaseCard { value: 30 }),
            MtgCard::BaseCard(BaseCard { value: 31 }),
            MtgCard::BaseCard(BaseCard { value: 32 }),
            MtgCard::BaseCard(BaseCard { value: 33 }),
            MtgCard::BaseCard(BaseCard { value: 34 }),
            MtgCard::BaseCard(BaseCard { value: 35 }),
            MtgCard::BaseCard(BaseCard { value: 36 }),
            MtgCard::BaseCard(BaseCard { value: 37 }),
            MtgCard::BaseCard(BaseCard { value: 38 }),
            MtgCard::BaseCard(BaseCard { value: 39 }),
            MtgCard::BaseCard(BaseCard { value: 40 }),
            MtgCard::BaseCard(BaseCard { value: 41 }),
            MtgCard::BaseCard(BaseCard { value: 42 }),
            MtgCard::BaseCard(BaseCard { value: 43 }),
            MtgCard::BaseCard(BaseCard { value: 44 }),
            MtgCard::BaseCard(BaseCard { value: 45 }),
            MtgCard::BaseCard(BaseCard { value: 46 }),
            MtgCard::BaseCard(BaseCard { value: 47 }),
            MtgCard::BaseCard(BaseCard { value: 48 }),
            MtgCard::BaseCard(BaseCard { value: 49 }),
            MtgCard::BaseCard(BaseCard { value: 50 }),
            MtgCard::BaseCard(BaseCard { value: 51 }),
            MtgCard::BaseCard(BaseCard { value: 52 }),
            MtgCard::BaseCard(BaseCard { value: 53 }),
            MtgCard::BaseCard(BaseCard { value: 54 }),
            MtgCard::BaseCard(BaseCard { value: 55 }),
            MtgCard::BaseCard(BaseCard { value: 56 }),
            MtgCard::BaseCard(BaseCard { value: 57 }),
            MtgCard::BaseCard(BaseCard { value: 58 }),
            MtgCard::BaseCard(BaseCard { value: 59 }),
            MtgCard::BaseCard(BaseCard { value: 60 }),
        ],
        hand: Vec::new(),
        board: Vec::new(),
        has_played_land: false,
    };

    game.shuffle();
    game.draw(30);

    for _ in 0..30 {
        if let Some(selected_card) = game.play_selector() {
            println!("{:?}", selected_card);
            game.play_selected_card(selected_card);
        } else {
            println!("No card to play")
        }
    }

    println!("{:#?}", game);
}
