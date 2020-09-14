//! # To provide several functions and boolean operations for filtering bam file.

use log::*;
use regex::Regex;
use rust_htslib::bam::record::CigarStringView;

fn syntax_rule() -> String {
  let func: &str = r"(?P<func>each|sum|sum_ratio)\((?P<variant_type>[MIDNSHP])\)";
  let operation: &str = r"(?P<operator>>|<|>=|<=)";
  let bool_operation: &str = r"(?P<bool_operator>&&|\|\|)";
  let number: &str = r"(?P<number>[1-9]\d*|0)";  // Only support integer
  let rest: &str = r"(?P<rest>.*)";
  let cigar_exp = format!("(?P<first>{}{}{})", func, operation, number);  // each(S) > 10
  return format!("{}{}?{}?", cigar_exp, bool_operation, rest)
}

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

pub fn check_expr(expression: &str) -> bool {
  let expression = &remove_whitespace(expression)[..];
  let r = Regex::new(&syntax_rule()[..]).unwrap();
  match r.captures(expression) {
    Some(caps) => match caps.name("bool_operator") {
      Some(_bool_operator) => {
        let bool_operator = _bool_operator.as_str();
        let rest = caps.name("rest").unwrap().as_str();

        match bool_operator {
          "&&" => return check_expr(rest),
          "||" => return check_expr(rest),
          _ => {
            eprintln!("Not valid expression: {:?}", rest);
            std::process::exit(exitcode::DATAERR);
          }
        }
      }
      None => {
        if caps.name("rest").unwrap().as_str() != "" {
          eprintln!("Not valid expression: {:?}", expression);
          std::process::exit(1);
        } else {
          return true;
        }
      }
    },
    None => {
      eprintln!("Not valid expression: {:?}", expression);
      std::process::exit(exitcode::DATAERR);
    }
  }
}

/// Exec expression for filtering bam file with cigar field.
pub fn exec(cigar: &CigarStringView, expression: &str) -> bool {
  let expression = &remove_whitespace(expression)[..];
  let r = Regex::new(&syntax_rule()[..]).unwrap();
  match r.captures(expression) {
    Some(caps) => {
      let first = caps.name("first").unwrap().as_str();
      match caps.name("bool_operator") {
        Some(_bool_operator) => {
          let bool_operator = _bool_operator.as_str();
          let rest = caps.name("rest").unwrap().as_str();

          debug!(
            "First {}, Bool Operator: {}, Rest: {}",
            first, bool_operator, rest
          );
          match bool_operator {
            "&&" => return exec_single(cigar, first) && exec(cigar, rest),
            "||" => return exec_single(cigar, first) || exec(cigar, rest),
            _ => return false,
          }
        }
        None => exec_single(cigar, first),
      }
    }
    None => {
      eprintln!("Not valid expression: {:?}", expression);
      std::process::exit(exitcode::DATAERR);
    }
  }
}

/// Exec a single expression. e.g. sum(S) > 100 / each(S) > 20 / sum_ratio(S) > 0.5
pub fn exec_single(cigar: &CigarStringView, expression: &str) -> bool {
  let expression = &remove_whitespace(expression)[..];
  let r = Regex::new(&syntax_rule()[..]).unwrap();
  match r.captures(expression) {
    Some(caps) => {
      let func = caps.name("func").unwrap().as_str();
      let variant_type = caps.name("variant_type").unwrap().as_str();
      let operation = caps.name("operator").unwrap().as_str();
      // Only support integer
      let number_str = caps.name("number").unwrap().as_str();
      let number = number_str.parse::<u32>().unwrap();

      if func == "each" {
        let value = len_vector(cigar, variant_type.chars().next().unwrap());
        match operation {
          "<" => return value.iter().all(|&x| x < number),
          ">" => return value.iter().all(|&x| x > number),
          ">=" => return value.iter().all(|&x| x >= number),
          "<=" => return value.iter().all(|&x| x <= number),
          _ => return false,
        }
      } else if func == "sum" {
        let value = dispatch(cigar, variant_type.chars().next().unwrap());
        // debug!("Operation {}, Sum: {}, Number: {}", operation, value, number);

        match operation {
          "<" => return value < number,
          ">" => return value > number,
          ">=" => return value >= number,
          "<=" => return value <= number,
          _ => return false,
        }
      } else if func == "sum_ratio" {
        let value = sum_ratio(
          cigar,
          variant_type.chars().next().unwrap(),
          cigar.iter().map(|cigar| cigar.len()).sum(),
        );

        // debug!("Operation: {}, Sum Ratio: {}, Number: {}", operation, value, number);

        match operation {
          "<" => return value < (number as f64 / 100.0),
          ">" => return value > (number as f64 / 100.0),
          ">=" => return (value - (number as f64 / 100.0)) >= 0.0,
          "<=" => return (value - (number as f64 / 100.0)) <= 0.0,
          _ => return true,
        }
      } else {
        return false;
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
