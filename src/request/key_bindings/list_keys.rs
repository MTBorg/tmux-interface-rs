use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LIST_KEYS: &'static str = "list-keys";

    /// # Manual
    ///
    /// ```text
    /// tmux list-keys [-T key-table]
    /// (alias: lsk)
    /// ```
    pub fn list_keys(&mut self, key_table: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = key_table {
            args.extend_from_slice(&[T_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_KEYS, &args)?;
        Ok(output)
    }
}
