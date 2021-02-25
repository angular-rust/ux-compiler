#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "ux.pest"]
pub struct UxParser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
