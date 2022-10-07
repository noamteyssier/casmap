
build:
  cargo build --release

run-spacers: build
  target/release/casmap spacers \
    -i example_data/casgen_R1.fastq \
    -I example_data/casgen_R2.fastq \
    -s example_data/casgen_sgrna.tsv

run-constructs: build
  target/release/casmap constructs \
    -i example_data/casgen_R1.fastq \
    -I example_data/casgen_R2.fastq \
    -s example_data/casgen_sgrna.tsv \
    -d example_data/casgen_dr.tsv
