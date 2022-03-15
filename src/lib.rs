#![no_std]

#[cfg(test)]
mod tests;

use codec::{Decode, Encode};
use gstd::{exec, msg, prelude::*, ActorId};
use lt_io::*;
use scale_info::TypeInfo;
use sp_core::hashing::blake2_256;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
struct Lottery {
    lottery_owner: ActorId,                  //Lottery owner
    players: BTreeMap<u32, Player>,          //Players
    lottery_history: BTreeMap<u32, ActorId>, //Winners list
    lottery_id: u32,                         //Lottery Id
}

impl Lottery {
    fn add_player(&mut self, player_id: &ActorId) {
        if msg::value() > 0 {
            let player = Player {
                player: *player_id,
                balance: msg::value(),
            };

            let player_index = self.players.len() as u32;
            self.players.insert(player_index, player);
            msg::reply(Event::PlayerAdded(player_index), 0);
        }
    }

    fn remove_player(&mut self, index: u32) {
        if self.players.len() > 0 {
            self.players.remove(&index);
        }
    }

    fn add_value(&mut self, index: u32, value: u128) {
        self.players.entry(index).and_modify(|item| {
            item.balance = item.balance.saturating_add(value);
        });
    }

    fn get_balance(&mut self, index: u32) {
        if let Some(player) = self.players.get(&index) {
            msg::reply(Event::Balance(player.balance), 0);
        }
    }

    fn get_players(&mut self) {
        if self.players.len() > 0 {
            msg::reply(Event::Players(self.players.clone()), 0);
        }
    }

    fn get_random_number(&mut self) -> u32 {
        let timestamp: u64 = exec::block_timestamp();
        let code_hash: sp_core::H256 = blake2_256(&timestamp.to_be_bytes()).into();
        let u_buf = code_hash.to_fixed_bytes();
        let mut number: u32 = 0;

        for u in u_buf {
            number += u as u32;
        }

        number
    }

    fn pick_winner(&mut self) {
        if self.players.len() > 0 {
            let index: u32 = self.get_random_number() % (self.players.len() as u32);

            if let Some(win_player) = self.players.get(&index) {
                msg::send_bytes(win_player.player, b"Winner", exec::value_available());
                self.lottery_history
                    .insert(self.lottery_id, win_player.player);
                msg::reply(Event::Winner(index), 0);
            }

            self.players = BTreeMap::new();
            self.lottery_id += 1;
        }
    }
}

static mut LOTTERY: Option<Lottery> = None;

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: Action = msg::load().expect("Could not load Action");
    let lottery: &mut Lottery = LOTTERY.get_or_insert(Lottery::default());

    match action {
        Action::Enter(account) => {
            lottery.add_player(&account);
        }

        Action::Start => {
            lottery.pick_winner();
            //msg::reply(Event::Winner(1), 0);
        }

        Action::BalanceOf(index) => {
            lottery.get_balance(index);
        }

        Action::GetPlayers => {
            lottery.get_players();
        }

        Action::DelPlayer(index) => {
            lottery.remove_player(index);
        }

        Action::AddValue(index) => {
            lottery.add_value(index, msg::value());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitConfig = msg::load().expect("Unable to decode InitConfig");

    let lottery = Lottery {
        lottery_owner: config.owner,
        ..Default::default()
    };

    LOTTERY = Some(lottery);
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: State = msg::load().expect("failed to decode input argument");
    let lottery: &mut Lottery = LOTTERY.get_or_insert(Lottery::default());

    let encoded = match query {
        State::GetPlayers => StateReply::Players(lottery.players.clone()).encode(),
        State::GetWinners => StateReply::Winners(lottery.lottery_history.clone()).encode(),

        State::BalanceOf(index) => {
            //let player = *lottery.players.get(&index).unwrap_or(&Player{player: ActorId::new([0u8; 32]), balance: 0,});

            if let Some(player) = lottery.players.get(&index) {
                StateReply::Balance(player.balance).encode()
            } else {
                StateReply::Balance(0).encode()
            }
        }
    };

    gstd::util::to_leak_ptr(encoded)
}

gstd::metadata! {
    title: "Lottery",
        init:
            input: InitConfig,
        handle:
            input: Action,
            output: Event,
        state:
            input: State,
            output: StateReply,
}
