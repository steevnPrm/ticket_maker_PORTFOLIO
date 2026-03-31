use std::fs::{self, File};
use std::io::Write;

use crate::ticket::model::TicketModel; 

pub fn save_ticket(ticket: &TicketModel) -> std::io::Result<()> {

    let dir_path = "tickets";
    fs::create_dir_all(dir_path)?;

    let file_path = format!("{}/{}.md", dir_path, ticket.title.replace(" ", "_"));
    
    let mut file = File::create(&file_path)?;
    

    file.write_all(ticket.to_markdown().as_bytes())?;
    
    println!("Ticket sauvegardé : {}", file_path);
    Ok(())
}