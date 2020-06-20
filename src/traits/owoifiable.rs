pub trait Owoifiable {
    fn owoify(&self) -> String;
}

impl Owoifiable for String {
    fn owoify(&self) -> String {
        unimplemented!()
    }
}

impl Owoifiable for &str {
    fn owoify(&self) -> String {
        unimplemented!()
    }
}