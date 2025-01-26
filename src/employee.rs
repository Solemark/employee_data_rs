#[allow(dead_code)]
pub struct Employee {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub rate: f32,
}
impl Employee {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_string(&self) -> String {
        format!(
            "name: {}, phone: {}, email: {}, rate: {}",
            self.name, self.phone, self.email, self.rate
        )
    }
}
