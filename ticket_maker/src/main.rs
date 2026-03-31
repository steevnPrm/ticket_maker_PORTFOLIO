mod ticket;

use inquire::{Text, Select};
use crate::ticket::model::{TicketModel, TicketType};
use crate::ticket::service::save_ticket;

fn main() {
    println!("--- 🎫 TICKET MAKER CLI ---");

    loop {
        let title = Text::new("Titre du ticket (laissez vide pour quitter) :")
            .with_placeholder("Ex: Fix crash au démarrage")
            .prompt();


        let title = match title {
            Ok(t) if t.trim().is_empty() => {
                println!("Fermeture du CLI. Au revoir !");
                break; 
            }
            Ok(t) => t,
            Err(_) => break, 
        };


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

 
        let content = Text::new("Description du ticket :")
            .prompt()
            .expect("Erreur lors de la saisie du contenu");


        let new_ticket = TicketModel::new(&title, &content, category);

        match save_ticket(&new_ticket) {
            Ok(_) => println!("\n✅ Succès : Le ticket a été généré !"),
            Err(e) => eprintln!("\n❌ Erreur : {}", e),
        }

        println!("\n--- Nouveau ticket ---");
    }
}