use std::{option, vec};

#[derive(Clone, Debug, PartialEq)]
struct MtgCard {
    value: u8,
    play_prio_calc: fn(&GameState) -> i8,
    play_card_fn: fn(&mut GameState, MtgCard),
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
        let selected_card = self.hand.iter().max_by_key(|card| {(card.play_prio_calc)(self)}).unwrap().clone();
        if (selected_card.play_prio_calc)(self) > 0 {
            return Some(selected_card)
        }
        else {
            None
        }
    }
    fn play_selected_card(&mut self, selected_card: MtgCard) {
        (selected_card.play_card_fn)(self, selected_card);
    }
}

fn land_prio_calc(game_state: &GameState) -> i8 {
    if game_state.has_played_land {
        return 0
    }
    else {
        return 1
    }
}
fn play_land(game_state: &mut GameState, land_to_play: MtgCard) {
    let hand_index = game_state.hand.iter().position(|x| *x == land_to_play).expect("Card missing from hand");
    game_state.hand.swap_remove(hand_index);
    game_state.board.push(land_to_play);
    game_state.has_played_land = true;
}
fn main() {
    let mut game = GameState {
        deck: vec![
            MtgCard { value: 01, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 02, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 03, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 04, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 05, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 06, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 07, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 08, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 09, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 10, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 11, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 12, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 13, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 14, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 15, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 16, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 17, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 18, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 19, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 20, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 21, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 22, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 23, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 24, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 25, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 26, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 27, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 28, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 29, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 30, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 31, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 32, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 33, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 34, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 35, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 36, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 37, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 38, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 39, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 40, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 41, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 42, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 43, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 44, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 45, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 46, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 47, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 48, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 49, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 50, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 51, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 52, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 53, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 54, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 55, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 56, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 57, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 58, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 59, play_prio_calc: land_prio_calc, play_card_fn: play_land},
            MtgCard { value: 60, play_prio_calc: land_prio_calc, play_card_fn: play_land},
        ],
        hand: Vec::new(),
        board: Vec::new(),
        has_played_land: false
    };

    game.shuffle();
    game.draw(7);
    if let Some(selected_card) = game.play_selector(){
        game.play_selected_card(selected_card);
    }
    else {
        println!("No card to play")
    }

    println!("{:#?}", game);
}
