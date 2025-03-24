use std::io; 

fn main(){
    println!("Digite ano de nascimento:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

    let input_num:u32 = input.trim().parse().expect("Erro ao converter string");
    let idade:u32 = 2023 - input_num;
    println!("Sua idade é {}", idade);




}