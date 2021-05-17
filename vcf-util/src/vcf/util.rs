use regex::Regex;

pub fn is_vcf_file(filepath: &str) -> bool {
  // Import at the crate root - preqc-pack.rs
  lazy_static! {
    static ref RE: Regex = Regex::new(".*(.gvcf|.vcf)$").unwrap();
  }

  RE.is_match(filepath)
}

pub fn is_vcf_gz_file(filepath: &str) -> bool {
  // Import at the crate root - preqc-pack.rs
  lazy_static! {
    static ref RE: Regex = Regex::new(".*(.vcf.gz|.gvcf.gz)$").unwrap();
  }

  RE.is_match(filepath)
}