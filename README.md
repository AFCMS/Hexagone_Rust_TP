# Hexagone - TP Rust

Deux struct : `Livre` et `Bibliotheque`.

```mermaid
classDiagram
    class Livre {
        +String titre
        +String auteur
        +u16 annee
        +bool disponible
    }
    class Bibliotheque {
        +Vec<Livre> livres
        +ajouter_livre(livre: Livre)
        +emprunter_livre(titre: &str) -> Option<&Livre>
        +retourner_livre(titre: &str) -> Option<&Livre>
        +contient_livre(titre: &str) -> bool
        +livres_disponibles() -> Vec<&Livre>
    }
```

La logique d'affichage et d'input utilisateur est volontairement séparée dans des fonctions dédiées dans un souci de responsabilité unique pour les struct crées.

Pour simplifier la construction de `Bibliotheque` et éviter de passer un tableau vide à chaque fois, on dérive le trait `Default` pour cette struct.

L'affichage des livres se fait simplement en dérivant le trait `Debug` pour la struct `Livre` (ce qui m'a fait désactiver les warnings dead_code).

Pour le fun, j'ai ajouté un mini pipeline GitHub Actions qui compile le projet à chaque push Git, et j'ai ajouté plusieurs optimisations de compilation dans les fichiers `Cargo.toml` et `.cargo/config.toml` (ciblage de l'architecture CPU native, optimisation plus aggressive, etc).
