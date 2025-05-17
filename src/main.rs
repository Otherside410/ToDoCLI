use std::io;
fn main() {
    // affichage du menu
    let actions = ["Créer une nouvelle liste", "Mettre à jour une liste existante", "Supprimer une liste existante", "Quitter"];
    display_actions(&actions);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)     // lit la ligne et la met dans `input`
        .expect("Erreur de lecture");                  // Affiche un message si ça plante

    // convertis l'entrée en nombres
    let choix: u32 = input.trim().parse().expect("Veuillez entrer un nombre");

    // utilisation de match pour exécuter une action selon le choix
    match choix {
        1 => {
            println!("Création de liste...");
            // ici tu pourrais appeler une fonction `creer_liste();`
        },
        2 => println!("Quelle liste souhaitez vous mettre à jour ?"),
        3 => println!("Quelle liste souhaitez vous supprimer ?"),
        _ => println!("Choix invalide."),
    }
}

// fonction pour afficher le menu
fn display_actions(actions: &[&str]) {
    println!("Bonjour! Choisissez une action parmis ce que vous souhaitez faire :");
    for (i, actions) in actions.iter().enumerate() {
        println!("{} - {}", i + 1, actions);
    }
}
