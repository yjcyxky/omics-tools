//! `Omics-tools` is a suite of programs for interacting with high-throughput sequencing data. e.g. Fastq/Bam/VCF File.

#[macro_use]
extern crate lazy_static;
extern crate vcf as extern_vcf;

pub mod bam;
pub mod vcf;

#[no_mangle]
pub extern "C" fn makedb(input: &str, output: &str) -> Result<Vec<String>, extern_vcf::VCFError> {
  vcf::convertor::makedb(input, output)
}
