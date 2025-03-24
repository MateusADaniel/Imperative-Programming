use std::io;
//DEIXEI EM UM MESMO ARQUIVO PARA FACÍLITAR, NÃO SEI SE POSSO SEPARAR TUDO EM FUNÇÕES ASSIM.
mod modulo_main;
use modulo_main::{ item1_5, item6_10};

fn main() {
    println!("TDE 01 - Sistema de notas (item 1 ao 5)\n");

    //Criar o vetor e adicionar as notas dos alunos.
    let (vet_notas,_vet_nomes, num_alunos) = item1_5::cria_enche_vetor();

    for _i in 0..2{ println!(); if _i ==1{println!("-------------------------------------");}}
    
    //Conclusão.
    for i in 0..vet_notas.len() {
        println!("O aluno {} possui nota: {:.2}", _vet_nomes[i], vet_notas[i]);}
    
    println!("\nA medía das notas ficou: {}", item1_5::media(&vet_notas,&num_alunos));

    let (maior, menor) = item1_5::maior_menor_nota(&vet_notas);
    println!("A maior nota é: {:.2}, e a menor nota é: {:.2}", maior, menor);

    for i in 0..2{println!(); if i == 0{println!("-----------------------------------");}}

/////////////////////////////////////////////////////////////////////////////////////////////////


    println!("TDE 01 – Sistema de notas (6 ao 10)");

    //Criar e adiciona os valores a matriz.
    let (matriz_notas, _média_turma, num_alunos, num_disciplinas, _vet_nomes) = item6_10::encher_matriz();

    for _i in 0..4{ println!(); if _i == 2{println!("Conclusão:-------------------------------------------------");}}

    //verific se alguem está abaixo da média.
    let _alunos_abaixo_média = item6_10::verific_nota_ruim(&matriz_notas, &num_alunos, &num_disciplinas, &_vet_nomes);

    for _i in 0..2{ println!(); if _i == 0{println!("-----------------------------------------------------------");}}
    
    //Descobre a maior nota na disciplina especifica escolhida.
    let mut disciplina_especif = String::new();
    println!("Sobre qual disciplina deseja saber à maior nota (Escolha entre 0..{}): ",num_disciplinas - 1);
    io::stdin().read_line(&mut disciplina_especif).expect("Erro ao ler entrada.");
    let mut disciplina_especif:usize = disciplina_especif.trim().parse().expect("Erro ao converter valor.");

    println!("\nA maior nota na disciplina escolhida({}) foi: {}", disciplina_especif, item6_10::maior_nota_disciplina(&matriz_notas,&disciplina_especif,&num_alunos));

    //Verifica qual a maior e a menor nota da matriz de notas.
    let (maior_nota_geral, menor_nota_geral) = item6_10::maior_menor_nota_geral(&matriz_notas, &num_alunos, &num_disciplinas);

    //Conclusão.
    for _i in 0..2{ println!(); if _i == 1{println!("-------------------------------------------------------------------------");}}
    println!("Resumo com as estatísticas gerais:\n");

    println!("A média da turma ficou: {}.", _média_turma);
    println!("A maior nota da turma foi: {}. E a menor foi: {}.", maior_nota_geral, menor_nota_geral);
    println!("A quantidade de alunos abaixo da média foi de: {}.", _alunos_abaixo_média);
    println!("A maior nota em cada disciplina foi de: ");
    for i in 0..num_disciplinas{
        disciplina_especif = i;
        println!("-Disciplina {i}: {}", item6_10::maior_nota_disciplina(&matriz_notas,&disciplina_especif,&num_alunos)); } 
}