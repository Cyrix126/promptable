#[cfg(test)]
mod tests {

    use inquire::Select;
    use promptable::clear_screen;
    use promptable_derive::Promptable;
    use std::fmt::Display;
    use time::Date;
    #[derive(Promptable, Clone)]
    #[prompt(params = "")]
    struct Test {
        // #[promptable(visible = true)]
        #[promptable(function_new = "test()")]
        #[promptable(function_modify = "test_m(self.champ1.clone())")]
        champ1: String,
        #[promptable(name = "nom du champ 2")]
        #[promptable(visible = true)]
        champ2: Option<u32>,
        #[promptable(default = true)]
        champ3: Option<Date>,
    }

    impl Display for Test {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.champ1)
        }
    }
    #[test]
    fn construct_struct() {
        clear_screen();
        let mut t = Test::new_by_prompt();
        t.modify_by_prompt();
    }

    fn test() -> String {
        "test".to_string()
    }
    fn test_m(value: String) -> String {
        "test".to_string()
    }
}
