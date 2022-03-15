#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitConfig {
    pub owner: ActorId,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct Player {
    pub player: ActorId,
    pub balance: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum Action {
    ///New player
    Enter(ActorId),
    ///Start lottery
    Start,
    ///Get balance
    BalanceOf(u32),
    ///Get players list
    GetPlayers,
    ///Remove player
    DelPlayer(u32),
    ///Add balance
    AddValue(u32),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    Winner(u32),
    Balance(u128),
    Players(BTreeMap<u32, Player>),
    ///Player added(Index)
    PlayerAdded(u32),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum State {
    GetWinners,
    GetPlayers,
    BalanceOf(u32),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateReply {
    Winners(BTreeMap<u32, ActorId>),
    Players(BTreeMap<u32, Player>),
    Balance(u128),
}
