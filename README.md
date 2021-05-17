# Omics Tools

A suite of programs for interacting with high-throughput sequencing data.

## Build Binary
Only for Mac User

```
git clone https://github.com/clinico-omics/omics-tools
cd omics-tools
cargo build --release

# Bam Utility
# ➜ ./target/release/bam-util -h
# Omics Tool Suite - Bam Utility 0.1.0
# Jingcheng Yang <yjcyxky@163.com>
# A suite of programs for interacting with bam file
#
# USAGE:
#     bam-util [FLAGS] [OPTIONS] <SUBCOMMAND>
#
# FLAGS:
#     -h, --help       Prints help information
#     -q, --quiet      A flag which control whether show more messages, true if used in the command line
#     -V, --version    Prints version information
#     -v, --verbose    The number of occurrences of the `v/verbose` flag Verbose mode (-v, -vv, -vvv, etc.)
#
# OPTIONS:
#     -t, --timestamp <ts>    Timestamp(sec, ms, ns, none)
#
# SUBCOMMANDS:
#     filter    Filter Bam file by some flags or indicators
#     help      Prints this message or the help of the given subcommand(s)

# VCF Utility
# ➜ ./target/release/vcf-util -h       
# Omics Tool Suite - VCF Utility 0.1.0
# Jingcheng Yang <yjcyxky@163.com>
# A suite of programs for interacting with vcf file
# 
# USAGE:
#     vcf-util [FLAGS] [OPTIONS] <SUBCOMMAND>
# 
# FLAGS:
#     -h, --help       Prints help information
#     -q, --quiet      A flag which control whether show more messages, true if used in the command line
#     -V, --version    Prints version information
#     -v, --verbose    The number of occurrences of the `v/verbose` flag Verbose mode (-v, -vv, -vvv, etc.)
# 
# OPTIONS:
#     -t, --timestamp <ts>    Timestamp(sec, ms, ns, none)
# 
# SUBCOMMANDS:
#     help      Prints this message or the help of the given subcommand(s)
#     makedb    Convert VCF file to a SQL Database File
```

## Build Jar Package
Only for Mac User

```
git clone https://github.com/clinico-omics/omics-tools
cd omics-tools

make build-jar

# Position of the Jar Package
ls -al ./omics-tools-clj/target/omics-tools-clj-0.1.0-SNAPSHOT.jar
```