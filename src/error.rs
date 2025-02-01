#[derive(Debug, Default)]
pub enum NemoErrorKind {
    Io,
    Parse,
    Invalid,
    Date,
    NotFound,
    Permissions,
    #[default]
    Generic,
}
#[derive(Debug)]
pub struct NemoError {
    kind: NemoErrorKind,
    msg: String,
}
impl NemoError {
    pub fn new(msg: &str) -> Self {
        let msg = msg.to_string();
        Self {
            kind: NemoErrorKind::default(),
            msg,
        }
    }
    pub fn print(&self) {
        println!("Error: {:?}", self.msg);
    }
    pub fn kind(mut self, kind: NemoErrorKind) -> NemoError {
        self.kind = kind;
        self
    }
}
