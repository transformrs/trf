#!/usr/bin/env bash

# Gives writing tips for a given text file.
#
# Usage:
#
#   ./writing-tips.sh <input-file>

set -euo pipefail

# Can also use other keys such as DEEPINFRA_KEY, GOOGLE_KEY, etc.
# trf will automatically use the right API based on the available keys.
export OPENAI_KEY="$(cat /path/to/key)"

if [ "$#" -ne 1 ]; then
    echo "Error: Exactly one input file argument is required"
    echo "Usage: $0 <input-file>"
    exit 1
fi

INPUT_FILE="$1"

PROMPT="
You are a helpful writing assistant.
Respond with a few suggestions for improving the text.
Use plain text only; no markdown.

Here is the text to check:

"
MODEL="meta-llama/Meta-Llama-3-8B-Instruct"

(echo "$PROMPT"; cat "$INPUT_FILE") | cargo run -- --verbose chat --model="$MODEL"