pub trait ResultExt<T, E> {
    fn ctx<M>(self, msg: M) -> Result<T, String>
    where
        M: Into<String>,
        E: std::fmt::Display;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn ctx<M>(self, msg: M) -> Result<T, String>
    where
        M: Into<String>,
        E: std::fmt::Display,
    {
        self.map_err(|e| format!("{}: {}", msg.into(), e))
    }
}