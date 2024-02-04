use anyhow::Result;
use inquire::{Editor, Select, Text};
use promptable::basics::display;
use promptable::basics::{
    display::PromptableDisplay,
    promptable::{Date, Promptable},
};
use promptable::inspect::Inspectable;
use promptable_derive::Promptable;
#[derive(Promptable, Clone)]
#[prompt(params = "msg_search: &str, msg_editor: &str, clients: &[String]")]
pub struct Prestation {
    #[promptable(short_display)]
    #[promptable(function_add = "increment(&self.last().unwrap())")]
    #[promptable(function_new = "Some(1)")]
    id: u32,
    #[promptable(multiple_once = true)]
    #[promptable(function_new = "search_client(msg_search, clients)?")]
    #[promptable(function_mod = "search_client_mod(field, msg_search, clients)?")]
    client: String,
    #[promptable(default = true)]
    // will put the default value of Date (which is today), can be modified.
    #[promptable(short_display)]
    date: Date,
    hours: f32,
    #[promptable(function_render = "add_euros(field_value)")]
    #[promptable(short_display)]
    price: f32,
    // #[promptable(function_mod = "can_not_be_unpayed(initial_value)")]
    #[promptable(short_display)]
    payed: bool,
    #[promptable(function_new = "editor(msg_editor)?")]
    #[promptable(function_mod = "editor_mod(field, msg_editor)?")]
    description: String,
    #[promptable(visible = false)]
    // will be skipped, can't edit by prompt, will use default value, None if Option.
    field_not_promptable: i128,
    #[promptable(function_new = "new_adr()?")]
    #[promptable(function_mod = "mod_adr(field)?")]
    #[promptable(inspect = false)]
    adr: Option<Adr>,
}

// Struct that doesn't implement Promptable
#[derive(Clone)]
pub struct Adr(String);
impl Inspectable for Adr {}

fn new_adr() -> Result<Option<Adr>> {
    if let Some(adr) = Text::new("Adresse").prompt_skippable()? {
        Ok(Some(Adr(adr)))
    } else {
        Ok(None)
    }
}
fn mod_adr(adresse: &mut Option<Adr>) -> Result<()> {
    if let Some(adr) = Text::new("Adresse").prompt_skippable()? {
        *adresse = Some(Adr(adr))
    } else {
        *adresse = None
    }
    Ok(())
}
impl PromptableDisplay for Adr {
    fn display_short(&self) -> String {
        self.0.clone()
    }
    fn display_human(&self) -> String {
        self.display_short()
    }
}

fn increment(p: &Prestation) -> Option<u32> {
    Some(p.id + 1)
}
fn search_client(msg_search: &str, clients: &[String]) -> Result<Option<String>> {
    display::clear_screen();
    Ok(Select::new(msg_search, clients.to_owned()).prompt_skippable()?)
}
fn search_client_mod(field: &mut String, msg_search: &str, clients: &[String]) -> Result<()> {
    if let Some(s) = Select::new(msg_search, clients.to_owned()).prompt_skippable()? {
        *field = s;
    }
    Ok(())
}

fn editor(msg_editor: &str) -> Result<Option<String>> {
    display::clear_screen();
    Ok(Editor::new(msg_editor).prompt_skippable()?)
}
fn editor_mod(field: &mut String, msg_editor: &str) -> Result<()> {
    display::clear_screen();
    if let Some(s) = Editor::new(msg_editor).prompt_skippable()? {
        *field = s;
    }
    Ok(())
}

fn add_euros(price: &f32) -> String {
    format!("{price}€")
}

fn main() -> Result<()> {
    let clients = ["ClientA".to_string(), "ClientB".to_string()];
    let client = Prestation::new_by_prompt(&("New prestation", "Description", &clients))?;
    let mut prestations = VecPrestation(Vec::new());
    if let Some(c) = client {
        prestations.push(c);
    }
    prestations.modify_by_prompt(&("New prestation", "Description", &clients[..]))?;

    prestations.inspect_menu()?;
    Ok(())
}
