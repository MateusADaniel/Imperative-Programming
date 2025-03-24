//DEIXEI EM UM MESMO ARQUIVO PARA FACÍLITAR, NÃO SEI SE POSSO SEPARAR TUDO EM FUNÇÕES ASSIM.
pub mod item1_5 {
    use std::io;

    pub fn cria_enche_vetor() -> (Vec<f32>, Vec<String>,usize){
        let mut vet_notas: Vec<f32> = Vec::new();
        let mut stg_num_alunos = String::new();
        let mut _vet_nomes: Vec<String> = Vec::new();
        
        println!("Indique a quantidade de alunos na turma: ");
        io::stdin().read_line(&mut stg_num_alunos).expect("Erro ao ler valor");
        let num_alunos: usize = stg_num_alunos.trim().parse().expect("Erro ao converter valor");
    
        for i in 0..num_alunos {
            let mut stg_notas = String::new();
            let mut stg_nomes = String::new();

            println!("\nDigite o nome do aluno {}",i);
            io::stdin().read_line(&mut stg_nomes).expect("Erro ao ler valor.");
            _vet_nomes.push(stg_nomes);

            print!("Digite a nota do aluno {}", _vet_nomes[i]);
            io::stdin().read_line(&mut stg_notas).expect("Erro ao ler valor");
            let nota: f32 = stg_notas.trim().parse().expect("Erro ao converter");
            
            vet_notas.push(nota);
        }
        (vet_notas, _vet_nomes, num_alunos)
    }

    pub fn media(vet:& [f32], tam:& usize) -> f32{
        let mut soma:f32 = 0.0;
        for elemento in vet{
            soma += *elemento;
        }
        return soma / (*tam as f32);
    }
    
    pub fn maior_menor_nota(vet_notas:& [f32]) ->(f32, f32){
        let mut menor_nota:&f32 = &vet_notas[0];
        let mut maior_nota:&f32 = &vet_notas[0];
        for elemento in vet_notas{
            if elemento > &maior_nota{
                maior_nota = elemento;
            }if elemento < &menor_nota{
                menor_nota = elemento; }
        } 
        (*maior_nota, *menor_nota)
    }

    
}

//DEIXEI EM UM MESMO ARQUIVO PARA FACÍLITAR, NÃO SEI SE POSSO SEPARAR TUDO EM FUNÇÕES ASSIM.
pub mod item6_10 {
    use std::io;

    pub fn maior_menor_nota_geral(matriz_notas:& Vec<Vec<f32>>, num_alunos:& usize, num_disciplinas:& usize) ->(f32, f32){
        let mut maior_nota_geral: &f32 = &matriz_notas[0][0];
        let mut menor_nota_geral: &f32 = &matriz_notas[0][0];

        for i in 0..*num_alunos{
            for j in 0..*num_disciplinas{
                if maior_nota_geral < &matriz_notas[i][j]{
                    maior_nota_geral = &matriz_notas[i][j];
                }if menor_nota_geral > &matriz_notas[i][j]{
                    menor_nota_geral = &matriz_notas[i][j] }
            }
        } (*maior_nota_geral, *menor_nota_geral)
    } 
    
    pub fn maior_nota_disciplina(matriz_notas: &Vec<Vec<f32>>, materia:& usize, tamanho:& usize) -> f32{

            let mut maior_nota:f32 = 0.0;
            for i in 0..*tamanho{
                if maior_nota < matriz_notas[i][*materia]{
                    maior_nota = matriz_notas[i][*materia]; } 
            }return maior_nota; 
    }    

    pub fn criar_matriz() ->(Vec<Vec<f32>>, usize, usize, Vec<String>){
        for _i in 0..3{ println!();}
        let mut num_alunos = String::new();
        let mut num_disciplinas = String::new();

        println!("Informe a quantidade de alunos na turma: ");
        io::stdin().read_line(&mut num_alunos).expect("Erro ao ler valor.");
        println!("Informe o número de matérias: ");
        io::stdin().read_line(&mut num_disciplinas).expect("Erro ao ler valor.");


        let num_alunos:usize = num_alunos.trim().parse().expect("Erro ao converter valor.");
        let num_disciplinas:usize = num_disciplinas.trim().parse().expect("Erro ao converter valor.");
        
        let matriz: Vec<Vec<f32>> = vec![vec![0.0; num_disciplinas]; num_alunos];    
        let mut _vet_nomes: Vec<String> = Vec::new();

        for i in 0..num_alunos {
            let mut stg_nomes = String::new();

            println!("\nDigite o nome do aluno {}",i);
            io::stdin().read_line(&mut stg_nomes).expect("Erro ao ler valor.");
            _vet_nomes.push(stg_nomes);
        }
        (matriz, num_alunos, num_disciplinas, _vet_nomes)
    }

    pub fn encher_matriz() -> (Vec<Vec<f32>>, f32, usize, usize, Vec<String>){
        //Queria muito chamar uma função dentro de outra, perdão se ficou meio confuso.
        
        let (mut matriz, num_alunos, num_disciplinas, vet_nomes) = criar_matriz();

        let mut soma_notas:f32 = 0.0;
        
        for i in 0..num_alunos{
            println!("\n\nAluno {}: ------------------------------ ", vet_nomes[i]);
            
            for j in 0..num_disciplinas{    
                let mut stg_nota = String::new();
                println!("Digite a nota do aluno {},na materia {}: ", vet_nomes[i], j);
                io::stdin().read_line(&mut stg_nota).expect("Erro ao ler valor.");
                let nota:f32 = stg_nota.trim().parse().expect("Erro ao converter valor.");

                soma_notas += nota;
                matriz[i][j] = nota;
            }      
        }let  _média_turma:f32 = soma_notas /(num_alunos as f32 * num_disciplinas as f32) ;
        
        (matriz, _média_turma, num_alunos, num_disciplinas, vet_nomes)
    }
    pub fn verific_nota_ruim(matriz_notas:& Vec<Vec<f32>>, num_alunos:& usize, num_disciplinas:&usize, vet_nomes:&Vec<String>) -> u8{
        let mut _alunos_abaixo_média:u8 = 0;

        for i in 0..*num_alunos{
            for j in 0..*num_disciplinas{
                if matriz_notas[i][j] < 7.0{
                    println!("O aluno {}, na matéria {} ficou abaixo da média com nota: {}.",vet_nomes[i], j, matriz_notas[i][j]);
                    _alunos_abaixo_média += 1;
                }
            }            
        }if _alunos_abaixo_média == 0 {println!("Nenhum aluno obteve nota abaixo da média.");}
        _alunos_abaixo_média
    }
}