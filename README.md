# 🎫 Ticket Maker CLI

Un outil en ligne de commande (CLI) écrit en **Rust** pour générer des tickets de développement uniformes. Il permet de sauvegarder vos idées localement en Markdown ou de les envoyer directement comme **GitHub Issues**.

## ✨ Fonctionnalités

* **Saisie Interactive** : Utilise `inquire` pour une expérience utilisateur fluide (titre, catégorie, description).
* **Export Markdown** : Génère automatiquement un fichier `.md` structuré dans un dossier `/tickets`.
* **Intégration GitHub** : Envoie vos tickets directement sur le dépôt de votre choix via l'API REST de GitHub.
* **Multi-plateforme** : Fonctionne sur Linux, macOS et Windows.

## 🚀 Installation

### Prérequis
* [Rust](https://www.rust-lang.org/tools/install) (cargo, rustc)
* Un **GitHub Personal Access Token** (Classic) avec les permissions `repo`.

### Clonage et Compilation
```bash
git clone https://github.com/steevnPrm/ticket_maker_PORTFOLIO.git
cd ticket_maker
cargo build --release
```

## 🛠 Utilisation

### 1. Configurer votre Token
Pour des raisons de sécurité, l'outil lit votre jeton d'accès depuis les variables d'environnement de votre système.

```bash
export GITHUB_TOKEN=ghp_votre_token_ici
```

### 2. Lancer l'outil
```bash
cargo run
```

### 3. Workflow
1.  Entrez le **titre** du ticket.
2.  Renseignez le **dépôt cible** (format `pseudo/projet`).
3.  Choisissez la **catégorie** (Bug, Feature, etc.).
4.  Rédigez la **description**.
5.  Le ticket est créé instantanément sur GitHub !

## 📂 Structure du projet

* `src/main.rs` : Point d'entrée et logique de la boucle interactive.
* `src/ticket/model.rs` : Définition des structures de données (`TicketModel`, `TicketType`).
* `src/ticket/service.rs` : Logique de sauvegarde locale (Système de fichiers).
* `src/ticket/github_service.rs` : Client HTTP pour l'API GitHub (via `reqwest`).

## 📦 Dépendances principales

* [inquire](https://crates.io/crates/inquire) : Pour les prompts interactifs.
* [reqwest](https://crates.io/crates/reqwest) : Client HTTP asynchrone.
* [tokio](https://crates.io/crates/tokio) : Runtime pour l'asynchronisme.
* [serde](https://crates.io/crates/serde) : Sérialisation JSON pour l'API.