// Each floating-point number is laid out in memory as scientific notation
// Taking `42.42_f32` as example, there are 5 parts, but 3 parts in memory
// #1 Sign bit, e.g., 0
// #2 Exponent, e.g., 132
// #3 Mantissa/significand, e.g., 1.325625
// #4 Base/radix, default to 2
// #5 Exponent bias, default to 127 for f32, based on IEEE standards

// so 42.42 = 1 * 1.325625 * 2 ^ (132 -127)
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;
pub fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("\n{} -> {}", n, n_);
    println!("field     | as bits | as real number");
    println!("sign      | {:01b} | {}", sign, sign_);
    println!("exponent  | {:08b} | {}", exp, exp_);
    println!("mantissa  | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    // Shift bits to right by 31, i.e., only 1st bit remains, the sign bit
    let sign = (bits >> 31) & 1;
    // Shift bits to right by 23, i.e., only first 9 bits remain,
    // Use `0xff`, i.e., AND mask to remove the sign bit
    let exponent = (bits >> 23) & 0xff;
    // Similarly use `0x7fffffff` to remove sign bit and exponent
    // We call it fraction here and mantissa after decoding it
    let fraction = bits & 0x7fffffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);
    // mantissa starts with 1, i.e., 2 ^ 0
    let mut mantissa = 1.0_f32;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

// For  simplicity, we use `f32` to calculate
fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}




