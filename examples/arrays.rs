#![deny(warnings)]

use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let to_send = [1, 2, 3];
    let future = world.any_process().immediate_receive::<[i32; 3usize]>();
    world.this_process().send(&to_send);
    let (msg, _status) = future.get();
    assert_eq!(to_send, msg);
}
