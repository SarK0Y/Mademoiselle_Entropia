/*Order of the Chaos*/
use num_traits::ops::overflowing::OverflowingMul;
use std::thread::spawn;
#[path = "signals.rs"]
mod signals;
use signals::*;
use crate::custom_traits::STRN;
#[derive(PartialEq)]
pub enum rnd_key{
    xor,
    add,
    rotate_Left,
    mul,
    ret,
}
pub fn true_rnd_byte(seed: Option<u8>, op: rnd_key) -> u8{
    static mut rnd_byte: u8 = 8u8;
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
    unsafe{rnd_byte}
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
    if let Some(seed0) = seed{};
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
