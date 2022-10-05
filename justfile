
build:
  cargo build

run: build
  target/debug/casmap \
    -i example_data/casgen_R1.fastq \
    -I example_data/casgen_R2.fastq \
    -s example_data/casgen_sgrna.tsv
