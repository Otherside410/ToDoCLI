# ToDoCLI - Gestionnaire de Todo Lists en Rust

## Introduction
ToDoCLI est une application de gestion de todo lists développée en Rust qui fonctionne en ligne de commande. Elle permet de créer, modifier et supprimer des listes de tâches avec stockage persistant en format JSON.

## Fonctionnalités

### 🎯 Fonctionnalités principales
- **Création de todo lists** : Créez de nouvelles listes avec des éléments personnalisés
- **Modification de listes existantes** : Ajoutez, supprimez ou modifiez le statut, l'état, la priorité ou la date d'échéance des éléments
- **Suppression de listes** : Supprimez définitivement des listes avec confirmation
- **Stockage persistant** : Toutes les listes sont sauvegardées automatiquement en JSON

### 📝 Gestion des éléments
Chaque élément de todo list contient :
- **ID unique** : Identifiant automatique pour chaque élément
- **Titre** : Nom de la tâche (obligatoire)
- **Description** : Détails optionnels sur la tâche
- **État** : À faire ⬜, En cours 🟦, En attente 🟨, Terminée ✅
- **Statut** : Terminé (✅) ou non (⬜)
- **Priorité** : Basse 🟢, Moyenne 🟡, Haute 🟠, Critique 🔴
- **Date d'échéance** : Optionnelle, format JJ/MM/AAAA, avec alertes si dépassée
- **Horodatage** : Date de création et de complétion

### 💾 Stockage des données
- Format : JSON lisible et structuré
- Nommage : `nom_de_la_liste.json` (espaces remplacés par des underscores)
- Localisation : Fichiers dans le répertoire d'exécution

## Installation et utilisation

### Prérequis
- Rust (version 1.70+ recommandée)
- Cargo (gestionnaire de paquets Rust)

### Installation
```bash
# Cloner le repository
git clone <url-du-repo>
cd ToDoCLI

# Compiler le projet
cargo build --release

# Exécuter l'application
cargo run
```

### Utilisation

#### Menu principal
L'application affiche un menu avec 4 options :
1. **Créer une nouvelle liste** - Créer une todo list avec des éléments
2. **Mettre à jour une liste existante** - Modifier une liste sauvegardée
3. **Supprimer une liste existante** - Supprimer définitivement une liste
4. **Quitter** - Fermer l'application

#### Création d'une liste
1. Choisissez l'option 1
2. Entrez le nom de votre liste
3. Ajoutez des éléments un par un :
   - Titre de l'élément (obligatoire)
   - Description (optionnelle)
   - État (À faire, En cours, En attente, Terminée)
   - Priorité (Basse, Moyenne, Haute, Critique)
   - Date d'échéance (optionnelle, format JJ/MM/AAAA)
   - Tapez "fin" pour terminer l'ajout d'éléments
4. La liste est automatiquement sauvegardée

#### Modification d'une liste
1. Choisissez l'option 2
2. Sélectionnez la liste à modifier
3. Sous-menu de modification :
   - **Ajouter un élément** : Nouvelle tâche (avec état, priorité et date d'échéance)
   - **Changer l'état d'un élément**
   - **Marquer comme terminé/non terminé**
   - **Supprimer un élément**
   - **Modifier la priorité d'un élément**
   - **Modifier la date d'échéance d'un élément**
   - **Afficher la liste**
   - **Retour au menu principal**

#### Suppression d'une liste
1. Choisissez l'option 3
2. Sélectionnez la liste à supprimer
3. Confirmez la suppression (oui/non)

## Structure des données

### Format JSON
```json
{
  "name": "Nom de la liste",
  "items": [
    {
      "id": 1,
      "title": "Titre de la tâche",
      "description": "Description optionnelle",
      "status": "Afaire",
      "priority": "High",
      "due_date": "2025-12-25",
      "created_at": "2025-06-24T21:29:00Z",
      "completed_at": null
    }
  ],
  "created_at": "2025-06-24T21:29:00Z",
  "last_modified": "2025-06-24T21:29:00Z"
}
```

- **status** : "Afaire", "EnCours", "EnAttente", "Terminee"
- **priority** : "Low", "Medium", "High", "Critical"
- **due_date** : chaîne au format "AAAA-MM-JJ" ou null

## Dépendances

- **serde** : Sérialisation/désérialisation JSON
- **serde_json** : Manipulation de fichiers JSON
- **chrono** : Gestion des dates et heures

## Développement

### Structure du projet
```
ToDoCLI/
├── Cargo.toml          # Configuration et dépendances
├── src/
│   └── main.rs         # Code source principal
└── README.md           # Documentation
```

### Compilation
```bash
# Mode développement
cargo build

# Mode production
cargo build --release

# Vérification du code
cargo check

# Tests (si implémentés)
cargo test
```

## Fonctionnalités futures

- [ ] Interface graphique (TUI)
- [ ] Catégories et tags pour les éléments
- [ ] Dates d'échéance récurrentes
- [ ] Priorités personnalisables
- [ ] Export/import de listes
- [ ] Synchronisation cloud
- [ ] Rappels et notifications

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :
- Signaler des bugs
- Proposer de nouvelles fonctionnalités
- Soumettre des pull requests

## Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.