use std::{
    path::Path,
    io::{Read, SeekFrom, Seek, Write},
};
use chrono::{DateTime, Local};
use num_traits::ops::overflowing::OverflowingSub;
use crate::custom_traits::{STRN, helpful_math_ops};
#[path="true_rnd.rs"]
mod true_rnd;
use true_rnd::*;
#[path="signals.rs"]
mod signals;
use signals::*;
pub fn get_file(filename: &String) -> Result<std::fs::File, std::io::ErrorKind>{
    if !Path::new(filename).exists(){println!("{} doesn't exist.", filename);return Err(std::io::ErrorKind::NotFound)}
    match std::fs::File::options().write(true).read(true).open(filename){
        Ok(file) => {return Ok(file)},
        Err(e) => match e.kind(){
            std::io::ErrorKind::PermissionDenied => {println!("Sorry, Dear User, You have no permissions to {}", filename); return Err(std::io::ErrorKind::PermissionDenied)},
            std::io::ErrorKind::InvalidData => {println!("Sorry, Dear User, {} was corrupted", filename); return Err(std::io::ErrorKind::InvalidData)},
            std::io::ErrorKind::OutOfMemory => {println!("Sorry, Dear User, No memory to process {}", filename); return Err(std::io::ErrorKind::OutOfMemory)},
            std::io::ErrorKind::Other => {println!("Sorry, Dear User, Not sure of error with {}", filename); return Err(std::io::ErrorKind::Other)},
            _ =>  {println!("Sorry, Dear User, Not sure of error with {}", filename); return Err(std::io::ErrorKind::Other)},
        },
    }
}
pub fn get_file_append(filename: &String) -> Result<std::fs::File, std::io::ErrorKind>{
    if !Path::new(filename).exists(){println!("{} doesn't exist.", filename);return Err(std::io::ErrorKind::NotFound)}
    match std::fs::File::options().write(true).read(true).append(true).open(filename){
        Ok(file) => {return Ok(file)},
        Err(e) => match e.kind(){
            std::io::ErrorKind::PermissionDenied => {println!("Sorry, Dear User, You have no permissions to {}", filename); return Err(std::io::ErrorKind::PermissionDenied)},
            std::io::ErrorKind::InvalidData => {println!("Sorry, Dear User, {} was corrupted", filename); return Err(std::io::ErrorKind::InvalidData)},
            std::io::ErrorKind::OutOfMemory => {println!("Sorry, Dear User, No memory to process {}", filename); return Err(std::io::ErrorKind::OutOfMemory)},
            std::io::ErrorKind::Other => {println!("Sorry, Dear User, Not sure of error with {}", filename); return Err(std::io::ErrorKind::Other)},
            _ =>  {println!("Sorry, Dear User, Not sure of error with {}", filename); return Err(std::io::ErrorKind::Other)},
        },
    }
}
pub fn new_IK(size: usize) -> Vec<u8>{
    let  func_name = "new_IK".strn();
    let mut rnd = match get_file(&"/dev/urandom".strn()){Ok(f) => f, _ => return vec![0; 0]};
    let mut buf = vec![0u8; size];
    if buf.len() < size{println!("{func_name} Can't allocate array of {size} bytes" ); signal(Some(director::stop)); return buf;}
    rndomize_u8_arr(&mut buf);
    rnd.read_exact(&mut buf); buf
}
pub trait dummy_file {
    fn populate_w_strn(&mut self, strn: &str, file_size: usize, buf_size: usize);
}
impl dummy_file for std::fs::File{
    fn populate_w_strn(&mut self, strn: &str, file_size: usize, buf_size: usize){
        let func_name = "populate_w_strn";
        let mut buf = String::with_capacity(buf_size);
        let mut file_size = file_size;
        while file_size > 0 {
            buf.clear();
            for i in 0..buf_size{
                buf.push_str(strn);
                if buf.len() + strn.len() > buf_size{break;} 
                if buf.len() + strn.len() > file_size{break;} 
            }
             match self.write(&buf.as_bytes()){Ok(f) => f, Err(e) => return println!("func {func_name} got {:?}", e)};
             let ret = file_size.overflowing_sub(buf.len());
             file_size = ret.0;
            if ret.1 || file_size == 0 {return};
        }
    }
}
pub trait file_exts{
    fn copy_from_to(&mut self, from: u64, to: u64) -> Option<(std::fs::File, std::fs::File)>;
    fn relocate_seg_from_to_tail(&mut self, from: u64, seg_size: u64);
    fn relocate_seg_from_to(&mut self, from: u64, to: u64, seg_size: u64);
    fn patch(&mut self, from: u64, patch: &Vec<u8>);
}
impl file_exts for std::fs::File{
    fn copy_from_to(&mut self, from: u64, to: u64) -> Option<(std::fs::File, std::fs::File)>{
        // Not completed
        let mut low_file = match get_file(&"/dev/urandom".strn()){Ok(f) => f, _ => return None};
        let mut high_file = match get_file(&"/dev/urandom".strn()){Ok(f) => f, _ => return None};
        None
    }
    fn relocate_seg_from_to_tail(&mut self, from: u64, seg_size: u64){
        let func_name = "relocate_seg_from_to_tail".strn();
        let mut seg = vec![0; seg_size as usize];
        let file_len = match self.seek(SeekFrom::End(0)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)}; let new_len = file_len + seg_size;
        if file_len < from{return println!("{func_name}: address to patch from is too high");}
        match self.set_len(new_len){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)}
        match self.seek(SeekFrom::Start(from)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.read(&mut seg){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.seek(SeekFrom::Start(file_len)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.write(&seg){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
    }
    fn relocate_seg_from_to(&mut self, from: u64, to: u64, seg_size: u64){
        let func_name = "relocate_seg_from_to".strn();
        let mut seg = vec![0; seg_size as usize];
        let file_len = match self.seek(SeekFrom::End(0)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        if file_len < to || file_len < from{return println!("{func_name}: address to patch from/to is too high");}
        match self.seek(SeekFrom::Start(from)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.read(&mut seg){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.seek(SeekFrom::Start(to)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.write(&seg){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
    }
    fn patch(&mut self, from: u64, patch: &Vec<u8>){
        let func_name = "patch".strn();
        let file_len = match self.seek(SeekFrom::End(0)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)}; 
        if file_len < from{return println!("{func_name}: address to patch from is too high");}
        match self.seek(SeekFrom::Start(from)){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
        match self.write(&patch){Ok(l) => l, Err(e) => return println!("{func_name} got {:?}", e)};
    }
}
pub(crate) fn delay_ms(sleep: u64){
    std::thread::sleep(std::time::Duration::from_millis(sleep));
}
pub(crate) fn delay_ns(sleep: u64){
    std::thread::sleep(std::time::Duration::from_nanos(sleep));
}
pub(crate) fn delay_mcs(sleep: u64){
    std::thread::sleep(std::time::Duration::from_micros(sleep));
}
//fn
