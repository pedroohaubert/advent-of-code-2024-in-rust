# Advent of Code 2024 em Rust

Este repositório contém as minhas soluções para o [Advent of Code 2024](https://adventofcode.com/2024), todas implementadas na linguagem [Rust](https://www.rust-lang.org/).

O objetivo é aprender e praticar Rust resolvendo os desafios diários do evento. Cada pasta/diretório corresponde a um dia do Advent of Code, com o código-fonte e explicações comentadas para ajudar no entendimento da linguagem e das soluções.

Sinta-se à vontade para acompanhar, sugerir melhorias ou tirar dúvidas!

Obs: Não sei nada sobre Rust quando iniciando isso

## Estrutura do Projeto

- Pasta `1/`: Soluções para o Dia 1
  - `1_1.rs`: Primeira parte do desafio
  - `1_2.rs`: Segunda parte do desafio
  - `input.txt`: Arquivo de entrada com os dados do desafio
  - `challenge.txt`: Descrição do desafio

## Como compilar e executar

Este projeto utiliza o Cargo, o gerenciador de pacotes e sistema de build do Rust.

### Compilação

Para compilar todos os programas em modo de desenvolvimento:
```bash
cargo build
```

Para compilar com otimizações (recomendado para medição de desempenho):
```bash
cargo build --release
```

### Execução

Para executar um programa específico após compilação (exemplo para o dia 1, desafio 1):
```bash
./target/release/d1_1
```

Alternativamente, você pode compilar e executar em um único comando:
```bash
cargo run --release --bin d1_1
```

Onde os binários estão nomeados no seguinte formato:
- `d1_1`: Dia 1, Parte 1
- `d1_2`: Dia 1, Parte 2
- E assim por diante para os próximos dias

### Desempenho

Os programas incluem medições de tempo de execução, exibidas em microssegundos (µs):
- 1 segundo (s) = 1.000.000 µs
- 1 milissegundo (ms) = 1.000 µs

Para medir o uso de memória, você pode usar ferramentas externas como:
- No macOS/Linux: `/usr/bin/time -l ./target/release/d1_1`
- No macOS (alternativa): `ps -o rss= -p $$ | xargs`
