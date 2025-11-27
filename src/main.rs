#![allow(dead_code)]
// Dead code requis pour les chanps auteur et annee de Livre qui ne sont utilisés
// que via le trait Debug (pour affichage) et causent donc des warnings compilateur

use std::io;

#[derive(Debug)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u16, // u8 limité à 255, donc u16 type le plus opti pour les années (limité à 65535)
    disponible: bool,
}

#[derive(Debug, Default)]
struct Bibliotheque {
    livres: Vec<Livre>,
}

impl Bibliotheque {
    fn ajouter_livre(&mut self, livre: Livre) {
        self.livres.push(livre);
    }
    fn emprunter_livre(&mut self, titre: &str) -> Option<&Livre> {
        self.livres
            .iter_mut()
            .find(|livre| livre.titre == titre && livre.disponible)
            .map(|livre| {
                livre.disponible = false;
                &*livre
            })
    }
    fn retourner_livre(&mut self, titre: &str) -> Option<&Livre> {
        self.livres
            .iter_mut()
            .find(|livre| livre.titre == titre && !livre.disponible)
            .map(|livre| {
                livre.disponible = true;
                &*livre
            })
    }
    fn contient_livre(&self, titre: &str) -> bool {
        self.livres.iter().any(|livre| livre.titre == titre)
    }
    /**
     * Renvoie une liste de pointeurs vers les livres disponibles
     */
    fn livres_disponibles(&self) -> Vec<&Livre> {
        self.livres
            .iter()
            .filter(|livre| livre.disponible)
            .collect()
    }
}

const FAILED_STDIN_READ: &str = "Impossible de lire stdin";

fn prompt_ajouter_livre(bib: &mut Bibliotheque) {
    let mut titre = String::new();
    let mut auteur = String::new();
    let mut annee_str = String::new();

    println!("Titre: ");
    io::stdin().read_line(&mut titre).expect(FAILED_STDIN_READ);

    if bib.contient_livre(titre.trim()) {
        println!("Le livre '{}' existe déjà", titre.trim());
        return;
    }

    println!("Auteur: ");
    io::stdin().read_line(&mut auteur).expect(FAILED_STDIN_READ);

    println!("Année: ");
    io::stdin()
        .read_line(&mut annee_str)
        .expect(FAILED_STDIN_READ);

    let annee: u16 = annee_str
        .trim()
        .parse()
        .expect("L'année doit etre un nombre");

    let nouveau_livre = Livre {
        titre: titre.trim().to_string(),
        auteur: auteur.trim().to_string(),
        annee,
        disponible: true,
    };

    bib.ajouter_livre(nouveau_livre);

    println!("Livre ajouté");
}

fn prompt_emprunter_livre(bib: &mut Bibliotheque) {
    let mut titre = String::new();

    println!("Titre: ");
    io::stdin().read_line(&mut titre).expect(FAILED_STDIN_READ);

    match bib.emprunter_livre(titre.trim()) {
        Some(livre) => println!("Emprunté: {:#?}", livre),
        None => println!("Livre non disponible ou introuvable"),
    }
}

fn prompt_retourner_livre(bib: &mut Bibliotheque) {
    let mut titre = String::new();

    println!("Titre: ");
    io::stdin().read_line(&mut titre).expect(FAILED_STDIN_READ);

    match bib.retourner_livre(titre.trim()) {
        Some(livre) => println!("Vous avez retourné: {:#?}", livre),
        None => println!("Livre introuvable ou déjà disponible"),
    }
}

fn main() {
    let mut bib = Bibliotheque::default();

    bib.livres.push(Livre {
        titre: String::from("Le Petit Prince"),
        auteur: String::from("Antoine de Saint-Exupéry"),
        annee: 1943,
        disponible: true,
    });

    println!("======Bibliothèque======");
    println!("1. Ajouter un livre");
    println!("2. Emprunter un livre");
    println!("3. Retourner un livre");
    println!("4. Afficher tous les livres");
    println!("5. Afficher les livres disponibles");
    println!("6. Quitter");

    loop {
        println!("\nVotre choix:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(FAILED_STDIN_READ);

        // trim pour enlever les espaces et convertir de String en str
        match input.trim() {
            "1" => {
                prompt_ajouter_livre(&mut bib);
            }
            "2" => {
                prompt_emprunter_livre(&mut bib);
            }
            "3" => {
                prompt_retourner_livre(&mut bib);
            }
            "4" => {
                if bib.livres.is_empty() {
                    println!("Aucun livres");
                    continue;
                }

                bib.livres.iter().for_each(|livre| println!("{:#?}", livre));
            }
            "5" => {
                let dispos = bib.livres_disponibles();
                if dispos.is_empty() {
                    println!("Aucun livres");
                    continue;
                }

                dispos.iter().for_each(|livre| println!("{:#?}", livre));
            }
            "6" => {
                break;
            }
            _ => println!("Option invalide"),
        }
    }
}
