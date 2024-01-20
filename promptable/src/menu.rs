use anyhow::Result;
use inquire::Confirm;

use crate::display::PromptableDisplay;

/// Struct Menu Classic will offer consts to make fast menu to manage a Vec.
pub struct MenuClassic;
impl MenuClassic {
    /// const ADD
    pub const ADD: &'static str = "Add";
    /// const MODIFY
    pub const MODIFY: &'static str = "Modify";
    /// const DELETE
    pub const DELETE: &'static str = "Delete";
    /// const CANCEL
    pub const CANCEL: &'static str = "Cancel";
    /// const CONFIRM
    pub const CONFIRM: &'static str = "Confirm";
    /// this method allows get all consts of the structs.
    pub fn consts() -> &'static [&'static str] {
        &[
            MenuClassic::ADD,
            MenuClassic::MODIFY,
            MenuClassic::DELETE,
            MenuClassic::CANCEL,
            MenuClassic::CONFIRM,
        ]
    }
}

/// this function can be used for prompting the user in a easy way for him to see the change before confirming.
pub fn menu_confirm<T: PromptableDisplay>(item_old: &T, item_new: &T) -> Result<bool> {
    println!("Differences between versions:\n");
    item_old.display_diff(item_new);
    Ok((Confirm::new("Do you wish to apply the modification ?")
        .with_default(true)
        .prompt_skippable()?)
    .unwrap_or(false))
}

/// this function allows to revert change.
pub fn menu_cancel<T: Clone>(item_restore: &T, item: &mut T) -> Result<bool> {
    if let Some(c) = inquire::Confirm::new("Do you wish to abandon the modification on this ?")
        .with_default(true)
        .prompt_skippable()?
    {
        if c {
            *item = item_restore.clone();
            return Ok(true);
        }
    }
    Ok(false)
}
