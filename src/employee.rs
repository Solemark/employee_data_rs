#[allow(dead_code)]
#[derive(Debug)]
pub struct Employee {
    pub(crate) name: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) rate: f32,
}
pub trait EmpFunctions {
    fn get_name(&self) -> String;
}
impl EmpFunctions for Employee {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
