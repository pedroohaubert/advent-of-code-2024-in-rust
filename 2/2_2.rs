// --- Part Two ---
// The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

// The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

// Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

// More of the above example's reports are now safe:

// 7 6 4 2 1: Safe without removing any level.
// 1 2 7 8 9: Unsafe regardless of which level is removed.
// 9 7 6 2 1: Unsafe regardless of which level is removed.
// 1 3 2 4 5: Safe by removing the second level, 3.
// 8 6 4 4 1: Safe by removing the third level, 4.
// 1 3 6 7 9: Safe without removing any level.
// Thanks to the Problem Dampener, 4 reports are actually safe!

// Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?

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
    let file: File = File::open(&input_path).expect("Não foi possível abrir o arquivo");
    let reader: BufReader<File> = BufReader::new(file);

    let mut safe_reports: i32 = 0;

    for line_result in reader.lines() {
        let line: String = line_result.expect("Não foi possível ler a linha");

        let numbers: Vec<i32> = match line.split_whitespace()
                                    .map(|s| s.parse::<i32>())
                                    .collect::<Result<Vec<i32>, _>>() {
            Ok(nums) => nums,
            Err(_) => continue, // Pula linhas inválidas
        };

        // Verifica se já é seguro ou encontra a primeira violação
        match find_first_violation(&numbers) {
            Ok(_) => {
                // Seguro sem remover nenhum nível
                safe_reports += 1;
            }
            Err(violation_index) => {
                // Não é seguro. Tenta remover o elemento no índice da violação
                if check_with_skip(&numbers, violation_index) {
                    safe_reports += 1;
                } 
                // Se não funcionou, tenta remover o *próximo* elemento
                // Verifica se o próximo índice existe antes de tentar
                else if violation_index + 1 < numbers.len() && check_with_skip(&numbers, violation_index + 1) {
                    safe_reports += 1;
                }
                // Se nenhuma das remoções tornou seguro, o relatório é inseguro.
            }
        }
    }

    println!("Quantidade de Reports seguros com o Problema Dampener: {}", safe_reports);
    let duration = start_time.elapsed();
    println!("Tempo de execução: {:?}", duration);
}

// Função que encontra o índice da primeira violação nas regras de segurança.
// Retorna Ok(()) se for seguro, ou Err(index) onde a violação ocorreu (entre index e index+1).
fn find_first_violation(numbers: &[i32]) -> Result<(), usize> {
    if numbers.len() < 2 {
        return Ok(()); // Seguro por padrão se tiver menos de 2 números
    }

    let mut direction: Option<i32> = None;
    
    for i in 0..numbers.len() - 1 {
        let current_num = numbers[i];
        let next_num = numbers[i + 1];
        let diff = next_num - current_num;
        let distance = diff.abs();
        
        // Violação: distância fora de [1, 3]
        if distance < 1 || distance > 3 {
            return Err(i);
        }
        
        let current_sign = diff.signum(); // 1 para crescente, -1 para decrescente
        
        // Se a direção já foi estabelecida, verifica consistência
        if let Some(dir) = direction {
            // Mudança de direção - Violação
            if current_sign != dir {
                return Err(i);
            }
        } else {
            // Primeira diferença válida, estabelece a direção
            direction = Some(current_sign);
        }
    }
    
    Ok(()) // Nenhuma violação encontrada
}

// Função que verifica se um relatório é seguro ignorando o elemento no índice `skip_index`.
// Otimizada para O(n) e O(1) de memória extra.
fn check_with_skip(numbers: &[i32], skip_index: usize) -> bool {
    // Se, após pular um, restarem menos de 2 números, é seguro.
    if numbers.len() < 3 { 
        return true;
    }

    let mut direction: Option<i32> = None;
    let mut prev_num: Option<i32> = None; // Armazena o número anterior *válido* (não pulado)

    for i in 0..numbers.len() {
        // Pula o índice especificado
        if i == skip_index {
            continue;
        }

        let current_num = numbers[i];

        // Se já temos um número anterior válido para comparar...
        if let Some(prev) = prev_num {
            let diff = current_num - prev;
            let distance = diff.abs();

            // Violação: distância fora de [1, 3]
            if distance < 1 || distance > 3 {
                return false;
            }

            let current_sign = diff.signum();

            // Se a direção já foi estabelecida, verifica consistência
            if let Some(dir) = direction {
                 // Violação: mudança de direção
                if current_sign != dir {
                    return false;
                }
            } else {
                 // Primeira diferença válida (pode não ser entre índice 0 e 1 se skip_index for 0 ou 1)
                direction = Some(current_sign);
            }
        }
        
        // Atualiza o número anterior válido para a próxima iteração
        prev_num = Some(current_num);
    }
    
    true // Nenhuma violação encontrada após pular o elemento
}
