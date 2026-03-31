mod ticket;

use inquire::{Text, Select};
use crate::ticket::model::{TicketModel, TicketType};
use crate::ticket::github_service::create_issue;

#[tokio::main]
async fn main() {
    println!("--- 🎫 TICKET MAKER CLI ---");

    loop {
        let title = Text::new("Titre du ticket (vide pour quitter) :")
            .prompt();

        let title = match title {
            Ok(t) if t.trim().is_empty() => break,
            Ok(t) => t,
            Err(_) => break,
        };

        let repo = Text::new("Dépôt GitHub cible :")
            .with_placeholder("Ex: utilisateur/mon-projet")
            .with_help_message("Format attendu : propriétaire/nom-du-repo")
            .prompt()
            .expect("Erreur lors de la saisie du dépôt");

        let options = vec!["Feature", "Bug", "Documentation", "Refactor"];
        let category_selection = Select::new("Catégorie :", options)
            .prompt()
            .expect("Erreur lors de la sélection");

        let category = match category_selection {
            "Feature" => TicketType::Feature,
            "Bug" => TicketType::Bug,
            "Documentation" => TicketType::Documentation,
            _ => TicketType::Refactor,
        };

        let content = Text::new("Description du ticket :").prompt().expect("Erreur");

        let new_ticket = TicketModel::new(&title, &content, category);
        
        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN non définie");

        match create_issue(&new_ticket, &token, &repo).await {
            Ok(_) => println!("\n✅ Succès : L'issue a été créée sur GitHub !"),
            Err(e) => eprintln!("\n❌ Erreur GitHub : {}", e),
        }

        println!("\n--- Nouveau ticket ---");
    }
}