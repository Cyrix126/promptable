use crate::basics::display::clear_screen;
use crate::basics::display::PromptableDisplay;
use crate::basics::menu::{menu_cancel, menu_confirm, MenuClassic};
use anyhow::Result;
use derive_more::{Deref, Display};
use inquire::{Confirm, CustomType, DateSelect, Select, Text};
use time::{Date as DateOrigin, OffsetDateTime};
use trait_gen::trait_gen;
/// This trait is implemented for some basic types.
///# Example
///```rust,no_run
///
///use promptable::basics::promptable::Date;
///use promptable::basics::promptable::Promptable;
///if let Some(mut anwser) = Date::new_by_prompt("choose the date")? {
/// anwser.modify_by_prompt("change the date")?;
///}
/// # Ok::<(), anyhow::Error>(())
///```

pub trait Promptable<P> {
    /// method to create a value.
    /// if Self is Option\<T\>, Ok(None) will be returned only if anwser is canceled.
    /// generic P is used to be able to pass multiple different parameters. Tuple could be used to pass multiple differents one.
    fn new_by_prompt(params: P) -> Result<Option<Self>>
    where
        Self: Sized;

    /// modify the self value.
    /// Will return a result, Ok(()) if everything went fine or canceled, Err(()) for otherwise.
    fn modify_by_prompt(&mut self, params: P) -> Result<bool>;
}

/// generic default implementation for Vec\<T\>
/// macro derive can still implement his one with much more customization.
#[trait_gen(T -> u8, u16, u32, u64, u128, i8, i32, i64, f32, f64, String)]
impl Promptable<&str> for Vec<T>
where
    T: Clone + PromptableDisplay,
    Vec<T>: PromptableDisplay,
{
    fn new_by_prompt(msg: &str) -> Result<Option<Self>> {
        if let Some(r) = T::new_by_prompt(msg)? {
            Ok(Some(vec![r]))
        } else {
            Ok(None)
        }
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<bool> {
        let options_menu = MenuClassic::consts();
        // idea: rather than cloning the self and chaning a new self or an old self, why not create a vec and only add what changes and then apply on self if confirmed ?
        let restore_self = self.clone();
        let mut modified = false;
        loop {
            if self.is_empty() {
                if let Some(s) = T::new_by_prompt(msg)? {
                    self.push(s);
                }
            }
            clear_screen();
            if let Some(choix) = Select::new(msg, options_menu.to_vec()).prompt_skippable()? {
                match choix {
                    MenuClassic::ADD => {
                        if add_by_prompt_vec(self, msg)? {
                            modified = true
                        }
                    }
                    MenuClassic::MODIFY => {
                        if modify_by_prompt_vec(self, msg)? {
                            modified = true
                        }
                    }
                    MenuClassic::DELETE => {
                        if delete_by_prompt_vec(self, msg)? {
                            modified = true
                        }
                    }
                    MenuClassic::CANCEL => {
                        if menu_cancel(&restore_self, self)? {
                            modified = false;
                            break;
                        }
                    }
                    _ => {
                        if menu_confirm(&restore_self, self)? {
                            break;
                        }
                    }
                }
            }
        }
        Ok(modified)
    }
}

fn add_by_prompt_vec<T: for<'a> Promptable<&'a str>>(
    vec: &mut Vec<T>,
    msg: &str,
) -> anyhow::Result<bool> {
    if let Some(c) = T::new_by_prompt(msg)? {
        vec.push(c);
        return Ok(true);
    }
    Ok(false)
}

fn delete_by_prompt_vec<T: for<'a> Promptable<&'a str> + Clone + PromptableDisplay>(
    vec: &mut Vec<T>,
    _msg: &str,
) -> anyhow::Result<bool> {
    clear_screen();
    if vec.is_empty() {
        return Ok(false);
    }
    let choix = match inquire::MultiSelect::new(
        "Select objects to delete",
        vec.iter().map(|e: &T| e.display_short()).collect(),
    )
    .raw_prompt_skippable()?
    {
        Some(l) => l,
        None => return Ok(false),
    };
    let mut indexes = Vec::new();
    for c in choix {
        indexes.push(c.index);
    }
    indexes.sort_unstable_by(|a, b| b.cmp(a));
    for index in indexes {
        vec.remove(index);
    }
    Ok(true)
}

fn modify_by_prompt_vec<T: for<'a> Promptable<&'a str> + Clone + PromptableDisplay>(
    vec: &mut [T],
    msg: &str,
) -> anyhow::Result<bool> {
    clear_screen();
    let choix = Select::new(
        "Sélection de l'objet à modifier",
        vec.iter().map(|e: &T| e.display_short()).collect(),
    )
    .raw_prompt()?;
    Ok(vec[choix.index].modify_by_prompt(msg)?)
}

/// macro to implement quicly the Promptable trait on basic types.
/// use of CustomType to re-ask user if input is incorrect.
/// can only be used with prompt backend that support custom_type.
#[trait_gen(T -> u8, u16, u32, u64, u128, i8, i32, i64, f32, f64)]
impl Promptable<&str> for T {
    /// method to create a value.
    /// if Self is Option\<T\>, Ok(None) will be returned only if anwser is canceled.
    fn new_by_prompt(msg: &str) -> Result<Option<T>> {
        clear_screen();
        Ok(CustomType::<T>::new(msg).prompt_skippable()?)
    }

    /// modify the self value.
    /// Will return a anyhow Result, Ok(()) if everything went fine or canceled, Err(()) for otherwise.
    fn modify_by_prompt(&mut self, msg: &str) -> Result<bool> {
        clear_screen();
        if let Some(prompt) = CustomType::<T>::new(msg)
            .with_default(*self)
            .with_placeholder(&self.to_string())
            .prompt_skippable()?
        {
            *self = prompt;
            return Ok(true);
        }
        Ok(false)
    }
}

/// String can be used with the Text type of inquire
impl Promptable<&str> for String {
    fn new_by_prompt(msg: &str) -> Result<Option<Self>> {
        Ok(Text::new(msg).prompt_skippable()?)
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<bool> {
        let prompt = Text::new(msg).with_initial_value(self).prompt_skippable()?;
        if let Some(v) = prompt {
            *self = v;
            return Ok(true);
        }
        Ok(false)
    }
}

/// bool can be used with the Confirm type of inquire
impl Promptable<&str> for bool {
    fn new_by_prompt(msg: &str) -> Result<Option<Self>> {
        Ok(Confirm::new(msg).prompt_skippable()?)
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<bool> {
        let prompt = Confirm::new(msg).with_default(*self).prompt_skippable()?;
        if let Some(v) = prompt {
            *self = v;
            return Ok(true);
        }
        Ok(false)
    }
}

/// Wrapper of time::Date
#[derive(Clone, Display, Deref, PartialEq, PartialOrd)]
pub struct Date(pub DateOrigin);

impl Default for Date {
    fn default() -> Self {
        let date = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
        Date(date.date())
    }
}

impl Promptable<&str> for Date {
    fn new_by_prompt(msg: &str) -> Result<Option<Self>> {
        clear_screen();
        let date = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()?;
        if let Some(date) = date {
            Ok(Some(Date(date)))
        } else {
            Ok(None)
        }
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<bool> {
        clear_screen();
        let prompt = DateSelect::new(msg)
            .with_starting_date(self.0)
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()?;
        if let Some(v) = prompt {
            *self = Date(v);
            return Ok(true);
        }
        Ok(false)
    }
}
