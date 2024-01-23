#![warn(missing_docs)]

#[cfg(feature = "basics")]
use {
    crate::basics::{
        display::{clear_screen, PromptableDisplay},
        promptable::Date,
    },
    inquire::Select,
    trait_gen::trait_gen,
};

use anyhow::Result;
/// trait to inspect a type with a PromptableDisplay with read only access.
pub trait Inspectable {
    /// method to show the PromptableDisplay element, until viewer press Enter or Escape.
    /// An implementation is made for Vec<T> where the viewer can choose which element to see.
    /// The vec implementation is automaticly implemented for VecStruct with derive macro. An implementation also exist for Vec of basic types.
    #[cfg(feature = "basics")]
    fn inspect_menu(&self) -> Result<()>
    where
        Self: PromptableDisplay,
    {
        println!("{}", self.display_human());
        Select::new("", vec![""])
            .with_help_message("Press Enter or Escape to quit the view.")
            .prompt_skippable()?;
        Ok(())
    }
    /// if basics feature is disabled, there is no default method for this trait.
    #[cfg(not(feature = "basics"))]
    fn inspect_menu(&self) -> Result<()>;
}

#[cfg(feature = "basics")]
#[trait_gen(T -> u8, u16, u32, u64, u128, i8, i32, i64, f32, f64, String, Date, bool)]
impl Inspectable for T {}
#[cfg(feature = "basics")]
#[trait_gen(T -> u8, u16, u32, u64, u128, i8, i32, i64, f32, f64, String, Date, bool)]
impl Inspectable for Vec<T>
where
    T: Clone + PromptableDisplay,
    Vec<T>: PromptableDisplay,
{
    fn inspect_menu(&self) -> Result<()> {
        use inquire::InquireError;

        let options = self
            .iter()
            .map(|e| e.display_short())
            .collect::<Vec<String>>();
        loop {
            clear_screen();
            match Select::new(
                "Choose the element to see.\nEscape to view",
                options.clone(),
            )
            .raw_prompt()
            {
                Ok(l) => self[l.index].inspect_menu()?,
                Err(InquireError::OperationCanceled) => break,
                Err(e) => return Err(e.into()),
            }
        }
        clear_screen();
        Ok(())
    }
}
