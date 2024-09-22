pub mod changsha_mj;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;
#[derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Wan(u8),
    Tiao(u8),
    Tong(u8),
    Zhong,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Wan(n) => write!(f, "{}万", n%10),
            Card::Tiao(n) => write!(f, "{}条", n % 10),
            Card::Tong(n) => write!(f, "{}筒", n % 10),
            Card::Zhong => write!(f, "中"),
        }
    }
}
// 实现 Debug 时重用 Display 的实现
impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)  // 调用 Display 的实现
    }
}

pub trait ToCard {
    fn to_card(&self) -> Card;
}

impl ToCard for u8 {
    fn to_card(&self) -> Card {
        match self {
            11..=19 => Card::Wan(*self),
            21..=29 => Card::Tiao(*self),
            31..=39 => Card::Tong(*self),
            41 => Card::Zhong,
            _ => panic!("Invalid card number: {}", self),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionType {
    Chi = 0,
    Peng = 1,
    Gang = 2,
}

#[derive(Debug, Clone)]
pub struct ActionCard {
    pub action_type: ActionType,
    /// The cards involved in the action. first card is the one being acted on,
    /// and the rest are the cards involved in the action.
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub struct SeatCard {
    pub seat_id: u8,
    pub hand_cards: Vec<Card>,
    pub action_cards: Vec<ActionCard>,
    pub out_cards: Vec<Card>,
}

impl SeatCard {
    pub fn new(seat_id: u8) -> Self {
        Self {
            seat_id,
            hand_cards: vec![],
            action_cards: vec![],
            out_cards: vec![],
        }
    }
}

pub struct Game {
    pub initial_total_cards: Vec<u8>,
    pub remain_cards: Vec<Card>,
    pub seats: [SeatCard; 4],
    pub cur_op_seat: usize,
}

impl Game {
    /// 洗牌
    pub fn shuffle_cards(&mut self) {
        self.initial_total_cards.shuffle(&mut rand::thread_rng());
        self.remain_cards = self.initial_total_cards.iter().map(|&c| c.to_card()).collect();
    }

    /// 摇骰子
    pub fn roll_dice(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=12)
    }

    /// 发牌
    pub fn deal_cards(&mut self, first_seat: usize) {
        for i in 0..4 {
            // 从 remain_cards 中取出前 13 张牌
            let cards = self.remain_cards.split_off(self.remain_cards.len() - 13);
            self.seats[i].hand_cards = cards; // 发给当前座位
        }
        // 多发一张牌
        self.seats[first_seat].hand_cards.push(self.remain_cards.pop().unwrap());
        self.cur_op_seat = first_seat;
        // sort hand cards
        for s in &mut self.seats {
            s.hand_cards.sort();
        }

        println!("Initial hand cards: {:#?}", self.seats)
    }
}
