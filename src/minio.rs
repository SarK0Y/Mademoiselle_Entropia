use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::fs::File;
use std::os::fd::FromRawFd;
use std::io::Write;
use std::io;
use std::io::Read; 
use crate::custom_traits::STRN;
pub fn InterruptMsg(msg: &str) {
    let msg = format!(
        "Interrupt msg: {}",
        msg
    );
    println! ("{msg}");
}

pub fn getch() -> char {
    let mut ch: char = '\0';
    let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 6] = [0; 6];
    let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap();
    new_termios.c_lflag &= !(ICANON | ECHO);
    let enter = || {
        let enter: [u8; 1] = [13; 1];
        let mut writeIn_stdin = unsafe {
            File::from_raw_fd(0 /*stdin*/)
        };
        writeIn_stdin.write(&enter);
        println!("gotta enter");
    };
    loop {
        let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios) {
            Err(e) => {
                format!("{}", e)
            }
            Ok(len) => {
                format!("kkkkkkkkkkk {:#?}", len)
            }
        };
        let red_stdin = stdin.read(&mut stdin_buf);
        stdout.lock().flush().unwrap();
        end_termios(&termios);
        let str0 = match str::from_utf8(&stdin_buf) {
            Ok(s) => s,
            _ => "",
        };
        let msg = format!("getch {} {:?}", str0, stdin_buf);
        if stdin_buf != [0; 6] {
            return str0.chars().nth(0).unwrap();
        }
    }
    ch
}
pub(crate) fn getkey() -> String {
    let mut Key: String = "".to_string();
    let mut stdin = io::stdin().lock();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 16] = [0; 16];
    let mut stdin_buf0: [u8; 1] = [1; 1];
    let termios = match Termios::from_fd(stdin_fd) {
        Ok(t) => t,
        _ => {return "".to_string()},
    };
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap();
    //new_termios.c_lflag &= !(ICANON | ECHO | ISIG);
    new_termios.c_lflag &= !(ICANON | ECHO);
    let enter = || {
        let enter: [u8; 1] = [13; 1];
        let mut writeIn_stdin = unsafe {
            File::from_raw_fd(0 /*stdin*/)
        };
    
    };
    let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios) {
        Err(e) => {
            format!("{}", e)
        }
        Ok(len) => {
            format!("kkkkkkkkkkk {:#?}", len)
        }
    };
    let red_stdin = match stdin.read(&mut stdin_buf) {
        Ok(red) => red,
        Err(e) => {
            InterruptMsg(&format!("{e:?}"));
            return "".strn();
        }
    };
    let mut j = 1usize;
    end_termios(&termios);
    let str0 = match str::from_utf8(&stdin_buf) {
        Ok(s) => s,
        _ => "",
    };
    let msg = format!("getch {} {:?}", str0, stdin_buf);
    if stdin_buf != [0; 16] {
        let mut i = 0;
        loop {
            let ch = match str0.chars().nth(i) {
                Some(ch) => ch,
                _ => return Key,
            };
            if ch == '\0' {
                return Key;
            }
            Key.push(ch);
            i += 1;
        }
    }
    //}
    Key
}
 fn end_termios(termios: &Termios) {
    let res = match tcsetattr(0, TCSANOW, &termios) {
        Err(e) => {
            format!("{}", e)
        }
        Ok(len) => {
            format!("{:?}", len)
        }
    };
    io::stdout().flush().unwrap();
}

