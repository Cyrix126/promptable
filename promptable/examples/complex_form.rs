use std::fmt::Display;

use inquire::{Editor, Select};
use promptable_derive::Promptable;
use time::Date;
#[derive(Promptable, Clone)]
#[prompt(msg_mod = "Select the field to modify the Prestation")]
#[prompt(params = "msg_search: &str, msg_editor: &str, clients: &Vec<String>")]
struct Prestation {
    #[promptable(visible = false)]
    #[promptable(function_add = "increment(self.last().unwrap().id)")]
    id: u32,
    #[promptable(multiple_once = true)]
    #[promptable(function_new = "search_client(msg_search, clients)")]
    client: String,
    date: Date,
    hours: f32,
    price: f32,
    payed: bool,
    #[promptable(function_new = "editor(msg_editor)")]
    description: String,
}

impl Display for Prestation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

fn increment(a: u32) -> u32 {
    a + 1
}
fn search_client(msg_search: &str, clients: &Vec<String>) -> String {
    Select::new(msg_search, clients.clone()).prompt().unwrap()
}

fn editor(msg_editor: &str) -> String {
    Editor::new(msg_editor).prompt().unwrap()
}

fn main() {
    let clients = vec!["ClientA".to_string(), "ClientB".to_string()];
    let mut prestations = <Vec<Prestation> as PromptableVecPrestation>::new();
    prestations.multiple_by_prompt("New prestation", "Description: ", &clients);
}
