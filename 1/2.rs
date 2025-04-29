use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn main(){

    let mut lista1: Vec<i32> =  Vec::new();
    let mut lista2: Vec<i32> =  Vec::new();

    let input_path = Path::new("1/input.txt");
    // Abre o arquivo
    let file = File::open(&input_path).expect("Não foi possível abrir o arquivo");

    let reader = BufReader::new(file);

    for line in reader.lines(){
        // extrai a linha
        let line = line.expect("Não foi possível ler a linha");

        // Divide a linha em partes usando espaços em branco como delimitador.
        // `.split_whitespace()` retorna um iterador sobre as substrings não vazias.
        let parts: Vec<&str> = line.split_whitespace().collect();

        // verifica se a linha tem 2 partes
        if parts.len() == 2 {
            let num1: i32 = parts[0].parse().expect("Não foi possível dar parse no numero 1");
            let num2: i32 = parts[1].parse().expect("Não foi possível dar parse no numero 2");
            lista1.push(num1);
            lista2.push(num2);
        } else {
            eprintln!("Pulando linha malformada {}", line);
        }
    }

    let mut soma: i32 = 0;

    for num in lista1.iter(){
        let contagem = lista2.iter().filter(|&&x| x == *num).count();
        soma += num * contagem as i32; 
    }

    println!("A soma é: {}", soma);

}