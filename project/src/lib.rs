

mod itemtype;
mod utility;


#[cfg(test)]
mod tests {
    #[test]
    fn itemtype_works() {
        //! ::itemtype::ItemType, first :: appear!
        let mut x : ::itemtype::ItemType = ::itemtype::ItemType::new("abc".to_string(), "def".to_string());
        
        assert!(*x.get_name() == "abc".to_string());
        assert!(*x.get_value() == "def".to_string());
        let y : String = String::from("DEF");
        x.set_name("ABC".to_string());
        x.set_value(y); //"DEF".to_string());

        assert!(*x.get_name() == "ABC".to_string());
        assert!(*x.get_value() == "DEF".to_string());
    }

    #[test]
    fn usertype_works() {

    }

    #[test]
    fn utility_works() {
           
    }
}
