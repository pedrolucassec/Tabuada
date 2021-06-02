use std::io;
use std::io::Write; // Flush Library

fn main() {
    std::process::Command::new("clear").status().unwrap(); // Limpa a Tela
    print!("\n[Informe o Numero]: ");
    let mut num = String::new(); // Armazena a String
    io::stdout().flush().unwrap(); //
    io::stdin().read_line(&mut num).expect("Failed to read line");
    
    print!("[Onde devo Parar]: ");
    let mut dest = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dest).expect("Failed to read line");


    let _intnum: usize = num.trim().parse().expect("Failed to convert str in int"); // Converte a variável num para inteiro
    let _intdest: usize = dest.trim().parse().expect("Failed to convert str in int");
    std::process::Command::new("clear").status().unwrap();
    
    print!("-------------------\n Tabuada de {}\n-------------------\n\n", _intnum);
    
    for i in 1.._intdest+1{
        println!("| {} x {} = {}", _intnum, i, _intnum*i);
    }
    println!();
}

