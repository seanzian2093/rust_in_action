mod ch5_1_int_vs_int;
mod ch5_2_f32_as_u32;
mod ch5_3_to_oblivion;
mod ch5_4_bit_patterns;
mod ch5_5_impossible_addition;
mod ch5_6_endianess;
mod ch5_10_deconstruct_f32;
mod cpu1;
mod cpu2;
mod cpu3;

use ch5_q::Q7;

fn main() {
    // ch5_1_int_vs_int::main();
    // ch5_2_f32_as_u32::main();
    // ch5_3_to_oblivion::main();
    // ch5_4_bit_patterns::main();

    // `cargo run --release` use `opt-level=3`, by default
    // `opt-level=3` will allow arithmetic overflow, i.e., wrong number but no overflow error
    // `cargo run --dev` use `opt-level=1`, by default
    // `opt-level=1` will not allow arithmetic overflow, i.e., no overflow error
    // ch5_5_impossible_addition::main();

    // ch5_6_endianess::main();
    // ch5_10_deconstruct_f32::main();
    // cpu1::main::main();
    // cpu2::main::main();
    // cpu3::main::main();

    let q = Q7::from(0.7);
    let f: f32 = f32::from(q);
    println!("q={:?}, f={}", q, f);
}
