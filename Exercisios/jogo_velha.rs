use std::{io, char};


fn add_matriz(matriz:&mut Vec<Vec<char>>, x:&usize, y:&usize, count:& i32 ){

    if count % 2 == 0 && matriz[*x][*y] == ' '{
        matriz[*x][*y] = 'X';
    } else if count % 2 != 0 && matriz[*x][*y] == ' '{
        matriz[*x][*y] = 'O';
    }else{
        println!("\nLocal ja ocupado, perdeu a vez!")
    }
}

fn verifica(matriz:&mut Vec<Vec<char>>, count: i32) -> i32{

    for _i in 0..2{println!();} // Puramente por estetica

    let mut verificar:i32 = 0;
    for i in 0..3{
        let mut ver_linha: Vec<char> = Vec::new();
        let mut ver_coluna: Vec<char> = Vec::new();
    
        for j in 0..3{
            ver_linha.push(matriz[i][j]);
            ver_coluna.push(matriz[j][i]);
        }
        if ver_linha == ['X','X','X'] || ver_linha == ['O','O','O'] || ver_coluna == ['X','X','X'] || ver_coluna == ['O','O','O']{
            
            if count%2==0{ println!("O jogador 1 venceu!!");} else{ println!("O jogador 2 venceu!!");}
            verificar = 1;
        } 
    }
    
    if matriz[0][0] == matriz[1][1] && matriz[1][1] == matriz[2][2] {
        verificar = 1; }

    if matriz[0][2] == matriz[1][1] && matriz[1][1] == matriz[2][0] {
        verificar = 1; }

return verificar
}

fn print_matriz(matriz:&mut Vec<Vec<char>>){
    
    for _i in 0..2{println!();}

    for i in 0..matriz.len(){
        for j in 0..matriz.len(){
            print!("'{}'", matriz[i][j]);
        }println!();
    }
}
fn main(){
    
    let mut matriz: Vec<Vec<char>> = vec![vec![' ' ;3]; 3];
    let mut count: i32 = 2;

    loop{

        print_matriz(&mut matriz);

        let mut linha = String::new();
        let mut coluna = String::new();

        println!("Digite a linha que deseja botar na matriz(0..2):");
        io::stdin().read_line(&mut linha).expect("Erro ao ler valor");
        let x: usize = linha.trim().parse().expect("Erro ao converter valor.");

        println!("Digite a coluna que deseja botar na matriz(0..2):");
        io::stdin().read_line(&mut coluna).expect("Erro ao ler valor");
        let y: usize = coluna.trim().parse().expect("Erro ao converter valor.");

        add_matriz(&mut matriz, &x, &y, &count);
            
        if verifica(&mut matriz, count) == 1{
            
            print_matriz(&mut matriz);
            break
        }
        
        //Jogador 1 == numeros pares, Jogador 2 == numeros impares.
        count += 1;
    }
}