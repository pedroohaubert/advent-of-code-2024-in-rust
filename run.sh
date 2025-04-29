#!/bin/bash

# Check if a binary name was provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <binary_name>"
    echo "Example: $0 d1_1"
    exit 1
fi

BINARY="$1"
BINARY_PATH="./target/release/$BINARY"

# Check if the binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Binary not found: $BINARY_PATH"
    echo "Make sure to build it first with: cargo build --release"
    exit 1
fi

echo "Executando: $BINARY"
echo "---------------------------------"

# Execute com o time para capturar estatísticas de memória no macOS
TIME_OUTPUT=$(/usr/bin/time -l $BINARY_PATH 2>&1)
EXITCODE=$?

# Extract maximum resident set size (RSS) in KB (macOS reports in bytes)
PEAK_MEM=$(echo "$TIME_OUTPUT" | grep "maximum resident set size" | awk '{print $1/1024}')
PEAK_MEM_MB=$(echo "scale=2; $PEAK_MEM/1024" | bc)

# Separar a saída do programa e as estatísticas
PROGRAM_OUTPUT=$(echo "$TIME_OUTPUT" | grep -v "maximum resident set size" | grep -v "^[0-9]" | grep -v "real" | grep -v "user" | grep -v "sys" | grep -v "page reclaims" | grep -v "page faults" | grep -v "swaps" | grep -v "average" | grep -v "block" | grep -v "messages" | grep -v "signals" | grep -v "context switches" | grep -v "instructions" | grep -v "cycles" | grep -v "peak memory")

echo "---------------------------------"
echo "RESULTADO DA EXECUÇÃO:"
echo "$PROGRAM_OUTPUT"
echo "---------------------------------"
echo "ESTATÍSTICAS DE DESEMPENHO:"
echo "Pico de memória: $PEAK_MEM KB ($PEAK_MEM_MB MB)"
echo "---------------------------------"
