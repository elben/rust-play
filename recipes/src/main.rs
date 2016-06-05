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

#[derive(RustcEncodable)]
struct Recipe {
  name: String,
  servings: u32,
  ingredients: Vec<Ingredient>,
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

fn parse_recipe(recipe_text: &str) -> Recipe {
    // Assume lines looks like: [name, servings, ...ingredients]
    let mut lines = recipe_text.split("\n");
    let name = lines.next().unwrap(); // Grab name
    let servings: u32 = lines.next() // Grab serving size from "serves 10" string
        .unwrap().split(" ")
        .nth(1).unwrap().parse().unwrap();

    let ingredients: Vec<Ingredient> = lines
      .collect::<Vec<&str>>()
      .split_off(2).iter()
      .map(|l| parse_ingredient(l)).collect();

    Recipe {
        name: name.to_string(),
        servings: servings,
        ingredients: ingredients,
    }
}

fn main() {
    println!("Hello, world!");
    let mut f = File::open("recipes.txt").expect("Failed to open file.");
    let mut s = String::new();
    f.read_to_string(&mut s);
    println!("{}", s);

    let recipes: Vec<Recipe> = s.split("\n\n")
        .map(|recipe_text| parse_recipe(recipe_text) )
        .collect();

    // encode returns a Result (i.e. Either), which we dangerously unwrap (e.g. get the OK value)
    println!("{}", json::encode(&recipes).unwrap());
}
