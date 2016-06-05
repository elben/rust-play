extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
struct Ingredient {
  name: String,
  quantity: Option<u32>,
  unit: Option<String>,
}

// a lifetime var for Ingredient
#[derive(RustcEncodable)]
struct Recipe<'a> {
  name: String,
  servings: u32,
  ingredients: Vec<&'a Ingredient>,
}

// Sample ingredients:
//
// 8 oz mixed greens
// 2 avocados
// salt
//
fn parse_ingredient(s: &str) -> Ingredient {
    let mut splits: Vec<&str> = s.split(" ").collect();

    // Maybe grab first; if value exists, parse into int, and convert the Result to Option (with
    // ok()).
    let quantity: Option<u32> = splits.first()
        .and_then(|fst| fst.parse::<u32>().ok());

    // TODO need to figure out how we will know if the first element was consumed or not? Or just
    // set varialbe.
    match quantity {
        Some(q) => Ingredient {
            name: splits.split_off(1).join(" "),
            quantity: Some(q),
            unit: None
        },
        None => {
            Ingredient {
                name: s.to_string(),
                quantity: None,
                unit: None
            }
        },
    }
}

fn main() {
    println!("Hello, world!");
    let mut f = File::open("recipes.txt").expect("Failed to open file.");
    let mut s = String::new();
    f.read_to_string(&mut s);
    println!("{}", s);

    let recipes_text = s.split("\n\n")
        .map(|recipe_text| recipe_text.split("\n"));

    // let blob: Vec<&str> = s.split("\n\n").collect();

    // let v: Vec<&str> = s.split("\n\n").map();

    // http://zsiciarz.github.io/24daysofrust/book/day6.html

    let i = Ingredient {
        name: "avocados".to_string(),
        quantity: Some(3),
        unit: None,
    };

    let r = Recipe {
        name: "Avocado Salad".to_string(),
        servings: 10,
        ingredients: vec![&i],
    };

    // encode returns a Result (i.e. Either), which we dangerously unwrap (e.g. get the OK value)
    println!("{}", json::encode(&r).unwrap());
    println!("{}", json::encode(&i).unwrap());
}
