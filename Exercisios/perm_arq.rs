use std::{io};
use std::fs::OpenOptions;
use std::io::{Write, Result, Lines};
use chrono::{Local};

enum Permissao {
    Leitura(bool),
    Escrita(bool),
    Execucao(bool),
}
fn registrar_erro(erro: &str, nome: &str) -> Result<()> {

    let hora_atual = Local::now();
    
    // criar arquivo
    let mut arquivo_erros = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("log.txt")?;

    arquivo_erros.write_all(b"Nome: {} / Erro: {} / Data/Hora: {}", nome, erro, hora_atual)?;
    arquivos_erros.flush()?;
    Ok(())

}

fn imprimir_arquivos( arquivos: &Vec<(String, String, Vec<Permissao>)>) {
    for (nome, conteudo, permissoes) in arquivos {
        println!("Nome do arquivo: {}", nome);
        println!("Conteúdo: '{}'", conteudo);
        
        for permissao in permissoes {
            match permissao {
                Permissao::Leitura(permitido) => {
                    println!("Permissao de Leitura: {}", permitido);
                }
                Permissao::Escrita(permitido) => {
                    println!("Permissao de Escrita: {}", permitido);
                }
                Permissao::Execucao(permitido) => {
                    println!("Permissao de Execução: {}", permitido);
                }
            }
        }
        println!("-------------------");
    }
}

fn prints() -> (String, usize){

    println!("0. Ler");
    println!("1. Escrever");
    println!("2. Executar");
    println!("3. Sair");
    println!("Digite uma opção: ");

    let mut opcao = String::new();
    io::stdin().read_line(&mut opcao).expect("Erro ao ler valor.");
    let opcao: usize = opcao.trim().parse().expect("Erro ao converter.");
        
    let mut nome = String::new();

    if opcao != 3{
        println!("Digite o nome do arquivo: ");
        io::stdin().read_line(&mut nome).expect("Erro ao ler valor.");
        let nome: String = nome.trim().parse().expect("Erro ao converter.");
    }   
    return (nome, opcao)
}

fn cruzar(arquivo: &mut (String, String, Vec<Permissao>), opcao: &usize){
    
    match &arquivo.2[*opcao] {
        Permissao::Leitura(permitido) => {
            if !permitido {
                let mut msg = "Você não tem permissão de ler esse arquivo";
                println!("{}", msg);
                registrar_erro(&mut msg, &arquivo.0);
            } else {
                println!("Conteúdo: {}", arquivo.1)
            }
        },
        Permissao::Escrita(permitido) => {
            if !permitido {
                let mut msg ="Você não tem permissão de escrever nesse arquivo";
                println!("{}", msg);
                registrar_erro(&mut msg, &arquivo.0);
            } else {
                println!("Conteúdo: {}", arquivo.1);
                let mut novo_conteudo = String::new();
                println!("Novo conteúdo: {}", novo_conteudo);
                io::stdin().read_line(&mut novo_conteudo).expect("Erro ao ler.");
                arquivo.1 = novo_conteudo;
            }

        },
        Permissao::Execucao(permitido) => {
            if !permitido {
                let mut msg ="Você não tem permissão de executar esse arquivo!";
                println!("{}", msg);
                registrar_erro(&mut msg, &arquivo.0);
            } else {
                println!("Você tem permissão de executar esse arquivo!")
            }
        }
    }
}

fn main() {
    let arquivo1 = (
        String::from("Olá.txt"),
        String::from("Olá, mundo!"),
        vec![Permissao::Leitura(true), Permissao::Escrita(false), Permissao::Execucao(false)]);

    let arquivo2: (String, String, Vec<Permissao>) = (
        String::from("Hello.txt"),
        String::from("Hello world!"),
        vec![Permissao::Leitura(true), Permissao::Escrita(true), Permissao::Execucao(false)]);

    let mut arquivos: Vec<(String, String, Vec<Permissao>)> = vec![arquivo1, arquivo2];
    imprimir_arquivos(&arquivos);
    pub let mut oi: File= File::create_now("ffoo.txt");
    loop {
        let ( _nome, opcao) = prints();

        if opcao == 3 { break; }

        if _nome == "Olá.txt" { 
            cruzar(&mut arquivos[0], &opcao); 
        } else {
            cruzar(&mut arquivos[1], &opcao); 
        }
    }
}