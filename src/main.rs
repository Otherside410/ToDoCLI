use std::io;
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate, Local};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn to_string(&self) -> &'static str {
        match self {
            Priority::Low => "Basse",
            Priority::Medium => "Moyenne",
            Priority::High => "Haute",
            Priority::Critical => "Critique",
        }
    }

    fn to_symbol(&self) -> &'static str {
        match self {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🟠",
            Priority::Critical => "🔴",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum TaskStatus {
    Afaire,
    EnCours,
    EnAttente,
    Terminee,
}

impl TaskStatus {
    fn to_string(&self) -> &'static str {
        match self {
            TaskStatus::Afaire => "À faire",
            TaskStatus::EnCours => "En cours",
            TaskStatus::EnAttente => "En attente",
            TaskStatus::Terminee => "Terminée",
        }
    }
    fn to_symbol(&self) -> &'static str {
        match self {
            TaskStatus::Afaire => "⬜",
            TaskStatus::EnCours => "🟦",
            TaskStatus::EnAttente => "🟨",
            TaskStatus::Terminee => "✅",
        }
    }
    fn is_done(&self) -> bool {
        matches!(self, TaskStatus::Terminee)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoItem {
    id: u32,
    title: String,
    description: Option<String>,
    status: TaskStatus,
    priority: Priority,
    due_date: Option<NaiveDate>,
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
    fn new(id: u32, title: String, description: Option<String>, priority: Priority, due_date: Option<NaiveDate>) -> Self {
        TodoItem {
            id,
            title,
            description,
            status: TaskStatus::Afaire,
            priority,
            due_date,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    fn mark_completed(&mut self) {
        self.status = TaskStatus::Terminee;
        self.completed_at = Some(Utc::now());
    }

    fn mark_incomplete(&mut self) {
        self.status = TaskStatus::Afaire;
        self.completed_at = None;
    }

    fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            if self.status.is_done() {
                return false;
            }
            let today = Local::now().date_naive();
            due_date < today
        } else {
            false
        }
    }

    fn days_until_due(&self) -> Option<i64> {
        if let Some(due_date) = self.due_date {
            let today = Local::now().date_naive();
            Some((due_date - today).num_days())
        } else {
            None
        }
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

    fn next_id(&self) -> u32 {
        self.items
            .iter()
            .map(|item| item.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }

    fn add_item(&mut self, title: String, description: Option<String>) {
        let id = self.next_id();
        let item = TodoItem::new(id, title, description, Priority::Low, None);
        self.items.push(item);
        self.last_modified = Utc::now();
    }

    fn add_item_with_details(&mut self, title: String, description: Option<String>, priority: Priority, due_date: Option<NaiveDate>) {
        let id = self.next_id();
        let item = TodoItem::new(id, title, description, priority, due_date);
        self.items.push(item);
        self.last_modified = Utc::now();
    }

    fn add_item_with_details_status(&mut self, title: String, description: Option<String>, status: TaskStatus, priority: Priority, due_date: Option<NaiveDate>) {
        let id = self.next_id();
        let item = TodoItem {
            id,
            title,
            description,
            status,
            priority,
            due_date,
            created_at: Utc::now(),
            completed_at: if status.is_done() { Some(Utc::now()) } else { None },
        };
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
            if item.status.is_done() {
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

    fn update_item_priority(&mut self, id: u32, priority: Priority) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.priority = priority;
            self.last_modified = Utc::now();
            true
        } else {
            false
        }
    }

    fn update_item_due_date(&mut self, id: u32, due_date: Option<NaiveDate>) -> bool {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.due_date = due_date;
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
            // Trier les éléments par priorité (critique en premier) puis par date d'échéance
            let mut sorted_items = self.items.clone();
            sorted_items.sort_by(|a, b| {
                // D'abord par priorité (Critical > High > Medium > Low)
                let priority_cmp = (b.priority as u8).cmp(&(a.priority as u8));
                if priority_cmp != std::cmp::Ordering::Equal {
                    return priority_cmp;
                }
                
                // Puis par date d'échéance (plus tôt en premier)
                match (a.due_date, b.due_date) {
                    (Some(a_date), Some(b_date)) => a_date.cmp(&b_date),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });

            for item in &sorted_items {
                let status_symbol = item.status.to_symbol();
                let status_text = item.status.to_string();
                let priority_symbol = item.priority.to_symbol();
                let priority_text = item.priority.to_string();
                println!("{} [{}] {} ({}) - {} {}", status_symbol, item.id, item.title, priority_text, status_text, priority_symbol);
                
                if let Some(desc) = &item.description {
                    println!("    Description: {}", desc);
                }
                
                if let Some(due_date) = item.due_date {
                    let date_str = due_date.format("%d/%m/%Y").to_string();
                    if item.is_overdue() {
                        println!("    ⚠️  ÉCHÉANCE DÉPASSÉE: {}", date_str);
                    } else if let Some(days) = item.days_until_due() {
                        if days == 0 {
                            println!("    📅 Échéance: {} (AUJOURD'HUI!)", date_str);
                        } else if days == 1 {
                            println!("    📅 Échéance: {} (DEMAIN)", date_str);
                        } else if days < 7 {
                            println!("    📅 Échéance: {} (dans {} jours)", date_str, days);
                        } else {
                            println!("    📅 Échéance: {}", date_str);
                        }
                    }
                }
                
                if item.status.is_done() {
                    if let Some(completed_at) = item.completed_at {
                        println!("    ✅ Terminé le: {}", completed_at.format("%d/%m/%Y à %H:%M"));
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

fn get_priority_from_user() -> Priority {
    println!("Choisissez la priorité:");
    println!("1 - Basse 🟢");
    println!("2 - Moyenne 🟡");
    println!("3 - Haute 🟠");
    println!("4 - Critique 🔴");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let choice: u32 = input.trim().parse().unwrap_or(0);
        
        match choice {
            1 => return Priority::Low,
            2 => return Priority::Medium,
            3 => return Priority::High,
            4 => return Priority::Critical,
            _ => println!("Choix invalide. Veuillez entrer 1, 2, 3 ou 4."),
        }
    }
}

fn get_due_date_from_user() -> Option<NaiveDate> {
    println!("Voulez-vous ajouter une date d'échéance? (oui/non)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
    if input.trim().to_lowercase() != "oui" {
        return None;
    }
    
    println!("Entrez la date d'échéance (format: JJ/MM/AAAA):");
    let mut date_input = String::new();
    io::stdin().read_line(&mut date_input).expect("Erreur de lecture");
    
    let date_str = date_input.trim();
    match NaiveDate::parse_from_str(date_str, "%d/%m/%Y") {
        Ok(date) => {
            let today = Local::now().date_naive();
            if date < today {
                println!("⚠️  Attention: Cette date est dans le passé!");
                println!("Voulez-vous continuer? (oui/non)");
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Erreur de lecture");
                if confirm.trim().to_lowercase() != "oui" {
                    return None;
                }
            }
            Some(date)
        }
        Err(_) => {
            println!("Format de date invalide. Utilisez JJ/MM/AAAA (ex: 25/12/2024)");
            None
        }
    }
}

fn get_status_from_user() -> TaskStatus {
    println!("Choisissez l'état de la tâche :");
    println!("1 - À faire ⬜");
    println!("2 - En cours 🟦");
    println!("3 - En attente 🟨");
    println!("4 - Terminée ✅");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let choice: u32 = input.trim().parse().unwrap_or(0);
        match choice {
            1 => return TaskStatus::Afaire,
            2 => return TaskStatus::EnCours,
            3 => return TaskStatus::EnAttente,
            4 => return TaskStatus::Terminee,
            _ => println!("Choix invalide. Veuillez entrer 1, 2, 3 ou 4."),
        }
    }
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
        let status = get_status_from_user();
        let priority = get_priority_from_user();
        let due_date = get_due_date_from_user();
        
        todo_list.add_item_with_details_status(title, desc, status, priority, due_date);
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
        println!("2 - Changer l'état d'un élément");
        println!("3 - Marquer un élément comme terminé/non terminé");
        println!("4 - Supprimer un élément");
        println!("5 - Modifier la priorité d'un élément");
        println!("6 - Modifier la date d'échéance d'un élément");
        println!("7 - Afficher la liste");
        println!("8 - Retour au menu principal");
        
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
                let status = get_status_from_user();
                let priority = get_priority_from_user();
                let due_date = get_due_date_from_user();
                
                todo_list.add_item_with_details_status(title, desc, status, priority, due_date);
                println!("Élément ajouté!");
            }
            2 => {
                if todo_list.items.is_empty() {
                    println!("La liste est vide!");
                    continue;
                }
                todo_list.display();
                println!("Entrez l'ID de l'élément dont vous voulez changer l'état:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Erreur de lecture");
                let id: u32 = id_input.trim().parse().expect("Veuillez entrer un nombre");
                let new_status = get_status_from_user();
                if let Some(item) = todo_list.items.iter_mut().find(|item| item.id == id) {
                    item.status = new_status;
                    if new_status.is_done() {
                        item.completed_at = Some(Utc::now());
                    } else {
                        item.completed_at = None;
                    }
                    todo_list.last_modified = Utc::now();
                    println!("État modifié!");
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
            4 => {
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
            5 => {
                if todo_list.items.is_empty() {
                    println!("La liste est vide!");
                    continue;
                }
                
                todo_list.display();
                println!("Entrez l'ID de l'élément dont vous voulez modifier la priorité:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Erreur de lecture");
                let id: u32 = id_input.trim().parse().expect("Veuillez entrer un nombre");
                
                let new_priority = get_priority_from_user();
                if todo_list.update_item_priority(id, new_priority) {
                    println!("Priorité modifiée!");
                } else {
                    println!("Élément non trouvé!");
                }
            }
            6 => {
                if todo_list.items.is_empty() {
                    println!("La liste est vide!");
                    continue;
                }
                
                todo_list.display();
                println!("Entrez l'ID de l'élément dont vous voulez modifier la date d'échéance:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Erreur de lecture");
                let id: u32 = id_input.trim().parse().expect("Veuillez entrer un nombre");
                
                let new_due_date = get_due_date_from_user();
                if todo_list.update_item_due_date(id, new_due_date) {
                    println!("Date d'échéance modifiée!");
                } else {
                    println!("Élément non trouvé!");
                }
            }
            7 => {
                todo_list.display();
            }
            8 => break,
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
