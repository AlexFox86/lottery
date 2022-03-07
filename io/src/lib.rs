#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct Player {
    pub player: ActorId,
    pub balance: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Action {
    Enter(ActorId), //Новый игрок
    Start,          //Запуск лотереи
    BalanceOf(u32), //Запрос баланса
    GetPlayers,     //Запрос списка
    DelPlayer(u32), //Удалить игрока
    AddValue(u32),  //Увеличить баланс
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    Winner(ActorId),                //Победитель
    Balance(u128),                  //Баланс
    Players(BTreeMap<u32, Player>), //Игроки
    PlayerAdded(u32),               //Игрок добавлен(Индекс)
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum State {
    GetWinner,
    GetPlayers,
    BalanceOf(u32),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateReply {
    Winner(ActorId),
    Players(BTreeMap<u32, Player>),
    Balance(u128),
}