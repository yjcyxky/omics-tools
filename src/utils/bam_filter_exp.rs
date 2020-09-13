use regex::Regex;
use rust_htslib::bam::record::CigarStringView;

pub fn remove_whitespace(s: &str) -> String {
  s.chars().filter(|c| !c.is_whitespace()).collect()
}

static CIGAR_EXP: &str = r"^(?P<func>each|sum|sum_ratio)\((?P<variant_type>[MIDNSHP])\)(?P<operator>>|<|>=|<=)(?P<number>\d)";
static COMBINED_CIGAR_EXP: &str = r"^(?P<first>(?P<func>each|sum|sum_ratio)\((?P<variant_type>[MIDNSHP])\)(?P<operator>>|<|>=|<=)(?P<number>\d))(?P<bool_operator>&&|\|\|)(?P<rest>.*)";

pub fn combine_cigar_exp(cigar: &CigarStringView, combined_cigar_exp: &str) -> bool {
  let combined_cigar_exp = &remove_whitespace(combined_cigar_exp)[..];
  let r = Regex::new(COMBINED_CIGAR_EXP).unwrap();
  match r.captures(combined_cigar_exp) {
    Some(caps) => {
      let first = caps.name("first").unwrap().as_str();
      let bool_operator = caps.name("bool_operator").unwrap().as_str();
      let rest = caps.name("rest").unwrap().as_str();

      match bool_operator {
        "&&" => return run_cigar_exp(cigar, first) && combine_cigar_exp(cigar, rest),
        "||" => return run_cigar_exp(cigar, first) || combine_cigar_exp(cigar, rest),
        _ => return true,
      }
    }
    None => return run_cigar_exp(cigar, combined_cigar_exp),
  }
}

// Parse cigar expression from a string
// sum(S) && each(S) || sum_ratio(S)
// sum(S) > 50
pub fn run_cigar_exp(cigar: &CigarStringView, cigar_exp: &str) -> bool {
  let cigar_exp = &remove_whitespace(cigar_exp)[..];
  let r = Regex::new(CIGAR_EXP).unwrap();
  match r.captures(cigar_exp) {
    Some(caps) => {
      let func = caps.name("func").unwrap().as_str();
      let variant_type = caps.name("variant_type").unwrap().as_str();
      let operation = caps.name("operator").unwrap().as_str();
      let number_str = caps.name("number").unwrap().as_str();
      let number = number_str.parse::<u32>().unwrap();

      if func == "each" {
        let value = cigar_all(cigar, variant_type.chars().next().unwrap());
        match operation {
          "<" => return value.iter().all(|&x| x < number),
          ">" => return value.iter().all(|&x| x > number),
          ">=" => return value.iter().all(|&x| x >= number),
          "<=" => return value.iter().all(|&x| x <= number),
          _ => return true,
        }
      } else if func == "sum" {
        let value = cigar_sum(cigar, variant_type.chars().next().unwrap());
        match operation {
          "<" => return value < number,
          ">" => return value > number,
          ">=" => return value >= number,
          "<=" => return value <= number,
          _ => return true,
        }
      } else if func == "sum_ratio" {
        let value = cigar_sum_ratio(
          cigar,
          variant_type.chars().next().unwrap(),
          cigar.iter().map(|cigar| cigar.len()).sum(),
        );
        match operation {
          "<" => return value < number as f64,
          ">" => return value > number as f64,
          ">=" => return value - number as f64 >= 0.0,
          "<=" => return value - number as f64 <= 0.0,
          _ => return true,
        }
      } else {
        return true;
      }
    }
    None => {
      eprintln!("Not valid expression: {:?}", cigar_exp);
      std::process::exit(exitcode::DATAERR);
    }
  }
}

fn cigar_var_sum(cigar: &CigarStringView, variant_type: char) -> u32 {
  return cigar
    .iter()
    .filter(|cigar| cigar.char() == variant_type)
    .map(|cigar| cigar.len())
    .sum();
}

fn cigar_sum(cigar: &CigarStringView, variant_type: char) -> u32 {
  match variant_type {
    'M' => cigar_var_sum(cigar, 'M'),
    'I' => cigar_var_sum(cigar, 'I'),
    'D' => cigar_var_sum(cigar, 'D'),
    'N' => cigar_var_sum(cigar, 'N'),
    'S' => (cigar.leading_softclips() + cigar.trailing_softclips()) as u32,
    'H' => (cigar.leading_hardclips() + cigar.trailing_hardclips()) as u32,
    'P' => cigar_var_sum(cigar, 'P'),
    '=' => cigar_var_sum(cigar, '='),
    'X' => cigar_var_sum(cigar, 'X'),
    _ => 0,
  }
}

fn cigar_sum_ratio(cigar: &CigarStringView, variant_type: char, seq_len: u32) -> f64 {
  f64::from(cigar_sum(cigar, variant_type)) / f64::from(seq_len)
}

fn cigar_all(cigar: &CigarStringView, variant_type: char) -> Vec<u32> {
  return cigar
    .iter()
    .filter(|cigar| cigar.char() == variant_type)
    .map(|cigar| cigar.len())
    .collect();
}
