// --- Dia 2: Relatórios de Nariz Vermelho ---
// Felizmente, o primeiro local que os Historiadores querem procurar não fica longe do escritório do Historiador Chefe.

// Embora a usina de fusão/fissão nuclear do Rena de Nariz Vermelho não pareça conter nenhum sinal do Historiador Chefe, os engenheiros de lá correm até você assim que o veem. Aparentemente, eles ainda falam sobre a vez em que Rudolph foi salvo por síntese molecular a partir de um único elétron.

// Eles rapidamente acrescentam que - já que você está aqui - eles realmente apreciariam sua ajuda na análise de alguns dados incomuns do reator de Nariz Vermelho. Você se vira para verificar se os Historiadores estão esperando por você, mas eles parecem já ter se dividido em grupos que estão atualmente vasculhando cada canto da instalação. Você se oferece para ajudar com os dados incomuns.

// Os dados incomuns (sua entrada do quebra-cabeça) consistem em muitos relatórios, um relatório por linha. Cada relatório é uma lista de números chamados níveis que são separados por espaços. Por exemplo:

// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9
// Este exemplo de dados contém seis relatórios, cada um contendo cinco níveis.

// Os engenheiros estão tentando descobrir quais relatórios são seguros. Os sistemas de segurança do reator de Nariz Vermelho só podem tolerar níveis que estão gradualmente aumentando ou gradualmente diminuindo. Portanto, um relatório só é considerado seguro se ambas as seguintes condições forem verdadeiras:

// Os níveis são todos crescentes ou todos decrescentes.
// Quaisquer dois níveis adjacentes diferem em pelo menos um e no máximo três.
// No exemplo acima, os relatórios podem ser considerados seguros ou inseguros verificando essas regras:

// 7 6 4 2 1: Seguro porque os níveis estão todos diminuindo em 1 ou 2.
// 1 2 7 8 9: Inseguro porque 2 7 é um aumento de 5.
// 9 7 6 2 1: Inseguro porque 6 2 é uma diminuição de 4.
// 1 3 2 4 5: Inseguro porque 1 3 está aumentando, mas 3 2 está diminuindo.
// 8 6 4 4 1: Inseguro porque 4 4 não é um aumento nem uma diminuição.
// 1 3 6 7 9: Seguro porque os níveis estão todos aumentando em 1, 2 ou 3.
// Assim, neste exemplo, 2 relatórios são seguros.

// Analise os dados incomuns dos engenheiros. Quantos relatórios são seguros?

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let input_path: &Path = Path::new("2/input.txt");

    // Abre o arquivo
    let file: File = File::open(&input_path).expect("Não foi possível abrir o arquivo");

    // Usa BufReader para uma leitura mais eficiente do arquivo linha por linha
    let reader: BufReader<File> = BufReader::new(file);

    let mut safe_reports: i32 = 0;

    for line_result in reader.lines() {
        let line: String = line_result.expect("Não foi possível ler a linha");

        // Em vez de coletar todos os números em um Vec<i32>
        // eu estou criando um iterador que processa cada número por vez.
        // Isso evita a alocação de memória para armazenar todos os números de uma linha,
        // o que fez o programa reduzir o tempo de execução pela metade comparado a utilizando um vetor.
        let mut numbers = line.split_whitespace()
                              .map(|s| s.parse::<i32>());

        // Pega o primeiro número para iniciar o processo de comparação
        // Isso dá um ponto de partida para comparar com os próximos números
        let mut prev_num = match numbers.next() {
            Some(Ok(n)) => n,                // Primeiro número foi analisado com sucesso
            Some(Err(_)) => continue,        // Pula a linha se o primeiro elemento não for um número válido
            None => continue,                // Pula linhas vazias
        };

        // Usamos para definir se o report é seguro ou não
        let mut is_safe: bool = true;
        // Direction armazena a direção: Some(1) para crescente, Some(-1) para decrescente, None se ainda não definido
        let mut direction: Option<i32> = None;

        // Processa cada número restante um por um, sem armazenar todos na memória
        // Isso permite que processemos relatórios de qualquer tamanho com uso mínimo de memória
        for num_result in numbers {
            let current_num = match num_result {
                Ok(n) => n,
                Err(_) => {
                    // Tratamento de erro caso encontre um valor não-numérico na linha
                    is_safe = false;
                    break;
                }
            };

            // Calcula a diferença entre o número atual e o anterior
            let diff = current_num - prev_num;
            let distance = diff.abs();

            // Verifica se a distancia está entre 1 e 3
            // Já garante que diff não é zero
            if distance < 1 || distance > 3 {
                is_safe = false;
                break;
            }

            // Obtém o sinal da diferença: 1 crescente, -1 descrescente
            let current_sign = diff.signum();

            // Se a direção já foi estabelecida, verifica se o sinal atual corresponde
            if let Some(dir) = direction {
                if current_sign != dir {
                    // Se a direção mudar o report é inseguro
                    is_safe = false;
                    break;
                }
            } else {
                // Primeiro par válido, estabelece a direção para o restante do relatório
                direction = Some(current_sign);
            }

            // Atualiza prev_num para a próxima iteração
            prev_num = current_num;
        }

        // Incrementa o contador se o relatório for seguro
        if is_safe {
            safe_reports += 1;
        }
    }

    println!("Quantidade de Reports seguros: {}", safe_reports);
    let duration = start_time.elapsed();
    println!("Tempo de execução: {:?}", duration);
}
