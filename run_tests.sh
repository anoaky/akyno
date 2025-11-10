#!/bin/zsh

cargo run --example tests
python tests/gen_report.py > tests/reports/report.html
open tests/reports/report.html