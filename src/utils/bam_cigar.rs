use regex::Regex;
use rust_htslib::bam::record::CigarStringView;

static CIGAR_EXP: &str = r"^(?P<func>each|sum|sum_ratio)\((?P<variant_type>[MIDNSHP])\)(?P<operator>>|<|>=|<=)(?P<number>\d)";
static COMBINED_CIGAR_EXP: &str = r"^(?P<first>(?P<func>each|sum|sum_ratio)\((?P<variant_type>[MIDNSHP])\)(?P<operator>>|<|>=|<=)(?P<number>\d))(?P<bool_operator>&&|\|\|)(?P<rest>.*)";

/// Remove whitespace from a string
/// 
/// # Examples
/// 
/// ```
/// let removed = remove_whitespace("  each(S) && all(M)");
/// assert_eq!("each(S)&&all(M)", removed);
/// ```
pub fn remove_whitespace(s: &str) -> String {
  s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Exec expression for filtering bam file with cigar field.
pub fn exec(cigar: &CigarStringView, expression: &str) -> bool {
  let expression = &remove_whitespace(expression)[..];
  let r = Regex::new(COMBINED_CIGAR_EXP).unwrap();
  match r.captures(expression) {
    Some(caps) => {
      let first = caps.name("first").unwrap().as_str();
      let bool_operator = caps.name("bool_operator").unwrap().as_str();
      let rest = caps.name("rest").unwrap().as_str();

      match bool_operator {
        "&&" => return exec_single(cigar, first) && exec(cigar, rest),
        "||" => return exec_single(cigar, first) || exec(cigar, rest),
        _ => return true,
      }
    }
    None => return exec_single(cigar, expression),
  }
}

/// Exec a single expression. e.g. sum(S) > 100 / each(S) > 20 / sum_ratio(S) > 0.5
pub fn exec_single(cigar: &CigarStringView, expression: &str) -> bool {
  let expression = &remove_whitespace(expression)[..];
  let r = Regex::new(CIGAR_EXP).unwrap();
  match r.captures(expression) {
    Some(caps) => {
      let func = caps.name("func").unwrap().as_str();
      let variant_type = caps.name("variant_type").unwrap().as_str();
      let operation = caps.name("operator").unwrap().as_str();
      let number_str = caps.name("number").unwrap().as_str();
      let number = number_str.parse::<u32>().unwrap();

      if func == "each" {
        let value = len_vector(cigar, variant_type.chars().next().unwrap());
        match operation {
          "<" => return value.iter().all(|&x| x < number),
          ">" => return value.iter().all(|&x| x > number),
          ">=" => return value.iter().all(|&x| x >= number),
          "<=" => return value.iter().all(|&x| x <= number),
          _ => return true,
        }
      } else if func == "sum" {
        let value = dispatch(cigar, variant_type.chars().next().unwrap());
        match operation {
          "<" => return value < number,
          ">" => return value > number,
          ">=" => return value >= number,
          "<=" => return value <= number,
          _ => return true,
        }
      } else if func == "sum_ratio" {
        let value = sum_ratio(
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
      eprintln!("Not valid expression: {:?}", expression);
      std::process::exit(exitcode::DATAERR);
    }
  }
}

fn dispatch(cigar: &CigarStringView, variant_type: char) -> u32 {
  match variant_type {
    'M' => sum_by(cigar, 'M'),
    'I' => sum_by(cigar, 'I'),
    'D' => sum_by(cigar, 'D'),
    'N' => sum_by(cigar, 'N'),
    'S' => (cigar.leading_softclips() + cigar.trailing_softclips()) as u32,
    'H' => (cigar.leading_hardclips() + cigar.trailing_hardclips()) as u32,
    'P' => sum_by(cigar, 'P'),
    '=' => sum_by(cigar, '='),
    'X' => sum_by(cigar, 'X'),
    _ => 0,
  }
}

/// Function
fn sum_by(cigar: &CigarStringView, variant_type: char) -> u32 {
  return cigar
    .iter()
    .filter(|cigar| cigar.char() == variant_type)
    .map(|cigar| cigar.len())
    .sum();
}

fn sum_ratio(cigar: &CigarStringView, variant_type: char, seq_len: u32) -> f64 {
  f64::from(dispatch(cigar, variant_type)) / f64::from(seq_len)
}

fn len_vector(cigar: &CigarStringView, variant_type: char) -> Vec<u32> {
  return cigar
    .iter()
    .filter(|cigar| cigar.char() == variant_type)
    .map(|cigar| cigar.len())
    .collect();
}
