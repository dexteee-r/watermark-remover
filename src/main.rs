//! Point d’entrée du programme watermark‑remover
//! 
//! Ce fichier appelle les fonctions du module `lib` et gère
//! les arguments CLI simples (entrée, watermark, sortie).

use std::env;
use watermark_remover::lib::{clean_watermark, load_text, write_output};

fn main() {
    // 1️⃣ Récupération des arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: watermark_remover <input> <watermark> <output>");
        std::process::exit(1);
    }
    let input_path = &args[1];
    let watermark = &args[2];
    let output_path = &args[3];

    // 2️⃣ Chargement du fichier
    let raw_text = load_text(input_path).expect("Impossible de lire le fichier d’entrée");
    // 3️⃣ Nettoyage du watermark
    let cleaned = clean_watermark(&raw_text, watermark);
    // 4️⃣ Écriture du fichier nettoyé
    write_output(&cleaned, output_path).expect("Impossible d’écrire le fichier de sortie");

    println!("✅ Fichier nettoyé créé : {}", output_path);
}
