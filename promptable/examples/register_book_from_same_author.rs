use std::fmt::Display;

use anyhow::Result;
use promptable::{Date, Promptable};
use promptable_derive::Promptable;
#[derive(Promptable, Clone)]
#[prompt(msg_mod = "Select the field to modify")]
pub struct Book {
    #[promptable(multiple_once = true)]
    author: String,
    title: String,
    price: f32,
    #[promptable(default = true)]
    #[promptable(name = "Number of pages")]
    nb_pages: Option<u32>,
    #[promptable(name = "Date of release")]
    release_date: Option<Date>,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
fn main() -> Result<()> {
    let mut forms = VecBook(Vec::new());
    forms.modify_by_prompt(())?;
    Ok(())
}
