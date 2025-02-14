use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy)]
pub struct NemoError {
    kind: NemoErrorKind,
    msg: &'static str,
}
impl NemoError {
    pub fn new(msg: &'static str) -> Self {
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
impl NemoErrorKind {
    pub fn as_str(&self) -> String {
        stringify!("{}", self.kind).to_string()
    }
}
impl From<String> for NemoErrorKind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "io" => NemoErrorKind::Io,
            "parse" => NemoErrorKind::Parse,
            "invalid" => NemoErrorKind::Invalid,
            "date" => NemoErrorKind::Date,
            "notfound" => NemoErrorKind::NotFound,
            "permsissions" => NemoErrorKind::Permissions,
            "generic" | _ => NemoErrorKind::Generic
        }
    }
}
impl From<&str> for NemoErrorKind {
    fn from(value: &str) -> Self {
        let val = value.to_string();
        NemoErrorKind::from(val)
    }
}
impl Into<String> for NemoErrorKind {
    fn into(self) -> String {
        let res = match self {
            NemoErrorKind::Io => "io",
            NemoErrorKind::Parse => "parse",
            NemoErrorKind::Invalid => "invalid",
            _ => "generic"
        };
        res.into()
    }
}
impl Display for NemoErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let errtype = self.as_str();
        write!(f, "{}", errtype)
    }
}
impl Display for NemoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error type: {}", self.kind)?;
        writeln!(f, "Error msg:  {}", self.msg)?;
        Ok(())
    }
}