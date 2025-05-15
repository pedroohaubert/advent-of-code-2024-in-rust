/* --- Part Two ---
As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.

There are two new instructions you'll need to handle:

The do() instruction enables future mul instructions.
The don't() instruction disables future mul instructions.
Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

For example:

xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.

This time, the sum of the results is 48 (2*4 + 8*5).

Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications? */

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use regex::Regex;

fn main() {
    let start_time = Instant::now();
    let input_path: &Path = Path::new("3/input.txt");
    let file: File = File::open(&input_path).expect("Não foi possível abrir o arquivo");
    let reader: BufReader<File> = BufReader::new(file);
    let mut total_sum: i32 = 0;
    let mut is_enabled: bool = true;
    
    // Expressão regular para encontrar tokens: mul(X,Y), do() e don't()
    let token_re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    
    // Itera sobre cada linha do arquivo
    for line in reader.lines() {
        let line = line.unwrap();
        
        // Para cada token encontrado na linha, processa conforme o tipo
        for cap in token_re.captures_iter(&line) {
            if cap.get(1).is_some() {
                // Habilita as próximas instruções mul
                is_enabled = true;
            } else if cap.get(2).is_some() {
                // Desabilita as próximas instruções mul
                is_enabled = false;
            } else if is_enabled {
                // Se for uma instrução mul e estiver habilitado, extrai os números e soma o produto
                let x: i32 = cap[3].parse().unwrap();
                let y: i32 = cap[4].parse().unwrap();
                total_sum += x * y;
            }
        }
    }
    
    println!("Resultado final: {}", total_sum);
    println!("Tempo de execução: {:?}", start_time.elapsed());
}
