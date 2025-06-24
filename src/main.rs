use std::io;
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    id: u32,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    name: String,
    items: Vec<TodoItem>,
    created_at: DateTime<Utc>,
    last_modified: DateTime<Utc>,
}

impl TodoItem {
    fn new(id: u32, title: String, description: Option<String>) -> Self {
        TodoItem {
            id,
            title,
            description,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }

    fn mark_incomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }
}

impl TodoList {
    fn new(name: String) -> Self {
        TodoList {
            name,
            items: Vec::new(),
            created_at: Utc::now(),
            last_modified: Utc::now(),
        }
    }

    fn add_item(&mut self, title: String, description: Option<String>) {
        let id = self.items.len() as u32 + 1;
        let item = TodoItem::new(id, title, description);
        self.items.push(item);
        self.last_modified = Utc::now();
    }

    fn remove_item(&mut self, id: u32) -> bool {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            self.items.remove(index);
            self.last_modified = Utc::now();
            true
        } else {
            false
        }
    }

    fn toggle_item(&mut self, id: u32) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            if item.completed {
                item.mark_incomplete();
            } else {
                item.mark_completed();
            }
            self.last_modified = Utc::now();
            true
        } else {
            false
        }
    }

    fn display(&self) {
        println!("\n=== {} ===", self.name);
        println!("Créée le: {}", self.created_at.format("%d/%m/%Y à %H:%M"));
        println!("Dernière modification: {}", self.last_modified.format("%d/%m/%Y à %H:%M"));
        println!("Nombre d'éléments: {}", self.items.len());
        println!();
        
        if self.items.is_empty() {
            println!("Aucun élément dans cette liste.");
        } else {
            for item in &self.items {
                let status = if item.completed { "✓" } else { "□" };
                println!("{} [{}] {}", status, item.id, item.title);
                if let Some(desc) = &item.description {
                    println!("    Description: {}", desc);
                }
                if item.completed {
                    if let Some(completed_at) = item.completed_at {
                        println!("    Terminé le: {}", completed_at.format("%d/%m/%Y à %H:%M"));
                    }
                }
                println!();
            }
        }
    }
}

fn save_todo_list(todo_list: &TodoList) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.json", todo_list.name.replace(" ", "_").to_lowercase());
    let json = serde_json::to_string_pretty(todo_list)?;
    fs::write(filename, json)?;
    println!("Liste '{}' sauvegardée avec succès!", todo_list.name);
    Ok(())
}

fn load_todo_list(name: &str) -> Result<TodoList, Box<dyn std::error::Error>> {
    let filename = format!("{}.json", name.replace(" ", "_").to_lowercase());
    let content = fs::read_to_string(filename)?;
    let todo_list: TodoList = serde_json::from_str(&content)?;
    Ok(todo_list)
}

fn list_saved_todo_lists() -> Vec<String> {
    let mut lists = Vec::new();
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".json") {
                        let name = filename.replace(".json", "").replace("_", " ");
                        lists.push(name);
                    }
                }
            }
        }
    }
    lists
}

fn creer_liste() {
    println!("Entrez le nom de votre nouvelle todo list:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Erreur de lecture");
    let name = name.trim().to_string();
    
    if name.is_empty() {
        println!("Le nom ne peut pas être vide!");
        return;
    }
    
    let mut todo_list = TodoList::new(name.clone());
    
    println!("Liste '{}' créée! Ajoutons quelques éléments:", name);
    
    loop {
        println!("\nEntrez le titre de l'élément (ou 'fin' pour terminer):");
        let mut title = String::new();
        io::stdin().read_line(&mut title).expect("Erreur de lecture");
        let title = title.trim().to_string();
        
        if title.to_lowercase() == "fin" {
            break;
        }
        
        if title.is_empty() {
            println!("Le titre ne peut pas être vide!");
            continue;
        }
        
        println!("Entrez une description (optionnel, appuyez sur Entrée pour passer):");
        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("Erreur de lecture");
        let description = description.trim().to_string();
        
        let desc = if description.is_empty() { None } else { Some(description) };
        todo_list.add_item(title, desc);
        println!("Élément ajouté!");
    }
    
    todo_list.display();
    
    if let Err(e) = save_todo_list(&todo_list) {
        println!("Erreur lors de la sauvegarde: {}", e);
    }
}

