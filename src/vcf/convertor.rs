// External Library
use flate2::read::MultiGzDecoder;
use log::*;
use regex::Regex;
use vcf::{VCFError, VCFReader, VCFRecord, ValueType};

// Standard Library
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{str, vec::Vec};

// VCF
pub fn get_reader_gz(path: &str) -> Result<VCFReader<BufReader<MultiGzDecoder<File>>>, VCFError> {
  let reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(path)?)));
  return reader;
}

pub fn get_reader(path: &str) -> Result<VCFReader<BufReader<File>>, VCFError> {
  let reader = VCFReader::new(BufReader::new(File::open(path)?));
  return reader;
}

fn infer_info_schema<R: BufRead>(
  reader: &VCFReader<R>,
  enable_prefix: bool,
) -> HashMap<String, String> {
  let mut info_schema = HashMap::new();
  let header = reader.header();
  for key in header.info_list() {
    let info_key = if enable_prefix {
      format!("info_{}", str::from_utf8(&key).unwrap().to_lowercase()).to_string()
    } else {
      str::from_utf8(&key).unwrap().to_lowercase()
    };

    let info_value = match header.info(key).unwrap().value_type {
      ValueType::String => String::from("VARCHAR(32)"),
      ValueType::Integer => String::from("INTEGER"),
      ValueType::Flag => String::from("VARCHAR(32)"),
      ValueType::Character => String::from("VARCHAR(32)"),
      ValueType::Float => String::from("FLOAT"),
      _ => String::from("VARCHAR(32)"),
    };

    info_schema.insert(remove_non_alphabet(&info_key), info_value);
  }

  return info_schema;
}

fn into_info_keys<R: BufRead>(reader: &VCFReader<R>) -> Vec<String> {
  let info_schema = infer_info_schema(reader, false);
  let mut keys = vec![];
  for key in info_schema.keys() {
    keys.push(key.clone());
  }

  return keys;
}

fn into_keys(info_keys: &Vec<String>) -> Vec<String> {
  let mut all_keys = vec![];

  for item in ["chrom", "pos", "id", "ref", "alt", "qual", "filter"].iter() {
    all_keys.push(String::from(*item));
  }

  for item in info_keys {
    all_keys.push(format!("info_{}", item));
  }

  return all_keys;
}

fn into_named_keys(keys: &Vec<String>) -> Vec<String> {
  let mut all_keys = vec![];
  for item in keys {
    all_keys.push(format!(":{}", item));
  }

  return all_keys;
}

fn to_info_map(vcf_record: &VCFRecord, keys: &Vec<String>) -> HashMap<String, String> {
  let mut info = HashMap::new();
  for key in keys {
    let info_value = match vcf_record.info(key.to_uppercase().as_bytes()) {
      Some(info_value) => info_value.to_vec(),
      None => vec![],
    };

    let value = into_vec_u8(&info_value);
    info.insert(
      format!(":info_{}", key),
      String::from_utf8(value).expect("Found invalid UTF-8"),
    );
  }

  return info;
}

fn infer_format_schema<R: BufRead>(reader: &VCFReader<R>) -> HashMap<String, String> {
  let mut info_schema = HashMap::new();
  let header = reader.header();
  for key in header.format_list() {
    let format_key = format!("format_{}", str::from_utf8(&key).unwrap().to_lowercase()).to_string();
    let format_value = match header.format(key).unwrap().value_type {
      ValueType::String => String::from("VARCHAR(32)"),
      ValueType::Integer => String::from("INTEGER"),
      ValueType::Flag => String::from("VARCHAR(32)"),
      ValueType::Character => String::from("VARCHAR(32)"),
      ValueType::Float => String::from("FLOAT"),
      _ => String::from("VARCHAR(32)"),
    };
    info_schema.insert(format_key, format_value);
  }

  return info_schema;
}

pub fn infer_schema<R: BufRead>(reader: &VCFReader<R>) -> HashMap<String, String> {
  let mut schema: HashMap<String, String> = [
    ("chrom", "INTEGER"),
    ("pos", "INTEGER"),
    ("id", "VARCHAR(32)"),
    ("ref", "VARCHAR(32)"),
    ("alt", "VARCHAR(32)"),
    ("qual", "INTEGER"),
    ("filter", "VARCHAR(128)"),
  ]
  .iter()
  .map(|item| (String::from(item.0), String::from(item.1)))
  .collect();

  let info_schema = infer_info_schema(reader, true);
  // let format_schema = infer_format_schema(reader);

  schema.extend(info_schema);
  // schema.extend(format_schema);

  return schema;
}

fn into_vec_u8(items: &Vec<Vec<u8>>) -> Vec<u8> {
  let mut record = vec![];
  for item in items {
    for i in item {
      record.push(i.clone());
    }
  }

  return record;
}

fn f64_into_vec_u8(value: std::option::Option<f64>) -> Vec<u8> {
  match value {
    None => vec![],
    Some(i) => format!("{}", i).into_bytes(),
  }
}

fn f64_into_string(value: std::option::Option<f64>) -> String {
  match value {
    None => String::from(""),
    Some(i) => format!("{}", i),
  }
}

fn into_string(items: &Vec<Vec<u8>>) -> String {
  return items.into_iter().flatten().map(|c| *c as char).collect();
}

