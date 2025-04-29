/*
--- Dia 1: Histeria do Historiador (Parte 2) ---

Sua análise apenas confirmou o que todos temiam: as duas listas de IDs de localização são realmente muito diferentes.

Ou será que são?

Os Historiadores não conseguem concordar sobre qual grupo cometeu os erros ou como ler a maior parte da caligrafia do Chefe, mas na confusão você nota um detalhe interessante: muitos IDs de localização aparecem em ambas as listas! Talvez os outros números não sejam IDs de localização, mas sim caligrafias mal interpretadas.

Desta vez, você precisará descobrir exatamente com que frequência cada número da lista da esquerda aparece na lista da direita. Calcule uma pontuação total de similaridade somando cada número na lista da esquerda depois de multiplicá-lo pelo número de vezes que esse número aparece na lista da direita.

Para o desafio da parte 2, você precisa:
1. Contar quantas vezes cada número da lista 2 aparece
2. Para cada número na lista 1, multiplicá-lo pela frequência correspondente na lista 2
3. Somar todos esses produtos para encontrar a "pontuação de similaridade"
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;  // Import HashMap from the standard library for O(1) lookups
use std::time::Instant; // Importa o módulo Instant para medir o tempo

fn main() {
    let start_time = Instant::now(); // Registra o tempo de início

    // Cria dois vetores explicitamente tipados para inteiros de 32 bits
    let mut lista1: Vec<i32> = Vec::new();
    let mut lista2: Vec<i32> = Vec::new();

    // Cria o caminho do arquivo explicitamente tipado
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
            lista1.push(num1);
            lista2.push(num2);
        } else {
            eprintln!("Pulando linha malformada {}", line);
        }
    }

    // Cria um HashMap chamado freq_map para armazenar a frequência de cada número presente em lista2.
    // A chave do HashMap é o número (i32) e o valor é a quantidade de vezes que esse número aparece (usize).
    // Isso permite consultas rápidas (O(1)) para saber quantas vezes um número de lista1 aparece em lista2.
    let mut freq_map: HashMap<i32, usize> = HashMap::new();

    // Itera sobre cada elemento de lista2 por referência.
    // Para cada número, utiliza o método entry() do HashMap:
    // - Se o número já existe como chave, retorna uma referência mutável para o valor existente.
    // - Se não existe, insere o valor inicial 0 e retorna uma referência mutável para ele.
    // O operador '*freq_map.entry(num).or_insert(0) += 1;' incrementa a contagem desse número.
    for &num in &lista2 {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    // Ao final, freq_map conterá, para cada número de lista2, quantas vezes ele aparece.

    // Calcula a soma em O(n) usando iterator adaptors e closures:
    // - lista1.iter() cria um iterator sobre referências a cada i32
    // - map aplica uma closure que multiplica o número pela sua frequência (convertendo usize para i32)
    // - sum() consome o iterator e retorna a soma de todos os resultados
    let soma: i32 = lista1.iter()
        .map(|&num| num * (*freq_map.get(&num).unwrap_or(&0) as i32))
        .sum();

    println!("Pontuação de Similaridade: {}", soma);

    let duration = start_time.elapsed(); // Calcula a duração total
    println!("Tempo de execução: {:?}", duration);
}