fn mettre_a_jour_liste() {
    let lists = list_saved_todo_lists();
    
    if lists.is_empty() {
        println!("Aucune liste sauvegardée trouvée.");
        return;
    }
    
    println!("Listes disponibles:");
    for (i, list_name) in lists.iter().enumerate() {
        println!("{} - {}", i + 1, list_name);
    }
    
    println!("Choisissez le numéro de la liste à modifier:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Erreur de lecture");
    let choice: usize = choice.trim().parse().expect("Veuillez entrer un nombre");
    
    if choice > 0 && choice <= lists.len() {
        let list_name = &lists[choice - 1];
        
        match load_todo_list(list_name) {
            Ok(mut todo_list) => {
                todo_list.display();
                modifier_liste(&mut todo_list);
                if let Err(e) = save_todo_list(&todo_list) {
                    println!("Erreur lors de la sauvegarde: {}", e);
                }
            }
            Err(e) => println!("Erreur lors du chargement: {}", e),
        }
    } else {
        println!("Choix invalide.");
    }
}

fn modifier_liste(todo_list: &mut TodoList) {
    loop {
        println!("\nActions disponibles:");
        println!("1 - Ajouter un élément");
        println!("2 - Marquer un élément comme terminé/non terminé");
        println!("3 - Supprimer un élément");
        println!("4 - Afficher la liste");
        println!("5 - Retour au menu principal");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erreur de lecture");
        let choice: u32 = choice.trim().parse().expect("Veuillez entrer un nombre");
        
        match choice {
            1 => {
                println!("Entrez le titre de l'élément:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Erreur de lecture");
                let title = title.trim().to_string();
                
                if title.is_empty() {
                    println!("Le titre ne peut pas être vide!");
                    continue;
                }
                
                println!("Entrez une description (optionnel):");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Erreur de lecture");
                let description = description.trim().to_string();
                
                let desc = if description.is_empty() { None } else { Some(description) };
                todo_list.add_item(title, desc);
                println!("Élément ajouté!");
            }
            2 => {
                if todo_list.items.is_empty() {
                    println!("La liste est vide!");
                    continue;
                }
                
                todo_list.display();
                println!("Entrez l'ID de l'élément à modifier:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Erreur de lecture");
                let id: u32 = id_input.trim().parse().expect("Veuillez entrer un nombre");
                
                if todo_list.toggle_item(id) {
                    println!("Statut modifié!");
                } else {
                    println!("Élément non trouvé!");
                }
            }
            3 => {
                if todo_list.items.is_empty() {
                    println!("La liste est vide!");
                    continue;
                }
                
                todo_list.display();
                println!("Entrez l'ID de l'élément à supprimer:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Erreur de lecture");
                let id: u32 = id_input.trim().parse().expect("Veuillez entrer un nombre");
                
                if todo_list.remove_item(id) {
                    println!("Élément supprimé!");
                } else {
                    println!("Élément non trouvé!");
                }
            }
            4 => {
                todo_list.display();
            }
            5 => break,
            _ => println!("Choix invalide."),
        }
    }
}

fn supprimer_liste() {
    let lists = list_saved_todo_lists();
    
    if lists.is_empty() {
        println!("Aucune liste sauvegardée trouvée.");
        return;
    }
    
    println!("Listes disponibles:");
    for (i, list_name) in lists.iter().enumerate() {
        println!("{} - {}", i + 1, list_name);
    }
    
    println!("Choisissez le numéro de la liste à supprimer:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Erreur de lecture");
    let choice: usize = choice.trim().parse().expect("Veuillez entrer un nombre");
    
    if choice > 0 && choice <= lists.len() {
        let list_name = &lists[choice - 1];
        let filename = format!("{}.json", list_name.replace(" ", "_").to_lowercase());
        
        println!("Êtes-vous sûr de vouloir supprimer la liste '{}'? (oui/non)", list_name);
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("Erreur de lecture");
        
        if confirm.trim().to_lowercase() == "oui" {
            if let Err(e) = fs::remove_file(filename) {
                println!("Erreur lors de la suppression: {}", e);
            } else {
                println!("Liste '{}' supprimée avec succès!", list_name);
            }
        } else {
            println!("Suppression annulée.");
        }
    } else {
        println!("Choix invalide.");
    }
}

fn main() {
    loop {
        // affichage du menu
        let actions = ["Créer une nouvelle liste", "Mettre à jour une liste existante", "Supprimer une liste existante", "Quitter"];
        display_actions(&actions);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture");

        // convertis l'entrée en nombres
        let choix: u32 = input.trim().parse().expect("Veuillez entrer un nombre");

        // utilisation de match pour exécuter une action selon le choix
        match choix {
            1 => creer_liste(),
            2 => mettre_a_jour_liste(),
            3 => supprimer_liste(),
            4 => {
                println!("Au revoir!");
                break;
            }
            _ => println!("Choix invalide."),
        }
        
        println!("\n\n");
    }
}

// fonction pour afficher le menu
fn display_actions(actions: &[&str]) {
    println!("Bonjour! Choisissez une action parmi ce que vous souhaitez faire :");
    for (i, action) in actions.iter().enumerate() {
        println!("{} - {}", i + 1, action);
    }
}