fn vec_u8_to_string(items: &Vec<u8>) -> String {
  return items.into_iter().map(|c| *c as char).collect();
}

pub fn into_row_map(vcf_record: &VCFRecord, info_keys: &Vec<String>) -> HashMap<String, String> {
  let mut record: HashMap<String, String> = HashMap::new();
  record.insert(
    String::from(":chrom"),
    vec_u8_to_string(&vcf_record.chromosome),
  );
  record.insert(String::from(":pos"), vcf_record.position.to_string());
  record.insert(String::from(":id"), into_string(&vcf_record.id));
  record.insert(
    String::from(":ref"),
    vec_u8_to_string(&vcf_record.reference),
  );
  record.insert(String::from(":alt"), into_string(&vcf_record.alternative));
  record.insert(String::from(":qual"), f64_into_string(vcf_record.qual));
  record.insert(String::from(":filter"), into_string(&vcf_record.filter));

  record.extend(to_info_map(vcf_record, info_keys));
  // record.extend(to_info_map(&vcf_record, format_keys));

  return record;
}

fn remove_non_alphabet(str: &str) -> String {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
  }

  String::from(RE.replace_all(str, "_").trim_end_matches("_"))
}

// SQLite
fn format_ctable(schema: &HashMap<String, String>) -> String {
  let ctable_prefix = "CREATE TABLE variant (";
  let ctable_suffix = ")";
  let mut ctable_content = String::new();
  for (key, value) in schema {
    ctable_content.push_str(format!("{} {}, ", key, value).as_str());
  }

  return String::from(format!(
    "{} {} {}",
    ctable_prefix,
    ctable_content.trim_end_matches(", "),
    ctable_suffix
  ));
}

pub fn create_table(db: &mut rusqlite::Connection, schema: &HashMap<String, String>) {
  let ctable = format_ctable(schema);
  info!("Create Table: {}", ctable);
  db.execute(&ctable[..], &[] as &[&dyn rusqlite::types::ToSql])
    .unwrap();
}

fn format_insert_by_keys(keys: &Vec<String>) -> String {
  let joined_keys = keys
    .into_iter()
    .map(|key| key.clone())
    .collect::<Vec<_>>()
    .join(", ");

  let values = keys
    .into_iter()
    .map(|key| format!(":{}", key))
    .collect::<Vec<_>>()
    .join(",");

  let insert_query = format!(
    "INSERT INTO {} ({}) VALUES ({})",
    "variant", joined_keys, values
  );

  return insert_query;
}

fn format_insert(row: &HashMap<String, String>) -> String {
  let keys = row
    .keys()
    .into_iter()
    .map(|key| key.clone())
    .collect::<Vec<_>>()
    .join(", ");
  let values = row
    .keys()
    .into_iter()
    .enumerate()
    .map(|(idx, _)| format!("?{}", idx + 1))
    .collect::<Vec<_>>()
    .join(",");

  let insert_query = format!("INSERT INTO {} ({}) VALUES ({})", "variant", keys, values);

  return insert_query;
}

pub fn insert_row(
  db: &mut rusqlite::Connection,
  row: &HashMap<String, String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let insert_query = format_insert(row);
  let tx = db.transaction().unwrap();
  let row_keys: Vec<String> = row.keys().into_iter().map(|item| item.clone()).collect();
  let row_values: Vec<String> = row.values().map(|item| item.clone()).collect();

  debug!("Insert: {}", insert_query);
  debug!("Row Keys: {:?}", row_keys);
  debug!("Row Values: {:?}", row_values);

  {
    let mut stmt = tx.prepare(&insert_query).expect("tx.prepare() failed");
    stmt.execute(&row_values).unwrap();
  }

  tx.commit().unwrap();
  Ok(row_keys)
}

pub fn insert_rows<'a, R: BufRead>(
  db: &mut rusqlite::Connection,
  reader: &mut VCFReader<R>,
) -> Result<Vec<String>, vcf::VCFError> {
  let tx = db.transaction().unwrap();
  let mut vcf_record = reader.empty_record();

  let info_keys = into_info_keys(&reader);

  let keys = into_keys(&info_keys);
  let named_keys = into_named_keys(&keys);
  let named_keys_str: Vec<&str> = named_keys.iter().map(|s| &**s).collect();

  let insert_query = format_insert_by_keys(&keys);
  debug!("Insert: {}", insert_query);
  debug!("Row Keys: {:?}", keys);

  {
    let mut stmt = tx.prepare(&insert_query).expect("tx.prepare() failed");

    while reader.next_record(&mut vcf_record)? {
      let m = into_row_map(&vcf_record, &info_keys);

      let converted_values: Vec<(&str, &dyn rusqlite::ToSql)> = named_keys_str
        .iter()
        .map(|&k| match m.get(k) {
          Some(_) => m.get(k).map(|v| (k, v as &dyn rusqlite::ToSql)).unwrap(),
          None => (k, &"" as &dyn rusqlite::ToSql),
        })
        .collect::<Vec<_>>();

      stmt.execute_named(&converted_values[..]).unwrap();
    }
  }

  tx.commit().unwrap();

  Ok(vec![])
}
