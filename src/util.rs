//! terraform model — infrastructure as code.

#[derive(Debug, Clone)]
pub struct Resource {
    pub type_: String,
    pub name: String,
    pub id: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub resources: Vec<Resource>,
}

impl Default for Model {
    fn default() -> Self { Self { resources: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn plan(&mut self, type_: &str, name: &str) -> bool {
        self.resources.push(Resource { type_: type_.into(), name: name.into(), id: format!("{}-{}", type_, name), state: "planned".into() });
        true
    }

    pub fn apply(&mut self, name: &str) -> bool {
        if let Some(r) = self.resources.iter_mut().find(|r| r.name == name) { r.state = "created".into(); true } else { false }
    }

    pub fn destroy(&mut self, name: &str) -> bool {
        if let Some(pos) = self.resources.iter().position(|r| r.name == name) { self.resources.remove(pos); true } else { false }
    }

    pub fn list(&self) -> &[Resource] { &self.resources }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plan_apply_destroy() {
        let mut t = Model::new();
        assert!(t.plan("aws_instance", "web"));
        assert!(t.apply("web"));
        assert!(t.destroy("web"));
        assert!(t.list().is_empty());
    }
}
