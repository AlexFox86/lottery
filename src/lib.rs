#![no_std]
//#![feature(const_btree_new)]

use sp_core::hashing::blake2_256;
use codec::{Decode, Encode};
use gstd::{debug, exec, msg, prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct Player{
    player: ActorId,
    balance: u128,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
struct Lottery {    
    lottery_owner: ActorId,                     //Хозяин лотереи
    players: BTreeMap<u32, Player>,             //Игроки
    lottery_history: BTreeMap<u64, ActorId>,   //Список победителей
    lottery_id: u64,                            //Id текущей лотереи
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Action {
    Enter(ActorId),     //Новый игрок
    Start,              //Запуск лотереи
    
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    Winner(ActorId),    //Победитель
}

/*#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum State {
    Players,    
    BalanceOf(ActorId),
    
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateReply {
    //GetPlayers(),    
    Balance(u128),
    
}*/


impl Lottery {
    fn add_player(&mut self, player_id: &ActorId){
        if msg::value() > 0 {
            let player = Player{
                player:  *player_id,
                balance: msg::value()
            };

            self.players.insert((self.players.len() + 1) as u32, player);
        }
    }

    fn get_random_number(&mut self) -> u128{        
        let timestamp: u64 = exec::block_timestamp();
        let code_hash: sp_core::H256 = blake2_256(&timestamp.to_be_bytes()).into();
        let u_buf = code_hash.to_fixed_bytes();        
        let mut number: u128 = 0;

        for &u_buf in u_buf.iter(){
            number += u_buf as u128;
        };       

        number
    }

    fn pick_winner(&mut self){
        let index: u32 = (self.get_random_number() % (self.players.len() as u128)) as u32;        

        if let Some(win_player) = self.players.get(&index){
            msg::send_bytes(win_player.player, b"Winner", exec::value_available());     
            self.lottery_history.insert(self.lottery_id, win_player.player);      
            msg::reply(Event::Winner(win_player.player), 0);  
        }
        else{
            debug!("win player Index error");
        }

        self.players = BTreeMap::new();
        self.lottery_id += 1;
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
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let owner: ActorId = msg::load().expect("Unable to decode Owner");
    debug!("Owner {:?}", owner);
    
    let lottery = Lottery{
        lottery_owner: msg::source(),
        players: BTreeMap::new(),
        lottery_history: BTreeMap::new(),
        lottery_id: 0,
    };

    LOTTERY = Some(lottery);
}