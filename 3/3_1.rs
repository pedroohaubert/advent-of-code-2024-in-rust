/* --- Dia 3: Reflita Sobre Isso ---
"Nossos computadores estão com problemas, então não faço ideia se temos algum Historiador Chefe em estoque! Você pode dar uma olhada no depósito, se quiser", diz o atendente levemente aflito da Loja de Aluguel de Tobogãs do Polo Norte. Os Historiadores saem para dar uma olhada.
O atendente se volta para você. "Será que você pode ver por que nossos computadores estão com problemas de novo?"
O computador parece estar tentando rodar um programa, mas sua memória (seu input do desafio) está corrompida. Todas as instruções foram embaralhadas!
Parece que o objetivo do programa é apenas multiplicar alguns números. Ele faz isso com instruções como mul(X,Y), onde X e Y são números de 1 a 3 dígitos. Por exemplo, mul(44,46) multiplica 44 por 46 para obter o resultado 2024. Da mesma forma, mul(123,4) multiplicaria 123 por 4.
No entanto, como a memória do programa foi corrompida, há também muitos caracteres inválidos que devem ser ignorados, mesmo que pareçam parte de uma instrução mul. Sequências como mul(4*, mul(6,9!, ?(12,34), ou mul ( 2 , 4 ) não fazem nada.
Por exemplo, considere a seguinte seção de memória corrompida:
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
Apenas as quatro seções destacadas são instruções reais de mul. Somando o resultado de cada instrução produz 161 (2*4 + 5*5 + 11*8 + 8*5).
Procure na memória corrompida por instruções mul não corrompidas. O que você obtém se somar todos os resultados das multiplicações?
 */
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use regex::Regex;

fn main() {
    // Marca o início da execução para medir o tempo
    let start_time = Instant::now();

    // Define o caminho do arquivo de entrada
    let input_path: &Path = Path::new("3/input.txt");
    
    // Abre o arquivo de entrada
    let file: File = File::open(&input_path).expect("Não foi possível abrir o arquivo");

    // Usa BufReader para uma leitura mais eficiente do arquivo linha por linha
    let reader: BufReader<File> = BufReader::new(file);

    // Variável para acumular a soma total das multiplicações
    let mut total_sum: i32 = 0;
    // Expressão regular para encontrar instruções válidas do tipo mul(X,Y)
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Itera sobre cada linha do arquivo
    for line in reader.lines() {
        let line: String = line.unwrap();

        // Para cada correspondência da regex na linha, extrai os números e soma o produto
        for match_content in pattern.captures_iter(&line) {
            let num1: i32 = match_content[1].parse().unwrap();
            let num2: i32 = match_content[2].parse().unwrap();
            total_sum += num1 * num2;
        }
    }

    // Exibe o resultado final da soma das multiplicações
    println!("A soma de todas as multiplicações é: {}", total_sum);

    // Exibe o tempo de execução do programa
    let duration = start_time.elapsed();
    println!("Tempo de execução: {:?}", duration);
}