#![feature(min_specialization)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use derive_more::{Deref, Display};
use inquire::{Confirm, CustomType, DateSelect, Text};
use std::{fmt::Display, str::FromStr};
pub extern crate inquire;
pub extern crate promptable_derive;
use anyhow::Result;
use termion::{clear::All, cursor::Goto};
use time::{Date as DateOrigin, OffsetDateTime};
#[doc(hidden)]
/// function using termion to clear the screen.
/// Used often in the method of declarative macro.
pub fn clear_screen() {
    println!("{}{}", All, Goto(1, 1));
}

/// This trait is implemented for some basic types.
///# Example
///```rust,no_run
///
///use promptable::Date;
///use promptable::Promptable;
///if let Some(mut anwser) = Date::new_by_prompt("choose the date").unwrap() {
/// anwser.modify_by_prompt("change the date");   
///}
///```

pub trait Promptable {
    /// method to create a value.
    /// if the user cancel the prompt, Err(()) will be returned, otherwise Ok(Self) will be.
    /// if Self is Option<T>, Ok(None) will be returned only if anwser is empty.
    /// If the prompt backend return an error other than a way to tell the user has canceled, the method will panic.
    fn new_by_prompt(msg: &str) -> Result<Option<Self>>
    where
        Self: Sized + Display + PartialEq;

    /// modify the self value.
    /// Will return a result, Ok(()) if everything went fine or canceled, Err(()) for otherwise.
    fn modify_by_prompt(&mut self, msg: &str) -> Result<()>
    where
        Self: Sized + Display + PartialEq;
}
impl<T: Sized + Clone + Display + FromStr + PartialEq> Promptable for T {
    /// method to create a value.
    /// if the user cancel the prompt, Err(()) will be returned, otherwise Ok(Self) will be.
    /// if Self is Option<T>, Ok(None) will be returned only if anwser is empty.
    /// If the prompt backend return an error other than a way to tell the user has canceled, the method will panic.
    default fn new_by_prompt(msg: &str) -> Result<Option<T>> {
        clear_screen();
        Ok(CustomType::<T>::new(msg).prompt_skippable()?)
    }

    /// modify the self value.
    /// Will return a result, Ok(()) if everything went fine or canceled, Err(()) for otherwise.
    default fn modify_by_prompt(&mut self, msg: &str) -> Result<()> {
        clear_screen();
        if let Some(prompt) = CustomType::<T>::new(msg)
            .with_default(self.clone())
            .with_placeholder(&self.to_string())
            .prompt_skippable()?
        {
            *self = prompt;
        }
        Ok(())
    }
}
/// macro to implement quicly the Promptable trait on basic types.
/// use of CustomType to re-ask user if input is incorrect.
/// can only be used with prompt backend that support custom_type.

/// String can be used with the Text type of inquire
impl Promptable for String {
    fn new_by_prompt(msg: &str) -> Result<Option<Self>> {
        clear_screen();
        Ok(Text::new(msg).prompt_skippable()?)
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<()> {
        clear_screen();
        let prompt = Text::new(msg).with_initial_value(self).prompt_skippable()?;
        if let Some(v) = prompt {
            *self = v;
        }
        Ok(())
    }
}
/// String can be used with the Text type of inquire
/// bool can be used with the Confirm type of inquire
impl Promptable for bool {
    fn new_by_prompt(msg: &str) -> Result<Option<Self>> {
        clear_screen();
        Ok(Confirm::new(msg).prompt_skippable()?)
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<()> {
        clear_screen();
        let prompt = Confirm::new(msg).with_default(*self).prompt_skippable()?;
        if let Some(v) = prompt {
            *self = v;
        }
        Ok(())
    }
}

/// Wrapper of time::Date
#[derive(Clone, Display, Deref, PartialEq)]
pub struct Date(pub DateOrigin);

impl Default for Date {
    fn default() -> Self {
        let date = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
        Date(date.date())
    }
}

// impl From<Option<DateOrigin>> for Option<Date> {
//     fn from(option_value: Option<DateOrigin>) -> Self {
//         match option_value {
//             Some(value) => Some(Date::from(value)),
//             None => None,
//         }
//     }
// }
impl Promptable for Date {
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
    fn modify_by_prompt(&mut self, msg: &str) -> Result<()> {
        clear_screen();
        let prompt = DateSelect::new(msg)
            .with_starting_date(self.0)
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()?;
        if let Some(v) = prompt {
            *self = Date(v);
        }
        Ok(())
    }
}
