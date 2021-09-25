use super::lambda::Lambda;
use position::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    name: String,
    original_name: String,
    lambda: Lambda,
    foreign: bool,
    public: bool,
    position: Position,
}

impl Definition {
    pub fn new(
        name: impl Into<String>,
        original_name: impl Into<String>,
        lambda: Lambda,
        foreign: bool,
        public: bool,
        position: Position,
    ) -> Self {
        Self {
            name: name.into(),
            original_name: original_name.into(),
            lambda,
            foreign,
            public,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    pub fn lambda(&self) -> &Lambda {
        &self.lambda
    }

    pub fn is_foreign(&self) -> bool {
        self.foreign
    }

    pub fn is_public(&self) -> bool {
        self.public
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}