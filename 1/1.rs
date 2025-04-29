use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();

    // Cria o caminho do arquivo
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
            heap1.push(Reverse(num1));
            heap2.push(Reverse(num2));
        } else {
            eprintln!("Pulando linha malformada {}", line);
        }
    }

    let mut distance: i64 = 0;
    let heap_size = heap1.len();

    for _i in 0..heap_size{
        if let Some(Reverse(min_h1)) = heap1.pop(){
            if let Some(Reverse(min_h2)) = heap2.pop(){
                distance += (min_h1 - min_h2).abs() as i64;
            } else {
                eprintln!("Heap2 está vazio");
            }
        } else {
            eprintln!("Heap1 está vazio");
        }
    }

    println!("Distância total das listas {}", distance);

}
