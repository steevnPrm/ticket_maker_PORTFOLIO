use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use serde::Serialize;
use crate::ticket::model::TicketModel;

#[derive(Serialize)]
struct GithubIssue {
    title: String,
    body: String,
    labels: Vec<String>,
}

pub async fn create_issue(ticket: &TicketModel, token: &str, repo: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/issues", repo);

    let issue = GithubIssue {
        title: ticket.title.clone(),
        body: ticket.to_markdown(),
        labels: vec![format!("{:?}", ticket.category)], 
    };


    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("ticket-maker-app"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());

    let response = client.post(url)
        .headers(headers)
        .json(&issue)
        .send()
        .await?;

    if response.status().is_success() {
        println!("🚀 Ticket créé sur GitHub !");
    } else {
        eprintln!("❌ Erreur GitHub : {}", response.status());
    }

    Ok(())
}