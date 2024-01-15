#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use inquire::{required, Confirm, CustomType, DateSelect, Text};
pub extern crate inquire;
pub extern crate promptable_derive;
use termion::{clear::All, cursor::Goto};
use time::{Date, OffsetDateTime};
const INQUIRE_ERROR: &str = "inquire returned an error that wasn't a cancel.";
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
///use time::Date;
///use promptable::Promptable;
///let mut anwser = Date::new_by_prompt("choose the date").unwrap();
///anwser.modify_by_prompt("change the date");
///```

pub trait Promptable {
    /// method to create a value.
    /// if the user cancel the prompt, Err(()) will be returned, otherwise Ok(Self) will be.
    /// if Self is Option<T>, Ok(None) will be returned only if anwser is empty.
    /// If the prompt backend return an error other than a way to tell the user has canceled, the method will panic.
    fn new_by_prompt(msg: &str) -> Result<Self, ()>
    where
        Self: Sized + ToString;
    /// modify the self value.
    /// Will return a result, Ok(()) if everything went fine, Err(()) if canceled, will panic otherwise.
    fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()>;
}

/// macro to implement quicly the Promptable trait on basic types.
/// use of CustomType to re-ask user if input is incorrect.
/// can only be used with prompt backend that support custom_type.
macro_rules! impl_promptable {
    ($t:ty) => {
        impl Promptable for $t {
            /// ask to create a new value, msg can transmit the message of the prompt.
            /// because of CustomType, it can not verify if the value is empty
            fn new_by_prompt(msg: &str) -> Result<Self, ()> {
                clear_screen();
                let prompt = CustomType::<$t>::new(msg)
                    .prompt_skippable()
                    .expect(INQUIRE_ERROR);
                if let Some(v) = prompt {
                    Ok(v)
                } else {
                    Err(())
                }
            }
            /// ask to modify the value, msg can transmit the message of the prompt.
            /// because of CustomType, it can not verify if the value is empty
            fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()> {
                clear_screen();
                let prompt = CustomType::<$t>::new(msg)
                    .with_default(self.clone())
                    .with_placeholder(&self.to_string())
                    .prompt_skippable()
                    .expect(INQUIRE_ERROR);
                if let Some(v) = prompt {
                    *self = v;
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
        impl Promptable for Option<$t> {
            /// ask to optionnaly create a value, msg can transmit the message of the prompt.
            /// because of CustomType, it can not verify if the value is empty
            fn new_by_prompt(msg: &str) -> Result<Self, ()> {
                clear_screen();
                let prompt = CustomType::<$t>::new(msg)
                    .prompt_skippable()
                    .expect(INQUIRE_ERROR);
                if let Some(v) = prompt {
                    Ok(Some(v))
                } else {
                    Err(())
                }
            }
            /// ask to modify a value, can return None, msg can transmit the message of the prompt.
            /// because of CustomType, it can not verify if the value is empty
            fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()> {
                clear_screen();
                let value = if let Some(s) = self {
                    CustomType::<$t>::new(msg)
                        .with_default(s.clone())
                        .with_placeholder(&s.to_string())
                        .prompt_skippable()
                        .expect(INQUIRE_ERROR)
                } else {
                    CustomType::<$t>::new(msg)
                        .prompt_skippable()
                        .expect(INQUIRE_ERROR)
                };

                if let Some(v) = value {
                    *self = Some(v);
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    };
}
// impl<T: Clone + Display + FromStr> Promptable for Option<T> {
//     fn new_by_prompt(msg: &str) -> Self {
//         let resp = CustomType::<T>::new(msg).prompt_skippable().unwrap();
//         clear_screen();
//         return resp;
//     }
//     fn modify_by_prompt(&mut self, msg: &str) {
// if let Some(s) = self {
//             let resp = CustomType::<T>::new(msg)
//                 .with_default(s.clone())
//                 .with_placeholder(&s.to_string())
//                 .prompt_skippable()
//                 .unwrap();
//             *self = resp;
//         } else {
//             let resp = CustomType::<T>::new(msg).prompt_skippable().unwrap();
//             *self = resp;
//         }
//         clear_screen();
//     }
// }

/// String can be used with the Text type of inquire
impl Promptable for String {
    fn new_by_prompt(msg: &str) -> Result<Self, ()> {
        clear_screen();
        let prompt = Text::new(msg)
            .with_validator(required!("This field is required"))
            .prompt_skippable()
            .expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            Ok(v)
        } else {
            Err(())
        }
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()> {
        clear_screen();
        let prompt = Text::new(msg)
            .with_validator(required!("This field is required"))
            .with_initial_value(self)
            .prompt_skippable()
            .expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            *self = v;
            Ok(())
        } else {
            Err(())
        }
    }
}
/// String can be used with the Text type of inquire
impl Promptable for Option<String> {
    fn new_by_prompt(msg: &str) -> Result<Self, ()> {
        clear_screen();
        let prompt = Text::new(msg).prompt_skippable().expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            // not canceled
            if v.is_empty() {
                // but anwser empty, return Some(None)
                Ok(None)
            } else {
                Ok(Some(v))
            }
        } else {
            //canceled
            Err(())
        }
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()> {
        clear_screen();
        let value = if let Some(s) = self {
            Text::new(msg)
                .with_default(&s)
                .with_placeholder(&s.to_string())
                .prompt_skippable()
                .expect(INQUIRE_ERROR)
        } else {
            Text::new(msg).prompt_skippable().expect(INQUIRE_ERROR)
        };

        if let Some(v) = value {
            *self = Some(v);
            Ok(())
        } else {
            Err(())
        }
    }
}
/// bool can be used with the Confirm type of inquire
impl Promptable for bool {
    fn new_by_prompt(msg: &str) -> Result<Self, ()> {
        clear_screen();
        let prompt = Confirm::new(msg).prompt_skippable().expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            Ok(v)
        } else {
            Err(())
        }
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()> {
        clear_screen();
        let prompt = Confirm::new(msg)
            .with_default(*self)
            .prompt_skippable()
            .expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            *self = v;
            Ok(())
        } else {
            Err(())
        }
    }
}
impl Promptable for Date {
    fn new_by_prompt(msg: &str) -> Result<Self, ()> {
        clear_screen();
        let prompt = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()
            .expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            Ok(v)
        } else {
            Err(())
        }
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()> {
        clear_screen();
        let prompt = DateSelect::new(msg)
            .with_starting_date(*self)
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()
            .expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            *self = v;
            Ok(())
        } else {
            Err(())
        }
    }
}

/// There is no way to enter a None Date on the inquire prompt.
impl Promptable for Option<Date> {
    fn new_by_prompt(msg: &str) -> Result<Self, ()> {
        clear_screen();
        let prompt = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()
            .expect(INQUIRE_ERROR);
        if let Some(v) = prompt {
            Ok(Some(v))
        } else {
            Err(())
        }
    }
    fn modify_by_prompt(&mut self, msg: &str) -> Result<(), ()> {
        clear_screen();
        let prompt = if let Some(d) = self {
            DateSelect::new(msg)
                .with_starting_date(*d)
                .with_week_start(time::Weekday::Monday)
                .prompt_skippable()
                .expect(INQUIRE_ERROR)
        } else {
            DateSelect::new(msg)
                .with_week_start(time::Weekday::Monday)
                .prompt_skippable()
                .expect(INQUIRE_ERROR)
        };
        if let Some(v) = prompt {
            *self = Some(v);
            Ok(())
        } else {
            Err(())
        }
    }
}

impl_promptable!(char);
impl_promptable!(u8);
impl_promptable!(u16);
impl_promptable!(u32);
impl_promptable!(u64);
impl_promptable!(u128);
impl_promptable!(usize);
impl_promptable!(i8);
impl_promptable!(i16);
impl_promptable!(i32);
impl_promptable!(i64);
impl_promptable!(i128);
impl_promptable!(isize);
impl_promptable!(f32);
impl_promptable!(f64);
impl_promptable!(::std::net::IpAddr);
impl_promptable!(::std::net::Ipv4Addr);
impl_promptable!(::std::net::Ipv6Addr);
impl_promptable!(::std::net::SocketAddrV4);
impl_promptable!(::std::net::SocketAddrV6);
impl_promptable!(::std::num::NonZeroI128);
impl_promptable!(::std::num::NonZeroI64);
impl_promptable!(::std::num::NonZeroI32);
impl_promptable!(::std::num::NonZeroI16);
impl_promptable!(::std::num::NonZeroI8);
impl_promptable!(::std::num::NonZeroIsize);
impl_promptable!(::std::num::NonZeroU128);
impl_promptable!(::std::num::NonZeroU64);
impl_promptable!(::std::num::NonZeroU32);
impl_promptable!(::std::num::NonZeroU16);
impl_promptable!(::std::num::NonZeroU8);
impl_promptable!(::std::num::NonZeroUsize);
