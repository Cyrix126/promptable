use anyhow::Result;
use promptable::{Date, Promptable};
use promptable_derive::Promptable;
#[derive(Promptable, Clone)]
pub struct Form {
    name: String,
    lastname: String,
    nickname: Option<String>,
    birthdate: Date,
    email: String,
}

fn main() -> Result<()> {
    // // single element and modify it.
    // let form = Form::new_by_prompt(())?;
    // if let Some(mut f) = form {
    //     f.modify_by_prompt(())?;
    // }
    // // create and manage multiples element.
    // let mut forms = VecForm(Vec::new());
    // forms.modify_by_prompt(())?;

    Ok(())
}
