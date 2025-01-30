#[allow(dead_code)]
pub struct Employee {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub rate: f32,
}
impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, phone: {}, email: {}, rate: {}",
            self.name, self.phone, self.email, self.rate
        )
    }
}
