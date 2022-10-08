
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
    -c example_data/casgen_constants.tsv

profile-constructs: build
  cargo flamegraph -F 1000000 -- constructs \
    -i example_data/casgen_R1.fastq \
    -I example_data/casgen_R2.fastq \
    -s example_data/casgen_spacers.tsv \
    -c example_data/casgen_constants.tsv

clean:
  rm -rv \
    constructs.fa \
    construct_counts.tsv \
    flamegraph.svg \
    perf.data*
