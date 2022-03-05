#![no_std]
//#![feature(const_btree_new)]

use codec::{Decode, Encode};
use gstd::{debug, exec, msg, prelude::*, ActorId};
use scale_info::TypeInfo;
use sp_core::hashing::blake2_256;

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
struct Player {
    player: ActorId,
    balance: u128,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
struct Lottery {
    lottery_owner: ActorId,                  //Хозяин лотереи
    players: BTreeMap<u32, Player>,          //Игроки
    lottery_history: BTreeMap<u32, ActorId>, //Список победителей
    lottery_id: u32,                         //Id текущей лотереи
}

#[derive(Debug, Decode, Encode, TypeInfo)]
enum Action {
    Enter(ActorId), //Новый игрок
    Start,          //Запуск лотереи
    BalanceOf(u32), //Запрос баланса
    GetPlayers,     //Запрос списка
    DelPlayer(u32), //Удалить игрока
    AddValue(u32),  //Увеличить баланс
}

#[derive(Debug, Encode, Decode, TypeInfo)]
enum Event {
    Winner(ActorId),                //Победитель
    Balance(u128),                  //Баланс
    Players(BTreeMap<u32, Player>), //Игроки
    PlayerAdded(u32),               //Игрок добавлен(Индекс)
}

impl Lottery {
    fn add_player(&mut self, player_id: &ActorId) {
        if msg::value() > 0 {
            let player = Player {
                player: *player_id,
                balance: msg::value(),
            };

            let player_index: u32 = self.players.len() as u32;
            self.players.insert(player_index, player);
            msg::reply(Event::PlayerAdded(player_index), 0);
        }
    }

    fn del_player(&mut self, index: u32) {
        if self.players.len() > 0 {
            self.players.remove(&index);
        }
    }

    fn add_value(&mut self, index: u32, value: u128) {
        self.players.entry(index).and_modify(|item| {
            item.balance += value;
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

        for &u_buf in u_buf.iter() {
            number += u_buf as u32;
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
                msg::reply(Event::Winner(win_player.player), 0);
            } else {
                debug!("win player Index error");
            }

            self.players = BTreeMap::new();
            self.lottery_id += 1;
        } else{
            debug!("no players in lottery");
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
        }

        Action::BalanceOf(index) => {
            lottery.get_balance(index);
        }

        Action::GetPlayers => {
            lottery.get_players();
        }

        Action::DelPlayer(index) => {
            lottery.del_player(index);
        }

        Action::AddValue(index) => {
            lottery.add_value(index, msg::value());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let owner: ActorId = msg::load().expect("Unable to decode Owner");
    debug!("Owner {:?}", owner);

    let lottery = Lottery {
        lottery_owner: msg::source(),
        players: BTreeMap::new(),
        lottery_history: BTreeMap::new(),
        lottery_id: 0,
    };

    LOTTERY = Some(lottery);
}

gstd::metadata! {
    title: "Lottery",
        init:
            input: ActorId,
        handle:
            input: Action,
            output: Event,
}