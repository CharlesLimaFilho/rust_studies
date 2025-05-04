use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write}; // Write has write_all()
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let mut option: &str;
    let mut buffer: String = String::new();
    let caminho: String = "output.txt".to_string();
    loop {
        buffer.clear();
        println!("Escolha uma opção:");
        println!("1. Adicionar pessoa");
        println!("2. Buscar pessoa");
        println!("3. Ler arquivo");
        println!("0. Sair");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        option = buffer.trim();

        match option {
            "1" => {
                if let Err(e) = add_person() {
                    eprintln!("Error writing to file: {}", e);
                }
            }
            "0" => {
                println!("Saindo...");
                break;
            }
            "3" => {
                print_file(caminho.to_string())?;
            }
            "2" => {
                buffer.clear();
                println!("Digite o CPF: ");
                stdout().flush()?;
                stdin().read_line(&mut buffer)?;
                let cpf: String = buffer.trim().to_string();
                find_person(caminho.to_string(), cpf)?;
            }
            _ => {
                println!("Opção inválida, tente novamente.");
            }
        }
    }
    Ok(())
}

fn add_person() -> io::Result<()> {
    let mut buffer: String = String::new();
    let mut file: File = OpenOptions::new().create(true).append(true).open("output.txt")?;

    println!("Digite o nome: ");
    stdout().flush()?;
    stdin().read_line(&mut buffer)?;
    let name: String = buffer.trim().to_string();
    buffer.clear();

    println!("Digite o CPF: ");
    stdout().flush()?;
    stdin().read_line(&mut buffer)?;
    let cpf: String = buffer.trim().to_string();
    buffer.clear();


    let message: String = format!("{{\n\tCPF: {{ {} }},\n\tnome: {{ {} }}\n}}\n", cpf, name);


    file.write_all(message.as_bytes())?;
    println!("Message written to file.");
    Ok(())
}

fn find_person(caminho: String, cpf: String) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(caminho)?;
    let reader = BufReader::new(file);
    let mut cond: bool = false;

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.trim();
                if l.contains(cpf.as_str()) || cond {
                    if l == "}" {
                        println!("");
                        return Ok(());
                    }

                    if !cond { println!("") };
                    println!("{}", l);
                    cond = true;
                }
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e);
            }
        }
    }
    println!("Não encontrado");
    return Ok(());
}

fn print_file(caminho: String) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(caminho)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.trim();
                println!("{}", l);
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}