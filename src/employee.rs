#[allow(dead_code)]
pub struct Employee {
    pub(crate) name: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) rate: f32,
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
