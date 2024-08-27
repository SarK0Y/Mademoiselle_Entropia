/*Order of the Chaos*/
use num_traits::{ops::overflowing::{OverflowingAdd, OverflowingMul}, PrimInt};
use unic_ucd::common::is_alphanumeric;
use std::thread::spawn;
#[path = "signals.rs"]
mod signals;
use signals::*;
use crate::custom_traits::STRN;
#[derive(PartialEq, Debug)]
pub enum rnd_key{
    xor,
    add,
    rotate_Left,
    mul,
    ret,
}
pub fn true_rnd_byte(seed: Option<u8>, op: rnd_key) -> u8{
    static mut rnd_byte: u8 = 8u8;
    crate::help_funcs::delay_ns(213);
    if unsafe {rnd_byte} == 0{unsafe {rnd_byte = 23}}
    if op == rnd_key::ret{return unsafe{rnd_byte}}
    if op == rnd_key::xor{
        if let Some(x) = seed{
            unsafe{
                rnd_byte  ^= rnd_byte.rotate_left(x as u32)
            }
        } else {unsafe {rnd_byte  ^= rnd_byte.rotate_left(7)}}
    }
    if op == rnd_key::add{
        if let Some(x) = seed{
            unsafe{
                rnd_byte = rnd_byte.overflowing_add(x).0;
            }
        } else {unsafe {rnd_byte = rnd_byte.overflowing_add(rnd_byte.rotate_right(5)).0}}
    }
    if op == rnd_key::rotate_Left{
        if let Some(x) = seed{
            unsafe{
                rnd_byte.rotate_left(x as u32);
            }
        } else {unsafe {rnd_byte ^= rnd_byte.rotate_left(3)}}
    }
    if op == rnd_key::mul{
        if let Some(x) = seed{
            unsafe{
                rnd_byte = rnd_byte.overflowing_mul(x).0;
            }
        } else {unsafe {rnd_byte = rnd_byte.overflowing_mul(rnd_byte.rotate_left(3)).0}}
    }
    let ret = unsafe{rnd_byte};
    //println!("{} {:#?}", ret, op);
    ret
}
pub fn rnd_u8_arr(seed: Option<u8>, size: usize) -> Vec<u8>{
    let func_name = "rnd_u8_arr".strn();
    let mut ret = vec![0u8; size];
    if ret.len() < size{println!("{func_name} Can't allocate array of {size} bytes" ); signal(Some(director::stop)); return ret;}
    let mut seed0 = 13u8;
    if let Some(seed0) = seed{}
    for j in 0..size{
        ret.push(get_true_rnd_u8(Some(seed0)))
    }
    ret    
}
pub fn rndomize_u8_arr(arr: &mut Vec<u8>) {
    let func_name = "rndomize_u8_arr".strn();
    for j in 0..arr.len(){
        arr[j] ^= get_true_rnd_u8(Some(arr[j]))
    }    
}
pub fn get_true_rnd_u8(seed: Option<u8>) -> u8{
    let mut seed0 = 13u8;
    if let Some (x) = seed{seed0 = x}
    let thr0 = spawn(move||{
        for i in 0..157{
            true_rnd_byte(Some(seed0.clone()), rnd_key::xor);
        }
    });
    let thr1 = spawn(move||{
        for i in 0..153{
            true_rnd_byte(Some(seed0.clone()), rnd_key::add);
        }
    });
    let thr2 = spawn(move||{
        for i in 0..83{
            true_rnd_byte(Some(seed0.clone()), rnd_key::mul);
        }
    });
       let thr3 = spawn(move||{
        for i in 0..57{
            true_rnd_byte(Some(seed0.clone()), rnd_key::rotate_Left);
        }
    });
    thr2.join(); thr0.join();
    true_rnd_byte(None, rnd_key::ret)
}
pub fn UID_UTF8(num_of_bytes: usize) -> String{
    use unic_ucd::Alphabetic;
    use crate::custom_traits::helpful_math_ops;
    let mut uid = "".strn();
    let mut rnd_bytes: Vec <u8> = Vec::new();
    let mut count_down = num_of_bytes;
    let mut shift = 0;
    let mut seed = 1u8;
    let mut valid_char: u32;
    let mut mask = 0u32;
    let mut mask1 = 0u32;
    let mut mask2 = 0u32;
    let mut mask3 = 0u32;
    while count_down > 0 {
        loop {
            if shift == 4 {break;}
            let rnd_byte = get_true_rnd_u8(Some(seed));
            seed = seed.overflowing_add(rnd_byte.clone() ).0.rotate_left(rnd_byte.clone() as u32);
            rnd_bytes.push(rnd_byte);
            if shift == 0{
                mask = rnd_byte as u32;
            }
            if shift == 1{
                mask1 = rnd_byte as u32;
                mask1 = mask1.overflowing_shl(shift).0;
            }
            if shift == 2{
                mask2 = rnd_byte as u32;
                mask2 = mask2.overflowing_shl(shift).0;
            }
            if shift == 1{
                mask3 = rnd_byte as u32;
                mask3 = mask3.overflowing_shl(shift).0;
            }
            shift.inc();
        }
        shift = 0;
        valid_char = mask + mask1 + mask2 + mask3; mask = 0; mask1 = 0; mask2 = 0; mask3 = 0;
        let ch = char::from_u32(valid_char).unwrap();
        if is_alphanumeric(ch){count_down.dec(); uid.push(ch)}
        valid_char = 0;
    }
//#[cfg(feature = "in_dbg")] println!("\n{:?}\n {:?}", rnd_bytes, uid.as_bytes() );
    uid
}
pub fn get_true_rnd_u64 () -> u64 {
    let mut byte = get_true_rnd_u8(Some(47) );
    let mut u64_: u64 = 0;
    let mut shift: u64;
    for i in 0..8{
        byte = get_true_rnd_u8(Some (byte) );
        shift = byte as u64;
        u64_ += shift << i; 
    }
u64_
}
//fn
