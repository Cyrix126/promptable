use anyhow::Result;
use inquire::{Editor, Select};
use promptable::{Date, Promptable};
use promptable_derive::Promptable;
#[derive(Promptable, Clone)]
#[prompt(msg_mod = "Select the field to modify the Prestation")]
#[prompt(params = "msg_search: &str, msg_editor: &str, clients: &[String]")]
pub struct Prestation {
    #[promptable(function_add = "increment(&self.last().unwrap())")]
    id: u32,
    #[promptable(multiple_once = true)]
    #[promptable(function_new = "search_client(msg_search, clients)")]
    #[promptable(function_mod = "search_client_mod(field, msg_search, clients)")]
    client: String,
    #[promptable(default = true)]
    // will put the default value of Date (which is today), can be modified.
    date: Date,
    hours: f32,
    #[promptable(function_render = "add_euros(field_value)")]
    price: f32,
    // #[promptable(function_mod = "can_not_be_unpayed(initial_value)")]
    payed: bool,
    #[promptable(function_new = "editor(msg_editor)")]
    #[promptable(function_mod = "editor_mod(field, msg_editor)")]
    description: String,
    #[promptable(visible = false)]
    // will be skipped, can't edit by prompt, will use default value, None if Option.
    field_not_promptable: i128,
}

fn increment(p: &Prestation) -> Result<Option<u32>> {
    Ok(Some(p.id + 1))
}
fn search_client(msg_search: &str, clients: &[String]) -> Result<Option<String>> {
    Ok(Select::new(msg_search, clients.to_owned()).prompt_skippable()?)
}
fn search_client_mod(field: &mut String, msg_search: &str, clients: &[String]) -> Result<()> {
    if let Some(s) = Select::new(msg_search, clients.to_owned()).prompt_skippable()? {
        *field = s;
    }
    Ok(())
}

fn editor(msg_editor: &str) -> Result<Option<String>> {
    Ok(Editor::new(msg_editor).prompt_skippable()?)
}
fn editor_mod(field: &mut String, msg_editor: &str) -> Result<()> {
    if let Some(s) = Editor::new(msg_editor).prompt_skippable()? {
        *field = s;
    }
    Ok(())
}

fn add_euros(price: &f32) -> String {
    format!("{price}€")
}

fn main() -> Result<()> {
    let clients = vec!["ClientA".to_string(), "ClientB".to_string()];
    Prestation::new_by_prompt(("New prestation", "Description", &clients))?;
    // let mut prestations = VecPrestation(Vec::new());
    // prestations.modify_by_prompt(("New prestation", "Description", &clients))?;
    Ok(())
}
