#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("illegal state")]
    IllegalState,
    #[error("no value found")]
    NoValue,
    #[error("database error")]
    Database(#[from] diesel::result::Error),
}

impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
