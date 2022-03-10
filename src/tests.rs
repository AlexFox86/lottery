use codec::Encode;
use lt_io::*;
use gstd::String;
use gtest::{Program, System};
const USERS: &'static [u64] = &[3, 4, 5];

fn init(sys: &System) {
    sys.init_logger();

    let ft = Program::from_file(&sys,
        "./target/wasm32-unknown-unknown/release/lottery.wasm",);

    let res = ft.send_with_value(USERS[0],
        USERS[0].into(),
    );

    assert!(res.log().is_empty());    
}

#[test]
fn add_player() {
    let sys = System::new();
    init(&sys);
    let lt = sys.get_program(1);
    let res = lt.send_with_value(USERS[0], Action::Enter(USERS[0].into()), 1000);
    assert!(res.contains(&(USERS[0], Event::PlayerAdded(0).encode())));

    let res2 = lt.send_with_value(USERS[0], Action::Enter(USERS[1].into()), 2000);
    assert!(res2.contains(&(USERS[0], Event::PlayerAdded(1).encode())));
}

#[test]
fn get_balance() {
    let sys = System::new();
    init(&sys);
    let lt = sys.get_program(1);
    let res1 = lt.send_with_value(USERS[0], Action::Enter(USERS[0].into()), 1000);
    assert!(res1.contains(&(USERS[0], Event::PlayerAdded(0).encode())));

    let res2 = lt.send_with_value(USERS[0], Action::Enter(USERS[1].into()), 2000);
    assert!(res2.contains(&(USERS[0], Event::PlayerAdded(1).encode())));

    let res3 = lt.send(USERS[0], Action::BalanceOf(0));
    assert!(res3.contains(&(USERS[0], Event::Balance(1000).encode())));

    let res3 = lt.send(USERS[0], Action::BalanceOf(1));
    assert!(res3.contains(&(USERS[0], Event::Balance(3000).encode())));
}