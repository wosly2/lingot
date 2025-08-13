#!/bin/bash

input_dir="$1"
output_file="$2"

> "$output_file"

find "$input_dir" -type f | while read -r file; do
    relpath="${file#$input_dir/}"
    echo "================ $relpath ================" >> "$output_file"
    cat "$file" >> "$output_file"
    echo "" >> "$output_file"
done
