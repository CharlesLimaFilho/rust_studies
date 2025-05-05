use std::fs::rename;
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
        println!("3. Alterar dados");
        println!("4. Ler arquivo");
        println!("5. Remover pessoa");
        println!("0. Sair");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        option = buffer.trim();

        match option {
            "1" => {
                let _ = add_person();
            }
            "2" => {
                buffer.clear();
                println!("Digite o CPF: ");
                stdout().flush()?;
                stdin().read_line(&mut buffer)?;
                let cpf: String = buffer.trim().to_string();
                find_person(caminho.to_string(), cpf)?;
            }
            "3" => {
                let mut dados: Vec<String> = Vec::new();
                let mut cont = 0;
                println!("\nCaso haja dados que não deseja alterar, digite -1\n");
                
                while cont < 2 {
                    buffer.clear();
                    match cont {
                        0 => {
                            println!("Digite o CPF: ");
                            stdout().flush()?;
                            stdin().read_line(&mut buffer)?;
                            dados.push(buffer.trim().to_string());
                        }
                        1 => {
                            println!("Digite o novo nome: ");
                            stdout().flush()?;
                            stdin().read_line(&mut buffer)?;
                            dados.push(buffer.trim().to_string());
                        }
                        _ => {}
                    }
                    buffer.clear();
                    cont += 1;
                }
                update_data(caminho.to_string(), dados)?;
            }
            "4" => {
                print_file(caminho.to_string())?;
            }
            "5"=> {
                buffer.clear();
                println!("Digite o CPF: ");
                stdout().flush()?;
                stdin().read_line(&mut buffer)?;
                let cpf: String = buffer.trim().to_string();
                remove_person(caminho.to_string(), cpf)?;
            }
            "0" => {
                println!("Saindo...");
                break;
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
    println!("Pessoa adicionada.");
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


// Vector de strings para armazenar as informações, caso info "-1" não alterar
fn update_data(caminho: String, novas_info: Vec<String>) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(&caminho)?;
    let mut temp_file: File = OpenOptions::new().write(true).create(true).open("output.tmp")?;
    let reader = BufReader::new(file);
    let mut cond: bool = false;

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.to_string();
                if l.contains(novas_info[0].as_str()) {
                    temp_file.write_all(l.as_bytes())?;
                    cond = true;
                }
                if cond {
                    if l == "}" {
                        temp_file.write_all(l.as_bytes())?;
                        temp_file.write_all(b"\n")?;
                        println!("");
                        cond = false;
                    }

                    if l.contains("nome") {
                        let nome: String = format!("\n\tnome: {{ {} }}\n", novas_info[1]);
                        temp_file.write_all(nome.as_bytes())?;
                    }
                } else {
                    temp_file.write_all(l.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                }
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    rename("output.tmp", caminho)?;
    Ok(())

}

fn remove_person(caminho: String, cpf: String) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(&caminho)?;
    let mut temp_file: File = OpenOptions::new().write(true).create(true).open("output.tmp")?;
    let reader = BufReader::new(file);
    let mut deve_remover: bool = false;
    let mut hold: String = String::new();

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.to_string();
                if l == "{" && hold == "" {
                    hold = l.clone();
                } else if l.contains(cpf.as_str()) {
                    deve_remover = true;
                } else if deve_remover && l == "}" {
                    deve_remover = false;
                    hold = String::new();
                } else if !deve_remover && hold != "" {
                    temp_file.write_all(hold.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                    hold = String::new();
                    temp_file.write_all(l.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                } else if !deve_remover {
                    hold = String::new();
                    temp_file.write_all(l.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                }
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    rename("output.tmp", caminho)?;
    Ok(())
}