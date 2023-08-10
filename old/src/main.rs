// main.rs
// use libc::{c_int, c_void};

#[link(name = "aleo")]
extern "C" {
    fn aleo_proof_calc();
}

fn main() {
    unsafe {
        aleo_proof_calc();
    }
}