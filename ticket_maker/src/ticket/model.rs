#[derive(Debug, Clone)]
pub enum TicketType {
    Feature,
    Bug,
    Documentation,
    Refactor,
}

#[derive(Debug, Clone)]
pub struct TicketModel {
    pub title: String,
    pub content: String,
    pub category: TicketType,
}

impl TicketModel {
    pub fn new(title: &str, content : &str, category: TicketType) -> Self {
        Self {
            title: title.trim().to_lowercase(),
            content : content.to_string(),
            category,
        }
    }
    pub fn to_markdown(&self) -> String {
        format!(
            "# [{:?}] {}\n\n**Catégorie :** {:?}\n\n---\n\n## Description\n{}",
            self.category,
            self.title,
            self.category,
            self.content
        )
    }
}