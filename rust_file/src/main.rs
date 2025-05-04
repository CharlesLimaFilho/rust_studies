use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write}; // Write has write_all()
use std::io::stdin;
use std::io::stdout;

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let mut file: File = OpenOptions::new().write(true).create(true).append(true).open("output.txt")?;
    //let name: String = "Jorge da Silva".to_string();
    //let cpf: String = "024.741.852-88".to_string();

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


    let message: String = format!("nome: {{ {} }},\nCPF: {{ {} }}\n", name, cpf);


    file.write_all(message.as_bytes())?;
    println!("Message written to file.");

    Ok(())
}