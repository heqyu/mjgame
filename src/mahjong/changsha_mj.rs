use super::*;

pub struct ChangshaMj {
    pub game: Game,
}

/// All cards in Changsha Mahjong
static ALL_CARDS: &[u8] = &[
    11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 33, 34, 35, 36, 37, 38, 39, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 33, 34, 35, 36, 37, 38, 39, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28,
    29, 31, 32, 33, 34, 35, 36, 37, 38, 39,
];

impl ChangshaMj {
    pub fn new() -> Self {
        Self {
            game: Game {
                initial_total_cards: ALL_CARDS.to_vec(),
                remain_cards: ALL_CARDS.iter().map(|&x| x.to_card()).collect(),
                seats: [SeatCard::new(0), SeatCard::new(1), SeatCard::new(2), SeatCard::new(3)],
                cur_op_seat: 0,
            },
        }
    }

    pub fn start_game(&mut self) {
        self.game.shuffle_cards();
        let roll= self.game.roll_dice();
        self.game.deal_cards(roll % 4);
    }
}
