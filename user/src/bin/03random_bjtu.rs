#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

/**
 * @author 王文
 * @description 实现一个输出个人信息 "I am Wang Wen from BJTU! Nice to meet u!"
 */

#[no_mangle]
fn main() -> i32 {
    print!("I am Wang Wen from BJTU! Nice to meet u!");
    println!("");
    print!("Test is finished!");
    0
}
