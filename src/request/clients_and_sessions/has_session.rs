use crate::error::Error;
use crate::tmux_interface::*;

impl<'a> TmuxInterface<'a> {
    const HAS_SESSION: &'static str = "has-session";

    // XXX: better result return?
    /// Report if the specified session exist
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux has-session [-t target-session]
    /// (alias: has)
    /// ```
    pub fn has_session(&mut self, target_session: Option<&str>) -> Result<bool, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::HAS_SESSION, &args)?;
        Ok(output.status.success())
    }
}
