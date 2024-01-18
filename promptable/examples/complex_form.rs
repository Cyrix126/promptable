use std::fmt::Display;

use anyhow::Result;
use inquire::{Editor, Select};
use promptable::{Date, Promptable};
use promptable_derive::Promptable;
#[derive(Promptable, Clone)]
#[prompt(msg_mod = "Select the field to modify the Prestation")]
#[prompt(params = "msg_search: &str, msg_editor: &str, clients: &[String]")]
pub struct Prestation {
    #[promptable(visible = false)]
    #[promptable(function_add = "increment(&self.last().unwrap())")]
    id: u32,
    #[promptable(multiple_once = true)]
    #[promptable(function = "search_client(msg_search, clients)")]
    client: String,
    date: Date,
    hours: f32,
    #[promptable(function_render = "add_euros(field_value)")]
    price: f32,
    payed: bool,
    #[promptable(function = "editor(msg_editor)")]
    description: String,
}

impl Display for Prestation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.id, self.client, self.date)
    }
}

fn increment(p: &Prestation) -> u32 {
    p.id + 1
}
fn search_client(msg_search: &str, clients: &[String]) -> String {
    Select::new(msg_search, clients.to_owned())
        .prompt()
        .unwrap()
}

fn editor(msg_editor: &str) -> String {
    Editor::new(msg_editor).prompt().unwrap()
}

fn add_euros(price: &f32) -> String {
    format!("{price}€")
}

fn main() -> Result<()> {
    let clients = vec!["ClientA".to_string(), "ClientB".to_string()];
    Prestation::new_by_prompt(("New prestation", "Description", &clients))?;
    let mut prestations = VecPrestation(Vec::new());
    prestations.modify_by_prompt(("New prestation", "Description", &clients))?;
    Ok(())
}
