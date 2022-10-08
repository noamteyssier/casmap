
build:
  cargo build --release

run-spacers: build
  target/release/casmap spacers \
    -i example_data/casgen_R1.fastq \
    -I example_data/casgen_R2.fastq \
    -s example_data/casgen_spacers.tsv

run-constructs: build
  target/release/casmap constructs \
    -i example_data/casgen_R1.fastq \
    -I example_data/casgen_R2.fastq \
    -s example_data/casgen_spacers.tsv \
    -d example_data/casgen_constants.tsv

run-cas12: build
  target/release/casmap constructs \
    -i chris_example/Subset_R1.fq.gz \
    -I chris_example/Subset_R2.fq.gz \
    -s chris_example/spacer_table.tsv \
    -d chris_example/constants.txt
