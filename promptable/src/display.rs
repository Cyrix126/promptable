use crate::Date;
use similar::{ChangeTag, TextDiff};
use trait_gen::trait_gen;
/// trait to show a very compact rendering.
/// a derive macro exist to implement it automaticlly with the 3 first fields.
/// a derive helper allow to choose the fields, and format them with functions.
pub trait PromptableDisplay {
    /// The arbitrary rule could be that a compact display should not take more than 3 fields.
    fn display_short(&self) -> String;
    /// trait to to show a human oriented display.
    /// by default will take the name of the fields and their value,
    /// TODO present them on column wrapped lines.
    /// derive helper can hide fields, format them with function.
    /// Some fields can be omitted.
    fn display_human(&self) -> String;
    /// trait to show difference beetwen two objects.
    /// will present the difference between the two structs like a diff.
    /// will show the difference beetwen self and another object of same type.
    fn display_diff(&self, b: &Self)
    where
        Self: PromptableDisplay,
    {
        let lines_a = self.display_human().to_string();
        let lines_b = b.display_human().to_string();
        let diff = TextDiff::from_lines(&lines_a, &lines_b);

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            print!("{}{}", sign, change);
        }
    }
}
/// generic impl for Vec\<T\> where T implement PromptableDisplay
impl<T: PromptableDisplay> PromptableDisplay for Vec<T> {
    fn display_short(&self) -> String {
        format!("{} elements.", self.iter().len())
    }
    fn display_human(&self) -> String {
        self.iter()
            .map(|c| c.display_human())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[trait_gen(T -> u8, u16, u32, u64, u128, i8, i32, i64, i128, f32, f64, String, bool, Date)]
impl PromptableDisplay for T {
    fn display_short(&self) -> String {
        self.to_string()
            .lines()
            .next()
            .unwrap_or_default()
            .to_string()
    }
    fn display_human(&self) -> String {
        self.to_string()
    }
}
