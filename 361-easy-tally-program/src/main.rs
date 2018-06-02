use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
  let input = "EbAAdbBEaBaaBBdAccbeebaec";
  let mut tally: HashMap<String, i32> = HashMap::new();

  for person in input.chars() {
    let modifier = if person.is_uppercase() {
      -1
    } else {
      1
    };

    let lowercase = person.to_lowercase().to_string();
    *tally.entry(lowercase).or_insert(0) += modifier;
  }

  let mut totals : Vec<(String, i32)> = tally.drain().collect();

  totals.sort_by(|(k1, v1), (k2, v2)| {
    let val_sort = v2.cmp(v1);

    return if val_sort == Ordering::Equal {
      k1.cmp(k2)
    } else {
      val_sort
    }
  });

  println!("{:?}", totals);
}
