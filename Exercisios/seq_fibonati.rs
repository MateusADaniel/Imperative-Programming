use std::io;

fn main(){

    let mut x:u16 = 0;
    let mut y:u16 = 1;
    let mut tam_stg = String::new();
    let mut trade:u16 = 0;
    let mut soma:u16 = 0;
    println!("Digite o tamananho ");
    io::stdin().read_line(&mut tam_stg).expect("Erro ao converter");
    let tamanho: u16 = tam_stg.trim().parse().expect("Erro ao trocar");
    

    if tamanho == 1{
        println!("Fibonita 1= {}",x);
    }
    else{
        println!("Fibonita 1= {}",x);
        println!("Fibonita 2= {}",y);
        for i in 3..tamanho +1{
            trade = x + y;
            x = y;
            y = trade;

            println!("Fibonati {i}= {}", trade);

            if trade % 2 ==0{
                soma += trade;
            }
        }
        println!("A soma dos apres ficou: {}", soma);
    }
}