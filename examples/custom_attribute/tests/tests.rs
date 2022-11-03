use custom_attribute::custom_attr;

// Using the value is not covered on purpose

#[custom_attr(foo)]
mod single_attr {}

#[custom_attr(foo, bar)]
mod multiple_names {}

#[custom_attr(foo)]
#[custom_attr(bar)]
mod multiple_attrs {}

#[custom_attr(bar=true, baz=false)]
mod attrs_with_value {}

mod test {
    #[test]
    fn single_attr() {
        assert!(super::single_attr::foo());
    }

    #[test]
    fn multiple_names() {
        assert!(super::multiple_names::foo());
        assert!(super::multiple_names::bar());
    }

    #[test]
    fn multiple_attrs() {
        assert!(super::multiple_attrs::foo());
        assert!(super::multiple_attrs::bar());
    }

    #[test]
    fn attrs_with_value() {
        assert!(super::attrs_with_value::bar());
        assert!(!super::attrs_with_value::baz());
    }
}
