# casmap

Mapping sgRNA counts for cas12 6-plex CRISPR screens

## Installation

How to install [cargo](https://rustup.rs/)

```bash
# from crates.io
cargo install casmap

# from github
git clone https://github.com/noamteyssier/casmap
cd casmap
cargo install --path .
```

## Inputs

This requires 2 fastqs - an R1 and a R2. These can be gzipped or plaintext.

This will also require a spacer table which is a 3 column tab-delim table.
The columns represent `[sequence, construct_id, ordering]`. The construct
id and the ordering currently must be numeric.

### Spacer Table

```text
ATGACGAGCTGAGAGCAAGAGCG	0	0
GAAGTCGGGTGGGCGGGGTCATT	0	1
CGCCGCTTCTACATAGTATCGTT	0	2
GAGTTCTGTCCCTCTGCACTTGC	0	3
TTATGAATCTAATGCCCGTCGGA	0	4
TTTAGCTTCGCCTTCGGGATTCA	0	5
GGAGCGAAGTAAACCCGTTGCGA	1	0
TGCAATCACCGCGCTGAGAAATG	1	1
AATGAGCATAAAAGCGATTTAAA	1	2
CATCTGCTCGACTAGTCGGTAAA	1	3
ATCCACGCTGTATACTAAAATTG	1	4
CGCGCACATCATGGTGCTTATCC	1	5
```

### Constant Table

This will also require a constants table representing the static regions
between the variable spacers.
It is a two column tab-delim table representing `[sequence, ordering]`.
Currently the ordering must be numeric.

```text
TACCGTTCACATCGATTTT	0
CGGCCCCATGTGCAAGTAT	1
AAAGAGGCAATTGGTCAAA	2
ATTACAGCCGCAACAGGTC	3
GTGCCCGGTTTAGGTTAAT	4
TGCGAATTTTTGGCTGATC	5
```

## Dummy Data

To have some dummy data to test the interface you can use
my sequence simulator: [casgen](https://github.com/noamteyssier/casgen)

```bash
# install
cargo install casgen

# run
casgen
```

## Usage

### Construct Counting

This will map constructs with exact mathcing on both the spacers and constant regions.

```bash
casmap constructs \
  -i casgen_R1.fastq \
  -I casgen_R2.fastq \
  -s casgen_spacers.tsv \
  -c casgen_constants.tsv
```

### Spacer Characterization

This will write which spacers each read maps against and the number of of each
spacer mapped.

```bash
casmap spacers \
  -i casgen_R1.fastq \
  -I casgen_R2.fastq \
  -s casgen_spacers.tsv
```
