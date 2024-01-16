use std::fmt::Display;

use promptable::Date;
use promptable_derive::Promptable;
#[derive(Promptable, Clone)]
pub struct Form {
    name: String,
    lastname: String,
    nickname: Option<String>,
    birthdate: Date,
    email: String,
}

impl Display for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
fn main() {
    let mut form = Form::new_by_prompt().unwrap();
    form.modify_by_prompt().unwrap();
    let mut forms = Vec::new();
    forms.push(form);
    // or to make an empty vec
    // let mut forms = <Vec<Form> as PromptableVecForm>::new();
    forms.multiple_by_prompt().unwrap();
}
