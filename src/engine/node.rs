#[derive(Debug, Clone)]
pub struct Node {
    pub board: Board,     // table
    pub seats: Vec<Seat>, // rotation
    pub pot: u32,         // table
    pub dealer: usize,    // rotation
    pub counter: usize,   // rotation
    pub pointer: usize,   // rotation.has_next == node.does_end_street
} // this data struct reads like a poem

impl Node {
    pub fn new(seats: Vec<Seat>) -> Self {
        Node {
            seats,
            board: Board::new(),
            pot: 0,
            dealer: 0,
            counter: 0,
            pointer: 0,
        }
    }

    pub fn has_more_hands(&self) -> bool {
        self.seats.iter().filter(|s| s.stack > 2).count() == 4
    }
    pub fn has_more_streets(&self) -> bool {
        !(self.are_all_folded() || (!self.has_more_players() && self.board.street == Street::River))
    }
    pub fn has_more_players(&self) -> bool {
        !(self.are_all_folded() || self.are_all_called() || self.are_all_shoved())
    }

    pub fn next(&self) -> &Seat {
        self.seats.get(self.pointer).unwrap()
    }
    pub fn seat(&self, id: usize) -> &Seat {
        self.seats.iter().find(|s| s.index == id).unwrap()
    }
    pub fn after(&self, index: usize) -> usize {
        (index + 1) % self.seats.len()
    }

    pub fn table_stack(&self) -> u32 {
        let mut totals = self
            .seats
            .iter()
            .map(|s| s.stack + s.stake)
            .collect::<Vec<u32>>();
        totals.sort();
        totals.pop().unwrap_or(0);
        totals.pop().unwrap_or(0)
    }
    pub fn table_stake(&self) -> u32 {
        self.seats.iter().map(|s| s.stake).max().unwrap()
    }

    pub fn are_all_folded(&self) -> bool {
        // exactly one player has not folded
        self.seats
            .iter()
            .filter(|s| s.status != BetStatus::Folded)
            .count()
            == 1
    }
    pub fn are_all_shoved(&self) -> bool {
        // everyone who isn't folded is all in
        self.seats
            .iter()
            .filter(|s| s.status != BetStatus::Folded)
            .all(|s| s.status == BetStatus::Shoved)
    }
    pub fn are_all_called(&self) -> bool {
        // everyone who isn't folded has matched the bet
        // or all but one player is all in
        let stakes = self.table_stake();
        let is_first_decision = self.counter == 0;
        let is_one_playing = self
            .seats
            .iter()
            .filter(|s| s.status == BetStatus::Playing)
            .count()
            == 1;
        let has_no_decision = is_first_decision && is_one_playing;
        let has_all_decided = self.counter > self.seats.len();
        let has_all_matched = self
            .seats
            .iter()
            .filter(|s| s.status == BetStatus::Playing)
            .all(|s| s.stake == stakes);
        (has_all_decided || has_no_decision) && has_all_matched
    }
}

// mutables
impl Node {
    pub fn add(&mut self, seat: Seat) {
        self.seats.push(seat);
    }
    pub fn apply(&mut self, action: Action) {
        let seat = self.seats.get_mut(self.pointer).unwrap();
        // bets entail pot and stack change
        match action {
            Action::Call(_, bet)
            | Action::Blind(_, bet)
            | Action::Raise(_, bet)
            | Action::Shove(_, bet) => {
                self.pot += bet;
                seat.stake += bet;
                seat.stack -= bet;
            }
            _ => (),
        }
        // folds and all-ins entail status change
        match action {
            Action::Fold(..) => seat.status = BetStatus::Folded,
            Action::Shove(..) => seat.status = BetStatus::Shoved,
            _ => (),
        }
        // player actions entail rotation
        match action {
            Action::Draw(card) => self.board.push(card.clone()),
            _ => self.rotate(),
        }
    }
    pub fn start_hand(&mut self) {
        self.pot = 0;
        self.board.cards.clear();
        self.board.street = Street::Pre;
        self.counter = 0;
        self.dealer = self.after(self.dealer);
        self.pointer = self.dealer;
        self.rotate();
    }
    pub fn start_street(&mut self) {
        self.counter = 0;
        self.pointer = match self.board.street {
            Street::Pre => self.after(self.after(self.dealer)),
            _ => self.dealer,
        };
        self.rotate();
    }
    pub fn end_street(&mut self) {
        for seat in self.seats.iter_mut() {
            seat.stake = 0;
        }
    }
    fn rotate(&mut self) {
        'left: loop {
            if !self.has_more_players() {
                return;
            }
            self.counter += 1;
            self.pointer = self.after(self.pointer);
            match self.next().status {
                BetStatus::Playing => return,
                BetStatus::Folded | BetStatus::Shoved => continue 'left,
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Pot:   {}\n", self.pot)?;
        write!(f, "Board: {}", self.board)?;
        for seat in &self.seats {
            write!(f, "{}", seat)?;
        }
        write!(f, "")
    }
}

use super::{
    action::Action,
    seat::{BetStatus, Seat},
};
use crate::cards::board::{Board, Street};
use std::fmt::{Display, Formatter, Result};
