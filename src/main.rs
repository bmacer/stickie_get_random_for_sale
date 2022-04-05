use std::env;
use std::process;
use std::fs::File;
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::io::prelude::*;
use rand::seq::SliceRandom;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Change {
    opType: String,
    new: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Nft {
    id: String,
    rootowner: String,
    changes: Vec<Change>,
    collection: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    nfts: HashMap<String, Nft>,
    lastBlock: u32,
}

fn is_for_sale(nft: Nft) -> bool {
    let mut is_for_sale = false;
    for i in nft.changes {
        if i.opType == "SEND"  {
            is_for_sale = false;
        }
        if i.opType == "LIST" {
            if i.new != "0" {
                is_for_sale = true;
            } else {
                is_for_sale = false;
            }
        }
    }
    is_for_sale
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: get-random-listed <dump-file-path> <output-choice-path>");
        println!("Example: get-random-listed backup.link random-choice.txt");
        process::exit(0);
    }
    let file_path = &args[1];
    let ouput_path = &args[2];

    let f = File::open(file_path);
    if f.is_err() {
        println!("File not found");
        println!("File: {}", file_path);
        process::exit(1);
    }
    let mut contents = String::new();
    f.unwrap().read_to_string(&mut contents);
    let d: Data = serde_json::from_str(&contents).unwrap();
    println!("last block: {:?}", d.lastBlock);
    let mut for_sale: Vec<String> = vec![];
    for i in d.nfts {
        if i.1.rootowner == "G9xJaAqygUMmeoTGu4tafGK9LdDbS6k54a3mLHyWydLyUA5" && i.1.collection == "9e5ba1a373b2e45818-STICKIES_OFFICIAL" {
            // println!("I own {:?}", i.1.id);
            if is_for_sale(i.1.clone()) {
                println!("is for sale!");
                for_sale.push(i.1.id);
            }
        }
    }
    println!("all for sale: {:?}", for_sale);
    let mut rng = rand::thread_rng();
    let choice = for_sale.choose(&mut rng).unwrap();
    println!("choice: {}", choice);
    let mut file = File::create(ouput_path).unwrap();
    file.write_all(choice.as_bytes()).unwrap();
}
