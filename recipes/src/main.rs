extern crate rustc_serialize;

#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;
use std::collections::HashSet;

lazy_static! {
    static ref UNITS: HashSet<String> = ["oz", "tbs", "tsp", "lbs", "kg", "g", "cups"]
        .iter().map(|s| s.to_string()).collect();
}

// Automatically generate `RustcEncodable` trait implementation
#[derive(RustcEncodable)]
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
    let splits: Vec<&str> = s.split(" ").collect();

    // Maybe grab first; if value exists, parse into int, and convert the Result to Option (with
    // ok()).
    let quantity: Option<u32> = splits.first()
        .and_then(|fst| fst.parse::<u32>().ok());

    let unit: Option<String> = splits
        .get(1)
        .and_then(|u| {
            let us: String = u.to_string();
            if UNITS.contains(&us) {
                Some(us)
            } else {
                None
            }

        });


    // TODO strip out the unit from the name
    let name: String = match quantity {
        None => s.to_string(),
        _    => {
            splits.clone().split_off(1).join(" ")
        },
    };

    Ingredient {
        name: name,
        quantity: quantity,
        unit: unit,
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

    // We create a Recipe here. When we return it, the owner of the Recipe is moved
    // to the caller function.
    //
    // Underneath the hood, what happens? Recipe may be on the stack, and it may be
    // copied into the return value location of the caller (Rust is smart enough to allocate)
    // enough room for the returned value. Or the compiler may optimize this and
    // eliminate the memcpy.
    //
    // http://rustbyexample.com/scope/move.html
    // http://stackoverflow.com/questions/35033806/how-does-rust-deal-with-structs-as-function-parameters-and-return-values
    Recipe {
        name: name.to_string(),
        servings: servings,
        ingredients: ingredients,
    }
}

fn main() {
    println!("Hello, world!");
    let mut f = File::open("recipes.txt").expect("Failed to open file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed to read file.");

    // File content
    println!("{}", contents);

    let recipes: Vec<Recipe> = contents.split("\n\n")
        .map(|recipe_text| parse_recipe(recipe_text) )
        .collect();

    // encode returns a Result (i.e. Either), which we dangerously unwrap (e.g. get the OK value)
    println!("{}", json::as_pretty_json(&recipes));
}
