use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    // Cria duas heaps de mínimos explicitamente tipadas para inteiros de 32 bits
    let mut heap1: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut heap2: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    // Cria o caminho do arquivo
    let input_path: &Path = Path::new("1/input.txt");

    // Abre o arquivo
    let file: File = File::open(&input_path).expect("Não foi possível abrir o arquivo");

    let reader: BufReader<File> = BufReader::new(file);

    for line_result in reader.lines() {
        // line_result é um Result<String, std::io::Error>
        let line: String = line_result.expect("Não foi possível ler a linha");

        // Divide a linha em partes usando espaços em branco como delimitador.
        let parts: Vec<&str> = line.split_whitespace().collect();

        // verifica se a linha tem 2 partes
        if parts.len() == 2 {
            // Faz o parse das partes para i32
            let num1: i32 = parts[0].parse().expect("Não foi possível dar parse no numero 1");
            let num2: i32 = parts[1].parse().expect("Não foi possível dar parse no numero 2");
            heap1.push(Reverse(num1));
            heap2.push(Reverse(num2));
        } else {
            eprintln!("Pulando linha malformada {}", line);
        }
    }

    // Variável para acumular a distância total, tipada como i64 para evitar overflow
    let mut distance: i64 = 0;
    let heap_size: usize = heap1.len();

    for _i in 0..heap_size {
        // Pop dos menores elementos das heaps, ambos do tipo i32
        if let Some(Reverse(min_h1)) = heap1.pop() {
            if let Some(Reverse(min_h2)) = heap2.pop() {
                // Calcula a diferença absoluta e soma à distância total
                distance += (min_h1 - min_h2).abs() as i64;
            } else {
                eprintln!("Heap2 está vazio");
            }
        } else {
            eprintln!("Heap1 está vazio");
        }
    }

    println!("Distância total das listas {}", distance);

    let duration = start_time.elapsed();
    println!("Tempo de execução: {:?}", duration);
}
