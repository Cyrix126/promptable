#[cfg(test)]
mod tests {
    use promptable::clear_screen;
    use promptable_derive::Promptable;
    use time::Date;
    #[derive(Promptable)]
    struct Test {
        #[promptable(visible = true)]
        #[promptable(name = "nom du champ 2")]
        champ1: String,
        #[promptable(name = "nom du champ 2")]
        #[promptable(visible = true)]
        champ2: Option<u32>,
        #[promptable(visible = true)]
        champ3: Option<Date>,
    }
    #[test]
    fn construct_struct() {
        clear_screen();
        let mut t = Test::new_by_prompt("");
        t.modify_by_prompt("modifier ce struct:");
    }
}
