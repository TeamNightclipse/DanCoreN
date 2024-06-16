use std::fmt::{Debug, Formatter};

pub struct Form {
    id: &'static str,
    texture: (&'static str, &'static str),
    //client_form:
}
impl Form {
    pub const SPHERE: Form = Form {
        id: "sphere",
        texture: ("DanmakuCore", "Sphere")
    };
}

impl Debug for Form {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Form({})", self.id)
    }
}