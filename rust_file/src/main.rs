use std::fs::rename;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let mut option: &str;
    let mut buffer: String = String::new();
    let caminho: String = "output.txt".to_string();
    let array: [&'static str; 8] = ["CPF", "Nome", "Endereço", "Salário", "Sexo", "Data de nascimento", "Departamento", "Projetos"];

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
                buffer.clear();
                let mut dados: Vec<String> = Vec::new();

                for i in 0..array.len() {
                    buffer.clear();
                    println!("Digite o {}: ", array[i]);
                    stdout().flush()?;
                    stdin().read_line(&mut buffer)?;
                    dados.push(buffer.trim().to_string());
                }
                let _ = add_person(caminho.to_string(), dados);
            }
            "2" => {
                buffer.clear();
                println!("Digite o CPF: ");
                stdout().flush()?;
                stdin().read_line(&mut buffer)?;
                let cpf: String = buffer.trim().to_string();
                let pessoa: String = find_person(caminho.to_string(), cpf);
                println!("{}\n", pessoa);
            }
            "3" => {
                let mut dados: Vec<String> = Vec::new();
                println!("\nCaso haja dados que não deseja alterar, digite -1\n");
                
                for i in 0..array.len() {
                    buffer.clear();
                    println!("Digite o {}: ", array[i]);
                    stdout().flush()?;
                    stdin().read_line(&mut buffer)?;
                    dados.push(buffer.trim().to_string());
                }
                update_data(caminho.to_string(), dados, array)?;
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

fn add_person(caminho: String, dados: Vec<String>) -> io::Result<()> {
    let mut file: File = OpenOptions::new().create(true).append(true).open(&caminho)?;

    
    if find_person(caminho, dados[0].clone()) != "Não encontrado" {
        println!("Pessoa já existe.");
        return Ok(());
    }

    let message: String = format!("{{\n\tCPF: {{ {} }}\n\tNome: {{ {} }}\n\tEndereço: {{ {} }}\n", dados[0], dados[1], dados[2]);
    let message: String = format!("{}\tSalário: {{ R$ {} }}\n\tSexo: {{ {} }}\n\tData de Nascimento: {{ {} }}\n", message, dados[3], dados[4], dados[5]);
    let message: String = format!("{}\tDepartamento: {{ {} }}\n\tProjetos: {{ {} }}\n}}\n", message, dados[6], dados[7]);


    file.write_all(message.as_bytes())?;
    println!("Pessoa adicionada.\n");
    Ok(())
}

fn find_person(caminho: String, cpf: String) -> String {
    let file: File = OpenOptions::new().read(true).open(&caminho).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut cond: bool = false;
    let mut pessoa: String = String::new();
    let encontrar_cpf: String = format!("CPF: {{ {} }}", cpf); 

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.trim();
                if l.contains(encontrar_cpf.as_str()) || cond {
                    if l == "}" {
                        println!("");
                        return pessoa;
                    }

                    if !cond { println!("") };
                    pessoa = format!("{}\t\n{}", pessoa, l);
                    cond = true;
                }
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    return "Não encontrado".to_string();
}

fn print_file(caminho: String) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(caminho)?;
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.trim();
                if l == "}" || l == "{" {
                    println!("");
                } else {
                    println!("{}", l);
                }
                
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}


// Vector de strings para armazenar as informações, caso info "-1" não alterar
fn update_data(caminho: String, novas_info: Vec<String>, texto: [&'static str; 8]) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(&caminho)?;
    let mut temp_file: File = OpenOptions::new().write(true).create(true).open("output.tmp")?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut message: String;
    let encontrar_cpf: String = format!("CPF: {{ {} }}", novas_info[0]);
    let mut deve_alterar: bool = false;
    let mut cont = 0;

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.to_string();
                if l.contains(encontrar_cpf.as_str()) {
                    deve_alterar = true;
                    cont = 0;
                }
                if deve_alterar && (l == "}" || novas_info[cont] != "-1") {
                    if l == "}" {
                        temp_file.write_all(l.as_bytes())?;
                        temp_file.write_all(b"\n")?;
                        println!("");
                        deve_alterar = false;
                        cont = 0;
                    } else {
                        if l.contains("Salário:") {
                            message = format!("\t{}: {{ R$ {} }}\n", texto[cont], novas_info[cont]);
                        } else {
                            message = format!("\t{}: {{ {} }}\n", texto[cont], novas_info[cont]);
                        }
                        temp_file.write_all(message.as_bytes())?;
                    }
                } else {
                    temp_file.write_all(l.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                }
                cont += 1;
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
    let reader: BufReader<File> = BufReader::new(file);
    let mut deve_remover: bool = false;
    let mut hold: String = String::new();
    let encontrar_cpf: String = format!("CPF: {{ {} }}", cpf);

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.to_string();
                if l == "{" && hold == "" {
                    hold = l.clone();
                } else if l.contains(encontrar_cpf.as_str()) {
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