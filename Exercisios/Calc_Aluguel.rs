use std::io;

fn main(){
    let valor: u8 = 100;
    let mut dias_stg = String::new();
    
    println!("Digite quantos dias vai alugar o caroo:");
    io::stdin().read_line(&mut dias_stg).expect("Erro ao ler entrada");
    let dias: u8 = dias_stg.trim().parse().expect("Erro ao converter");

    let pf: u32 = (dias as u32) * (valor as u32);

    println!("valor ficou {}", pf);


}