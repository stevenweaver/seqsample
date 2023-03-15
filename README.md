Randomly samples from FASTA file

## Installation

#### Source

Download the source code and run

    cargo install

## Usage

```
seqsample -n .5 --fasta original.fas --store-background ./background.fas
```

Arguments: 
```
    -f, --fasta <fasta>
            The input FASTA file (gzip acceptable).

    -h, --help
            Print help information

    -i, --store-background <store-background>
            Write the samples not selected randomly in a separate file.

    -n, --number <number>
            Number of sequences to randomly sample. If the number is less than 1, then it will be
            treated as a percentage of the dataset.

    -V, --version
            Print version information
```
