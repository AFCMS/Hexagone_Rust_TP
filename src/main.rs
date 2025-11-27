#![allow(dead_code)]

use std::io;

#[derive(Debug)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u16, // u8 limité à 255, donc u16 type le plus opti pour les années (limité à 65535)
    disponible: bool,
}

#[derive(Debug)]
struct Bibliotheque {
    livres: Vec<Livre>,
}

fn main() {
    println!("Hello, world!");

    let mut bib = Bibliotheque { livres: Vec::new() };

    bib.livres.push(Livre {
        titre: String::from("Le Petit Prince"),
        auteur: String::from("Antoine de Saint-Exupéry"),
        annee: 1943,
        disponible: true,
    });

    println!("{:#?}", bib);

    println!("======Bibliothèque======");
    println!("1. Ajouter un livre");
    println!("2. Emprunter un livre");
    println!("3. Retourner un livre");
    println!("4. Afficher tous les livres");
    println!("5. Afficher les livres disponibles");
    println!("6. Quitter");
    println!("Votre choix:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // trim pour enlever les espaces et convertir de String en str
    match input.trim() {
        "1" => println!("Ajouter un livre"),
        "4" => {
            for livre in &bib.livres {
                println!("{:#?}", livre);
            }
        }
        "5" => {
            for livre in &bib.livres {
                if livre.disponible {
                    println!("{:#?}", livre);
                }
            }
        }
        _ => println!("Option non implémentée"),
    }
}
