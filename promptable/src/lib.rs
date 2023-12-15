use inquire::{Confirm, CustomType, DateSelect, Text};
pub extern crate promptable_derive;
use termion::{clear::All, cursor::Goto};
use time::{Date, OffsetDateTime};
pub fn clear_screen() {
    println!("{}{}", All, Goto(1, 1));
}

pub trait Promptable {
    // Required method
    fn new_by_prompt(msg: &str) -> Self;
    fn modify_by_prompt(&mut self, msg: &str);
}

// use of CustomType to re-ask user if input is incorrect.
macro_rules! impl_promptable {
    ($t:ty) => {
        impl Promptable for $t {
            fn new_by_prompt(msg: &str) -> Self {
                let resp = CustomType::<$t>::new(msg).prompt().unwrap();
                clear_screen();
                return resp;
            }
            fn modify_by_prompt(&mut self, msg: &str) {
                *self = CustomType::<$t>::new(msg)
                    .with_default(self.clone())
                    .with_placeholder(&self.to_string())
                    .prompt()
                    .unwrap();
                clear_screen();
            }
        }
        impl Promptable for Option<$t> {
            fn new_by_prompt(msg: &str) -> Self {
                let resp = CustomType::<$t>::new(msg).prompt_skippable().unwrap();
                clear_screen();
                return resp;
            }
            fn modify_by_prompt(&mut self, msg: &str) {
                if let Some(s) = self {
                    let resp = CustomType::<$t>::new(msg)
                        .with_default(s.clone())
                        .with_placeholder(&s.to_string())
                        .prompt_skippable()
                        .unwrap();
                    *self = resp;
                } else {
                    let resp = CustomType::<$t>::new(msg).prompt_skippable().unwrap();
                    *self = resp;
                }
                clear_screen();
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

impl Promptable for String {
    fn new_by_prompt(msg: &str) -> Self {
        let resp = Text::new(msg).prompt().unwrap();
        clear_screen();
        return resp;
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        *self = Text::new(msg).with_initial_value(&self).prompt().unwrap();
        clear_screen();
    }
}
impl Promptable for Option<String> {
    fn new_by_prompt(msg: &str) -> Self {
        let resp = Text::new(msg).prompt_skippable().unwrap();
        clear_screen();
        return resp;
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        if let Some(s) = self {
            *self = Text::new(msg)
                .with_initial_value(&s)
                .prompt_skippable()
                .unwrap();
        } else {
            let resp = Text::new(msg).prompt_skippable().unwrap();
            *self = resp;
        }
        clear_screen();
    }
}
impl Promptable for bool {
    fn new_by_prompt(msg: &str) -> Self {
        let resp = Confirm::new(msg).prompt().unwrap();
        clear_screen();
        return resp;
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        *self = Confirm::new(msg).with_default(*self).prompt().unwrap();
        clear_screen();
    }
}
impl Promptable for Date {
    fn new_by_prompt(msg: &str) -> Self {
        let date = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt()
            .unwrap();
        clear_screen();
        return date;
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        *self = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt()
            .unwrap();
        clear_screen();
    }
}
impl Promptable for Option<Date> {
    fn new_by_prompt(msg: &str) -> Self {
        let date = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()
            .unwrap();
        clear_screen();
        return date;
    }
    fn modify_by_prompt(&mut self, msg: &str) {
        *self = DateSelect::new(msg)
            .with_starting_date(OffsetDateTime::now_utc().date())
            .with_week_start(time::Weekday::Monday)
            .prompt_skippable()
            .unwrap();
        clear_screen();
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
