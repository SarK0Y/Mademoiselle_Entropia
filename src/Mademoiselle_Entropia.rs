use libc::TCA_STAB;
use num_traits::{ops::overflowing::{OverflowingAdd, OverflowingSub}, PrimInt};

use crate::custom_traits::{STRN, helpful_math_ops, arr2number, arrays};
use std::io::{Read, SeekFrom, Seek, Write};
use self::help_funcs::{new_IK, file_exts, dummy_file};
#[path ="help_funcs.rs"]
mod help_funcs;
#[path ="signals.rs"]
mod signals;
use signals::*;
pub trait cipher{
    fn encrypt(&mut self, PK: &String, key_size: usize, buf_size: usize);
    fn decrypt(&mut self, PK: &String, buf_size: usize);
    fn inject_IK(&mut self, PK: &String, IK: &mut Vec<u8>);
    fn extract_IK(&mut self, PK: &String) -> (Vec<u8>, u64);
}
impl cipher for std::fs::File{
    fn encrypt(&mut self, PK: &String, IK_size: usize, buf_size: usize){
        let func_name = "encrypt".strn();
        let mut buf: Vec<u8> = vec![0; buf_size];
        if buf.len() < buf_size{println!("Sorry, no memory for buffer that size"); return}
        let mut IK = new_IK(IK_size); let mut cursor_in_file = 0u64; let mut prev_cursor_in_file = 0u64;
        let mut IK0 = IK.clone();
        let mut progress = 0.0f64;
        let mut sum_of_encrypted_seg = 0.0f64;
        let file_len = match self.seek(SeekFrom::End(0)){Ok(s) => s, Err(e) => return println!("id: 0 func {func_name} got {:?}", e)} as f64;
        let step = 0.01;
        let mut prev_progress = progress;
        cursor_in_file = match self.seek(SeekFrom::Start(0)){Ok(s) => s, Err(e) => return println!("id: 1 func {func_name} got {:?}", e)};
        let mut iter_IK = 0usize;
        loop {
            let len_of_read = match self.read(&mut buf){Ok(f) => f, Err(e) => return println!("id: 2 func {func_name} got {:?}", e)};
            if len_of_read == 0{return print!("\rEncryption complete");}

            for i in 0..len_of_read{
               if iter_IK == IK.len(){ mutate(&mut IK); iter_IK = 0; }
                buf[ i ] ^= IK[ iter_IK ];
                iter_IK.inc();
            }
            if len_of_read < buf_size{
                let last_buf = buf[0..len_of_read].to_vec();
                match self.seek(SeekFrom::Start(cursor_in_file)){Ok(s) => s, Err(e) => return println!("id: 8 func {func_name} got {:?}", e)};
                match self.write(&last_buf){Ok(f) => f, Err(e) => return println!("id: 3 func {func_name} got {:?}", e)};
                self.inject_IK(PK, &mut IK0);
                return;
            }
            if progress - prev_progress > step{
                print!("\rEncrypted {progress} ...", );
                prev_progress = progress;
            }
            prev_cursor_in_file = cursor_in_file;
            cursor_in_file = match self.seek(SeekFrom::Current(0)){Ok(s) => s, Err(e) => return println!("id: 4 func {func_name} got {:?}", e)};
            match self.seek(SeekFrom::Start(prev_cursor_in_file)){Ok(s) => s, Err(e) => return println!("id: 5 func {func_name} got {:?}", e)};
            match self.write(&buf){Ok(f) => f, Err(e) => return println!("id: 6 func {func_name} got {:?}", e)};
            match self.seek(SeekFrom::Start(cursor_in_file)){Ok(s) => s, Err(e) => return println!("id: 7 func {func_name} got {:?}", e)};
            sum_of_encrypted_seg += len_of_read as f64;
            progress = sum_of_encrypted_seg / file_len;
        }        
    }
    fn inject_IK(&mut self, PK: &String, IK: &mut Vec<u8>){
        let func_name = "inject_IK".strn();
        let mut point_of_inject = [0u16; 4];
#[cfg(feature="dbg0")] let open_IK = IK.clone(); 
        for ch in PK.chars(){
            let ch0 = ch as usize;
            point_of_inject[ ch0 % 4] = point_of_inject[ ch0 % 4].overflowing_add(ch as u16).0;
        }
        let mut point_of_inject =point_of_inject.arr2u64();
        let file_len: u64 = match self.seek(SeekFrom::End(0)){Ok(s) => s, Err(e) => return println!("{func_name} got {:?}", e)};
        point_of_inject = point_of_inject % file_len;
        self.relocate_seg_from_to_tail(point_of_inject, IK.len() as u64);
        self.patch(point_of_inject, &mut IK.encrypt_IK(&PK));
        let mut tst = file_len as u128;
#[cfg(feature="dbg0")]
        println!("shift L {}, tst {}, sh R {}", tst.overflowing_shl(64).0, tst, tst.overflowing_shl(64).0.overflowing_shr(64).0);
        let mut tail = (file_len as u128).overflowing_shl(64).0 + point_of_inject as u128; 
#[cfg(feature="dbg0")]
        println!("{func_name}: {point_of_inject}, file len {:?}\n{:x}\n{:?}", file_len, tail, tail.to_be_bytes());
        let tail = tail.encrypt(&PK);
        match self.seek(SeekFrom::End(0)){Ok(s) => s, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.write(&tail.to_be_bytes()){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
#[cfg(feature="dbg0")]
        println!("{func_name}: {point_of_inject}, file len {:?}\n{:x}\n{:?}", file_len, tail, open_IK);
    }
    fn extract_IK(&mut self, PK: &String) -> (Vec<u8>, u64){
        let func_name = "extract_IK".strn();
        let mut tail = [0u8; 16]; tail[1] = 11;
        let mut none_  = |e: std::io::Error| -> (Vec<u8>, u64) {println!("{func_name} got {:?}", e); return (vec![0; 0], 0u64) };
        let cipher_text_len: u64 = match self.seek(SeekFrom::End(-16)){Ok(s) => s, Err(e) => return none_(e)};
        match self.read(&mut tail){Ok(s) => s, Err(e) => return none_(e)}; 
        let mut tail = tail.low_to_high().arr2u128(); #[cfg(feature="dbg0")] println!("{func_name}: tail {:?}", tail.to_be_bytes());
        let tail = tail.decrypt(&PK); #[cfg(feature="dbg0")] println!("{func_name}: tail {:?}", tail.to_be_bytes());
         let mut tail_b = tail.to_be_bytes();
        let mut inject_point: &mut [u8] =&mut tail_b[8..16]; #[cfg(feature="dbg0")] println!("{func_name}: inject point {:?}", inject_point);
        let inject_point = inject_point.low_to_high().arr2u64();
        let mut len_orig_file: &mut [u8] = &mut tail_b[0..8]; let len_orig_file = len_orig_file.low_to_high().arr2u64();
        let IK_len = cipher_text_len.overflowing_sub(len_orig_file.clone()).0;
        #[cfg(feature="dbg0")] println!("{func_name}: IK_len {IK_len} ciphertext len {cipher_text_len}");
        #[cfg(feature="dbg0")] println!("{func_name}: {inject_point} tail {:?}\n{:?}", tail_b, len_orig_file);
        let mut IK = vec![0u8; IK_len as usize];
        match self.seek(SeekFrom::Start(inject_point)){Ok(s) => s, Err(e) => return none_(e)};
        match self.read(&mut IK){Ok(s) => s, Err(e) => return none_(e)};
#[cfg(feature="dbg0")] println!("{func_name}: {inject_point} tail {:?}\n{:?}", tail_b, len_orig_file);
#[cfg(feature="dbg0")] println!("{func_name}: IK {:?}", IK.clone().decrypt_IK(&PK));
        (IK.decrypt_IK(&PK), inject_point)
    }
    fn decrypt(&mut self, PK: &String, buf_size: usize){
        let func_name = "decrypt".strn();
        let mut buf = vec![0u8; buf_size];
        if buf.len() == 0{println!("Sorry, no memory (RAM) for buffer that size"); return}
        let (mut IK, inject_point) = self.extract_IK(&PK); let IK_size = IK.len();
        let mut progress = 0.0f64;
        let mut sum_of_decrypted_seg = 0.0f64;
        let file_len_f64 = match self.seek(SeekFrom::End(0)){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)} as f64;
        let step = 0.01;
        let mut prev_progress = progress;
        let where_is_patch = match self.seek(SeekFrom::End((IK.len() as i64) * -1 - 16)){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)};
        let mut patch = vec![0u8; IK.len()];
        if patch.len() != IK.len(){println!("Sorry, not enough memory (RAM) for buffer that size"); return}
        match self.read(&mut patch){Ok(f) => f, Err(e) => return println!("func {func_name} got {:?}", e)};
        match self.seek(SeekFrom::Start(inject_point)){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)};
        match self.write(&mut patch){Ok(f) => f, Err(e) => return println!("func {func_name} got {:?}", e)};
        let file_len = match self.seek(SeekFrom::End(0)){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)};
        let decrypted_size = match self.seek(SeekFrom::Start(file_len - IK.len() as u64 - 16)){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)};
        match self.set_len(decrypted_size){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)};
        match self.seek(SeekFrom::Start(0)){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)};
        let mut cursor_in_file = match self.seek(SeekFrom::Current(0)){Ok(s) => s, Err(e) => return println!("func {func_name} got {:?}", e)};
        let mut prev_cursor_in_file = cursor_in_file;
        let mut iter_IK = 0usize;
        loop {
            let len_of_read = match self.read(&mut buf){Ok(f) => f, Err(e) => return println!("func {func_name} got {:?}", e)};
            if len_of_read == 0{return print!("\rDecryption complete");}
            for i in 0..len_of_read{
                if iter_IK == IK.len(){ mutate(&mut IK); iter_IK = 0; }
                buf[ i ] ^= IK[ iter_IK ];
                iter_IK.inc();
            }
            if len_of_read < buf_size{
                let last_buf = buf[0..len_of_read].to_vec();
                match self.seek(SeekFrom::Start(cursor_in_file)){Ok(s) => s, Err(e) => return println!("id: 8 func {func_name} got {:?}", e)};
                match self.write(&last_buf){Ok(f) => f, Err(e) => return println!("func {func_name} got {:?}", e)};
                return;
            }
            if progress - prev_progress > step{
                print!("\rDecrypted {progress} ...", );
                prev_progress = progress;
            }
            prev_cursor_in_file = cursor_in_file;
            cursor_in_file = match self.seek(SeekFrom::Current(0)){Ok(s) => s, Err(e) => return println!("id: 4 func {func_name} got {:?}", e)};
            match self.seek(SeekFrom::Start(prev_cursor_in_file)){Ok(s) => s, Err(e) => return println!("id: 5 func {func_name} got {:?}", e)};
            match self.write(&buf){Ok(f) => f, Err(e) => return println!("id: 6 func {func_name} got {:?}", e)};
            let tst_cur = match self.seek(SeekFrom::Start(cursor_in_file)){Ok(s) => s, Err(e) => return println!("id: 7 func {func_name} got {:?}", e)};
            if buf.len() != buf_size{
                println!("\nDecryption failed.. buf len {}", buf.len()); return;
            }
            sum_of_decrypted_seg += len_of_read as f64;
            progress = sum_of_decrypted_seg / file_len_f64;
    }
}
}
pub trait cipher_u128 {
    fn encrypt(&mut self, PK: &String) -> u128;
    fn decrypt(&mut self, PK: &String) -> u128;
}
impl cipher_u128 for u128{
    fn encrypt(&mut self, PK: &String) -> u128 {
        let func_name = "encrypt for u128".strn();
        let mut u128_ = [0u8; 16];
         let mut PK_as_vec = Vec::<u8>::with_capacity(PK.len()); if PK_as_vec.capacity() < PK.len(){println!("{func_name}: Sorry, no memory (RAM) for encryption");
         signal(Some(director::stop)); return 0}
        for j in PK.chars(){
            let mut u32_ = j as u32;
            while u32_ > 0{
                let u8_ = (u32_ & 255u32) as u8;
                PK_as_vec.push(u8_);
                u32_ = u32_ >> 8;
            }
        }    
        for ch in PK_as_vec.iter(){
            let ch0 = *ch as usize; let cursor = ch0 % 16;
            u128_[cursor] = u128_[cursor].overflowing_add(ch0 as u8).0;
            u128_[cursor] = u128_[cursor].overflowing_add(ch0.rotate_left(8) as u8).0;
        }
        let PK_len = PK_as_vec.len();
        for ch in u128_{
            let ch0 = ch as usize; let cursor = ch0 % PK_len;
            u128_[cursor % 16] = u128_[cursor].overflowing_add(PK.chars().nth(cursor).unwrap() as u8).0;
            u128_[cursor % 16] = u128_[cursor].overflowing_add((PK.chars().nth(cursor).unwrap() as u16).rotate_left(8) as u8).0;
        }
        let u128_ = u128_.arr2u128();
        let u128__ = !u128_;
        let u128_ = u128_.overflowing_mul(u128__).0;
        //let u128_ = u128__.rotate_left((u128_ & u32::MAX as u128) as u32);
#[cfg(feature="dbg0")] println!("{func_name}: u128_ {:?}", u128_.to_be_bytes());
        *self = self.overflowing_add(u128_.clone()).0; *self
    }
    fn decrypt(&mut self, PK: &String) -> u128 {
        let func_name = "decrypt for u128".strn();
        let mut u128_ = [0u8; 16];
         let mut PK_as_vec = Vec::<u8>::with_capacity(PK.len()); if PK_as_vec.capacity() < PK.len(){println!("{func_name}: Sorry, no memory (RAM) for encryption");
         signal(Some(director::stop)); return 0}
        for j in PK.chars(){
            let mut u32_ = j as u32;
            while u32_ > 0{
                let u8_ = (u32_ & 255u32) as u8;
                PK_as_vec.push(u8_);
                u32_ = u32_ >> 8;
            }
        }    
        for ch in PK_as_vec.iter(){
            let ch0 = *ch as usize; let cursor = ch0 % 16;
            u128_[cursor] = u128_[cursor].overflowing_add(ch0 as u8).0;
            u128_[cursor] = u128_[cursor].overflowing_add(ch0.rotate_left(8) as u8).0;
        }
        let PK_len = PK_as_vec.len();
        for ch in u128_{
            let ch0 = ch as usize; let cursor = ch0 % PK_len;
            u128_[cursor % 16] = u128_[cursor].overflowing_add(PK.chars().nth(cursor).unwrap() as u8).0;
            u128_[cursor % 16] = u128_[cursor].overflowing_add((PK.chars().nth(cursor).unwrap() as u16).rotate_left(8) as u8).0;
        }
        let u128_ = u128_.arr2u128();
        let u128__ = !u128_;
        let u128_ = u128_.overflowing_mul(u128__).0;
        //let u128_ = u128__.rotate_left((u128_ & u32::MAX as u128) as u32);
#[cfg(feature="dbg0")] println!("{func_name}: u128_ {:?}", u128_.to_be_bytes());
        *self = self.overflowing_sub(u128_.clone()).0; *self
    }
}
pub trait cipher_vec {
    fn encrypt_IK(&mut self, PK: &String) -> Self;
    fn decrypt_IK(&mut self, PK: &String) -> Self;
}
impl cipher_vec for Vec<u8>{
        fn encrypt_IK(&mut self, PK: &String) -> Self{
        let func_name = "encrypt_IK".strn();        
        let mut none_  = |e: std::io::Error| -> (Vec<u8>, u64) {println!("{func_name} got {:?}", e); return (vec![0; 0], 0u64) };
        let mut progress = 0.0f64;
        let mut sum_of_encrypted_seg = 0.0f64;
        let step = 0.01;
        let mut prev_progress = progress;
        let mut PK_as_vec = Vec::<u8>::with_capacity(PK.len()); if PK_as_vec.capacity() < PK.len(){println!("{func_name}: Sorry, no memory (RAM) for encryption"); return self.to_vec()}
        for j in PK.chars(){
            let mut u32_ = j as u32;
            while u32_ > 0{
                let u8_ = (u32_ & 255u32) as u8;
                PK_as_vec.push(u8_);
                u32_ = u32_ >> 8;
            }
        }            
         for i in 1..self.len(){
                let a =PK_as_vec[(i - 1) % PK_as_vec.len()]; let b =PK_as_vec[i % PK_as_vec.len()];
                self[i - 1] ^= a; self[i] ^= b;
                PK_as_vec[ (i - 1) % PK.len()] = a.overflowing_add(b).0; PK_as_vec[ i % PK.len()] ^= a;
            if progress - prev_progress > step{
                print!("\rEncrypted {progress} of IK ...", );
                prev_progress = progress;
            }
            sum_of_encrypted_seg += i as f64;
            progress = sum_of_encrypted_seg / (self.len() as f64);
        }        
        print!("\rIK's Encryption complete"); self.to_vec()
    }
    fn decrypt_IK(&mut self, PK: &String) -> Self{
        let func_name = "encrypt for cipher_vec".strn();        
        let mut none_  = |e: std::io::Error| -> (Vec<u8>, u64) {println!("{func_name} got {:?}", e); return (vec![0; 0], 0u64) };
        let mut progress = 0.0f64;
        let mut sum_of_encrypted_seg = 0.0f64;
        let step = 0.01;
        let mut prev_progress = progress;
        let mut PK_as_vec = Vec::<u8>::with_capacity(PK.len()); if PK_as_vec.capacity() < PK.len(){println!("{func_name}: Sorry, no memory (RAM) for encryption"); return self.to_vec()}
        for j in PK.chars(){
            let mut u32_ = j as u32;
            while u32_ > 0{
                let u8_ = (u32_ & 255u32) as u8;
                PK_as_vec.push(u8_);
                u32_ = u32_ >> 8;
            }
        }            
         for i in 1..self.len(){
                let a =PK_as_vec[(i - 1) % PK_as_vec.len()]; let b =PK_as_vec[i % PK_as_vec.len()];
                self[i - 1] ^= a; self[i] ^= b;
                PK_as_vec[ (i - 1) % PK.len()] = PK_as_vec[ (i - 1) % PK.len()].overflowing_add(b).0; PK_as_vec[ i % PK.len()] ^= a;
            if progress - prev_progress > step{
                print!("\rDecrypted {progress} of IK ...", );
                prev_progress = progress;
            }
            sum_of_encrypted_seg += i as f64;
            progress = sum_of_encrypted_seg / (self.len() as f64);
        }        
        print!("\rIK's Decryption complete"); self.to_vec()
    }
}
pub fn mutate(data: &mut Vec<u8>){
    for i in 1..data.len(){
        let a = data[ i - 1 ]; let b = data[ i ]; 
        data[ i - 1 ] = data[ i - 1 ].overflowing_add(b).0; data[ i ] ^= a;
    }
}
pub fn tst(){
    let mut tst = match help_funcs::get_file(&"./tst".strn()){Ok(f) => f, _ => return};
    tst.set_len(0);
    tst.populate_w_strn("a", /*file size*/41_219, /*buf size*/40*1024);
    tst.encrypt(&"pswd".strn(), /*key size*/30_000, /*buf size*/40*1024);
   // tst.extract_IK(&"pswd".strn());
    tst.decrypt(&"pswd".strn(), /*buf size*/40*1024)
}
//fn
