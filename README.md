# radix_converter
Conver a number from one radix to onother one  

## Usage
```
Usage: radix_converter [OPTIONS] <NUM>

Arguments:
  <NUM>

Options:
  -i, --in-base <IN_BASE>    Regard input NUM as IN_BASE based number [default: hex] [possible values: bin, oct, dec, hex]
  -o, --out-base <OUT_BASE>  Convert input NUM into OUT_BASE based number [default: dec] [possible values: bin, oct, dec, hex]
  -a, --all-out              If specified, the input NUM will be converted into all possible bases
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Examples
```
# Convert '0b11111' into hexadecimal integer
radix_converter -i bin -o hex 11111
1f

# Convert '0o701234' into decimal integer
radix_converter -i oct -o dec 701234
230044

# Convert '0xabcde0' into the interger with all possbile bases
radix_converter -i hex -a abcde0
Bin: 101010111100110111100000
Oct: 52746740
Dec: 11259360
Hex: abcde0
```
