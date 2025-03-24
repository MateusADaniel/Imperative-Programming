use std::io;

fn main(){
    let mut num:u32 = 0;

    for i in 1..101{
        if i % 2 !=0{
            num += i;
        }
    }
    println!("A soma dos impares ficou {}", num)




}