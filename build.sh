#!/bin/bash

set -e

# Cabeçalho fixo do Cargo.toml
cat <<EOF > Cargo.toml
[package]
name = "advent_of_code_2024"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
regex = "1.10.4"
EOF

echo "" >> Cargo.toml

echo "# Define the binary executables" >> Cargo.toml

# Para cada diretório numerado
for dir in [0-9]*; do
  if [[ -d "$dir" ]]; then
    # Para cada arquivo .rs dentro do diretório
    for file in "$dir"/*.rs; do
      fname=$(basename "$file" .rs)
      # Nome do bin: d<dir>_<fname_sem_extensao_original>
      # Se fname for "3_1", o nome do bin será "d3_1"
      # Se fname for apenas "main" dentro de "1/", o nome do bin será "d1_main"
      bin_name="d${dir}_${fname}"

      echo "" >> Cargo.toml
      echo "[[bin]]" >> Cargo.toml
      # Corrigido para usar o nome do arquivo original no nome do binario, prefixado pelo diretorio
      echo "name = \"d${fname}\"" >> Cargo.toml
      echo "path = \"$dir/$fname.rs\"" >> Cargo.toml
    done
  fi
done

echo "Cargo.toml atualizado!" 

echo "Executando cargo build --release..."
cargo build --release