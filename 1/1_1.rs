/*
--- Dia 1: Histeria do Historiador ---

O Historiador Chefe está sempre presente no grande lançamento do trenó de Natal, mas ninguém o vê há meses! A última vez que alguém soube dele, ele estava visitando locais historicamente significativos para o Polo Norte; um grupo de Historiadores Seniores pediu que você os acompanhasse enquanto verificam os lugares onde acham que ele provavelmente visitou.

À medida que cada local é verificado, eles o marcarão em sua lista com uma estrela. Eles acham que o Historiador Chefe deve estar em um dos primeiros cinquenta lugares que procurarão, então, para salvar o Natal, você precisa ajudá-los a conseguir cinquenta estrelas em sua lista antes que o Papai Noel parta em 25 de dezembro.

Colete estrelas resolvendo quebra-cabeças. Dois quebra-cabeças serão disponibilizados em cada dia no calendário do Advento; o segundo quebra-cabeça é desbloqueado quando você completa o primeiro. Cada quebra-cabeça concede uma estrela. Boa sorte!

Você nem saiu ainda e o grupo de Historiadores Seniores Elfos já encontrou um problema: sua lista de locais para verificar está atualmente vazia. Eventualmente, alguém decide que o melhor lugar para verificar primeiro seria o escritório do Historiador Chefe.

Ao entrar no escritório, todos confirmam que o Historiador Chefe realmente não está em lugar nenhum. Em vez disso, os Elfos descobrem uma variedade de notas e listas de locais historicamente significativos! Parece ser o planejamento que o Historiador Chefe estava fazendo antes de partir. Talvez essas notas possam ser usadas para determinar quais locais procurar?

Em todo o escritório do Chefe, os locais historicamente significativos são listados não por nome, mas por um número único chamado ID de localização. Para garantir que não percam nada, os Historiadores se dividem em dois grupos, cada um procurando pelo escritório e tentando criar sua própria lista completa de IDs de localização.

Há apenas um problema: ao colocar as duas listas lado a lado (sua entrada do quebra-cabeça), rapidamente fica claro que as listas não são muito semelhantes. Talvez você possa ajudar os Historiadores a reconciliar suas listas?

Para o desafio da parte 1, você precisa:
1. Parear os números das duas listas (o menor da lista 1 com o menor da lista 2, o segundo menor da lista 1 com o segundo menor da lista 2, etc)
2. Para cada par, calcular a distância absoluta entre os números
3. Somar todas essas distâncias para encontrar a "distância total" entre as listas
*/

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

    println!("Distância total entre as listas: {}", distance);

    let duration = start_time.elapsed();
    println!("Tempo de execução: {:?}", duration);
}
