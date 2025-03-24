use std::io;

fn fat(tam:u64) -> u64{ 
    let mut resultado: u64 = 1;

    for i in 1..tam+1{

        resultado *= i;
    }
    
    return resultado;

}

fn main(){

    let mut tam_stg = String::new();
    println!("Digite o tamanho do fatorial: ");
    io::stdin().read_line(&mut tam_stg).expect("Erro ao ocnverter");
    let tam:u64= tam_stg.trim().parse().expect("erro ao converter");
    fat(tam);

    println!("Retorno {}", fat(tam));

}

