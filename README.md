# Tahwil | تحويل
A blazingly fast currency converter written in Rust.

Tahwil is a simple currency converter tool designed to help users convert between different currencies easily.
## Why Use Tahwil?

  * Simple to Use: Tahwil provides a straightforward command-line interface, making it easy to convert currencies.
  * Fast Conversion: With its integration with the ExchangeRate-API, Tahwil delivers quick and accurate conversion results.
  * Flexible Formatting: You can choose to view the converted amounts in either human-readable format or raw numbers, depending on your preference.

Start using Tahwil today to streamline your currency conversion needs!
## Installation
To install Tahwil, follow these steps:
1. Ensure you have Rust installed on your system. If not, you can install it from [Rust's official website](https://www.rust-lang.org/tools/install).
2. then it's simple as:
```bash
cargo install --git https://github.com/Phant80m/Tahwil/
```

## Usage 
```bash
tahwil <Input Currency> <Output Currency> <Amount> --real <bool>
```
example:
```bash
tahwil AUD USD 20
```
Output:
```bash
20.00 AUD = 13.30 USD
```

the `--raw or `-r` flag displays the number as a raw number, and not the rounded number like `10M` which is default

#### You can also use numbers like `10k` or `10m` eg,
example:
```bash
tahwil AUD GBP 10.5m
```
Output:
```bash
10.50M AUD = 5.47M GBP
```
or with `--raw` 
```bash
10500000 AUD = 5470500 GBP
```
