#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use inquire::{required, Confirm, CustomType, DateSelect, Text};
pub extern crate promptable_derive;
use termion::{clear::All, cursor::Goto};
use time::{Date, OffsetDateTime};

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
///let mut anwser = Date::new_by_prompt("choose the date");
///anwser.modify_by_prompt("change the date");
///```
pub trait Promptable {
    /// method to create a value.
    fn new_by_prompt(msg: &str) -> Self;
    /// modify the self value.
    fn modify_by_prompt(&mut self, msg: &str);
}

#[doc(hidden)]
/// macro to implement quicly the Promptable trait on basic types.
/// use of CustomType to re-ask user if input is incorrect.
macro_rules! impl_promptable {
    ($t:ty) => {
        impl Promptable for $t {
            /// ask to create a new value, msg can transmit the message of the prompt.
            fn new_by_prompt(msg: &str) -> Self {
                clear_screen();
                CustomType::<$t>::new(msg).prompt().unwrap()
            }
            /// ask to modify the value, msg can transmit the message of the prompt.
            fn modify_by_prompt(&mut self, msg: &str) {
                clear_screen();
                *self = CustomType::<$t>::new(msg)
                    .with_default(self.clone())
                    .with_placeholder(&self.to_string())
                    .prompt()
                    .unwrap();
            }
        }
        impl Promptable for Option<$t> {
            /// ask to optionnaly create a value, msg can transmit the message of the prompt.
            fn new_by_prompt(msg: &str) -> Self {
                clear_screen();
                CustomType::<$t>::new(msg).prompt_skippable().unwrap()
            }
            /// ask to modify a value, can return None, msg can transmit the message of the prompt.
            fn modify_by_prompt(&mut self, msg: &str) {
                clear_screen();
                if let Some(s) = self {
                    *self = CustomType::<$t>::new(msg)
                        .with_default(s.clone())
                        .with_placeholder(&s.to_string())
                        .prompt_skippable()
                        .unwrap();
                } else {
                    *self = CustomType::<$t>::new(msg).prompt_skippable().unwrap();
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
    fn new_by_prompt(msg: &str) -> Self {
        clear_screen();
        Text::new(msg)
            .with_validator(required!("This field is required"))
            .prompt()
            .unwrap()
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        clear_screen();
        *self = Text::new(msg)
            .with_validator(required!("This field is required"))
            .with_initial_value(self)
            .prompt()
            .unwrap();
    }
}
/// String can be used with the Text type of inquire
impl Promptable for Option<String> {
    fn new_by_prompt(msg: &str) -> Self {
        clear_screen();
        Text::new(msg).prompt_skippable().unwrap()
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        clear_screen();
        if let Some(s) = self {
            *self = Text::new(msg)
                .with_initial_value(s)
                .prompt_skippable()
                .unwrap();
        } else {
            *self = Text::new(msg).prompt_skippable().unwrap();
        }
    }
}
/// bool can be used with the Confirm type of inquire
impl Promptable for bool {
    ///
    fn new_by_prompt(msg: &str) -> Self {
        Confirm::new(msg).prompt().unwrap()
    }
    ///
    fn modify_by_prompt(&mut self, msg: &str) {
        clear_screen();
        *self = Confirm::new(msg).with_default(*self).prompt().unwrap();
    }
}
impl Promptable for Date {
    fn new_by_prompt(msg: &str) -> Self {
        clear_screen();
        DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt()
            .unwrap()
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        clear_screen();
        *self = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt()
            .unwrap();
    }
}

/// Date can be used with the DateSelect type of inquire
impl Promptable for Option<Date> {
    fn new_by_prompt(msg: &str) -> Self {
        clear_screen();
        DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()
            .unwrap()
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        clear_screen();
        *self = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()
            .unwrap();
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
