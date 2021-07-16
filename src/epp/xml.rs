pub trait EppXml {
    fn serialize(&self) -> Result<String, Box<dyn Error>>;
    fn deserialize(&self) -> Result<Self, Box<dyn Error>>;
}
