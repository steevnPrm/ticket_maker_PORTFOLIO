use std::fs::{self, File};
use std::io::Write;

use crate::ticket::model::TicketModel; 

pub fn save_ticket(ticket: &TicketModel) -> std::io::Result<()> {

    let dir_path = "tickets";
    fs::create_dir_all(dir_path)?;

    let filename = format!("{}.md", ticket.title.replace(" ", "_"));
    
    let mut file = File::create(&filename)?;
    

    file.write_all(ticket.to_markdown().as_bytes())?;
    
    println!("Ticket sauvegardé : {}", filename);
    Ok(())
}