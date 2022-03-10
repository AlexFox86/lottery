use codec::Encode;
use lt_io::*;
use gstd::String;
use gtest::{Program, System};
const USERS: &'static [u64] = &[3, 4, 5];

fn init(sys: &System) {
    sys.init_logger();

    let ft = Program::from_file(&sys,
        "./target/wasm32-unknown-unknown/release/lottery.wasm",);

    /*let res = ft.send(USERS[0],
        ,
    );

    assert!(res.log().is_empty());

    let res = ft.send(USERS[0], Action::Mint(1000000));
    assert!(res.contains(&(
        USERS[0],
        Event::Transfer {
            from: 0.into(),
            to: USERS[0].into(),
            amount: 1000000,
        }
        .encode()
    )));*/
}