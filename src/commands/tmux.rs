use crate::commands::constants::*;
use crate::commands::tmux_command::TmuxCommand;
use crate::commands::tmux_commands::TmuxCommands;
use std::borrow::Cow;

// NOTE: [-N] missing in man
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux [-2lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux [-28lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux [-28lquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.1:
/// ```text
/// tmux [-28lquv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux [-28dlqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux [-28dqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux [-28dqUuVv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
// XXX: using environment vars
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Tmux<'a> {
    /// `[-2]` - Force tmux to assume the terminal supports 256 colours
    #[cfg(feature = "tmux_0_8")]
    pub colours256: bool,

    /// `[-8]` - indicates that tmux supports 88 colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub colours88: bool,

    /// `[-d]` - indicates that tmux supports defaults colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub default_colours: bool,

    /// `[-q]` - prevent the server sending various information messages
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    pub prevent_msg: bool,

    /// `[-C]` - Start in control mode
    #[cfg(feature = "tmux_1_8")]
    pub control_mode: bool,

    /// `[-CC]` - Disable echo
    #[cfg(feature = "tmux_1_8")]
    pub disable_echo: bool,

    /// `[-D]` - Do not start the tmux server as a daemon. This also turns the exit-empty option off.  With -D, command may not be specified.
    #[cfg(feature = "tmux_3_2")]
    pub no_daemon: bool,

    /// `[-l]` - Behave as a login shell
    #[cfg(feature = "tmux_1_0")]
    pub login_shell: bool,

    /// `[-N]` - Do not start the server even if the command would normally do so (for example new-session or start-server).
    #[cfg(feature = "tmux_3_2")]
    pub no_start: bool,

    /// `[-U]` - Unlock the server
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub unlock: bool,

    /// `[-u]` - Write UTF-8 output to the terminal
    #[cfg(feature = "tmux_0_8")]
    pub force_utf8: bool,

    /// `[-v]` - Request verbose logging
    #[cfg(feature = "tmux_0_8")]
    pub verbose_logging: bool,

    /// `[-V]` - Report the tmux version
    #[cfg(feature = "tmux_0_8")]
    pub version: bool,

    /// `[-c shell-command]` - Execute shell-command using the default shell
    #[cfg(feature = "tmux_1_1")]
    pub shell_cmd: Option<Cow<'a, str>>,

    /// `[-f file]` - Specify an alternative configuration file
    #[cfg(feature = "tmux_0_8")]
    pub file: Option<Cow<'a, str>>,

    /// `[-L socket-name]` - Allow a different socket name to be specified
    #[cfg(feature = "tmux_0_8")]
    pub socket_name: Option<Cow<'a, str>>,

    /// `[-S socket-path]` - Specify a full alternative path to the server socket
    #[cfg(feature = "tmux_0_8")]
    pub socket_path: Option<Cow<'a, str>>,

    /// `[-T features]` - Set terminal features for the client
    #[cfg(feature = "tmux_3_2")]
    pub features: Option<Cow<'a, str>>,

    pub command: Option<TmuxCommands<'a>>,
}

impl<'a> Tmux<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-2]` - Force tmux to assume the terminal supports 256 colours
    #[cfg(feature = "tmux_0_8")]
    pub fn colours256(mut self) -> Self {
        self.colours256 = true;
        self
    }

    /// `[-8]` - indicates that tmux supports 88 colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub fn colours88(mut self) -> Self {
        self.colours88 = true;
        self
    }

    /// `[-d]` - indicates that tmux supports defaults colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub fn default_colours(mut self) -> Self {
        self.default_colours = true;
        self
    }

    /// `[-q]` - prevent the server sending various information messages
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    pub fn prevent_msg(mut self) -> Self {
        self.prevent_msg = true;
        self
    }

    /// `[-C]` - Start in control mode
    #[cfg(feature = "tmux_1_8")]
    pub fn control_mode(mut self) -> Self {
        self.control_mode = true;
        self
    }

    /// `[-CC]` - Disable echo
    #[cfg(feature = "tmux_1_8")]
    pub fn disable_echo(mut self) -> Self {
        self.disable_echo = true;
        self
    }

    /// `[-D]` - Do not start the tmux server as a daemon. This also turns the exit-empty option off.  With -D, command may not be specified.
    #[cfg(feature = "tmux_3_2")]
    pub fn no_daemon(mut self) -> Self {
        self.no_daemon = true;
        self
    }

    /// `[-l]` - Behave as a login shell
    #[cfg(feature = "tmux_1_0")]
    pub fn login_shell(mut self) -> Self {
        self.login_shell = true;
        self
    }

    /// `[-N]` - Do not start the server even if the command would normally do so (for example new-session or start-server).
    #[cfg(feature = "tmux_3_2")]
    pub fn no_start(mut self) -> Self {
        self.no_start = true;
        self
    }

    /// `[-U]` - Unlock the server
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub fn unlock(mut self) -> Self {
        self.unlock = true;
        self
    }

    /// `[-u]` - Write UTF-8 output to the terminal
    #[cfg(feature = "tmux_0_8")]
    pub fn force_utf8(mut self) -> Self {
        self.force_utf8 = true;
        self
    }

    /// `[-v]` - Request verbose logging
    #[cfg(feature = "tmux_0_8")]
    pub fn verbose_logging(mut self) -> Self {
        self.verbose_logging = true;
        self
    }

    /// `[-V]` - Report the tmux version
    #[cfg(feature = "tmux_0_8")]
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// `[-c shell-command]` - Execute shell-command using the default shell
    #[cfg(feature = "tmux_1_1")]
    pub fn shell_cmd<S: Into<Cow<'a, str>>>(mut self, shell_cmd: S) -> Self {
        self.shell_cmd = Some(shell_cmd.into());
        self
    }

    /// `[-f file]` - Specify an alternative configuration file
    #[cfg(feature = "tmux_0_8")]
    pub fn file<S: Into<Cow<'a, str>>>(mut self, file: S) -> Self {
        self.file = Some(file.into());
        self
    }

    /// `[-L socket-name]` - Allow a different socket name to be specified
    #[cfg(feature = "tmux_0_8")]
    pub fn socket_name<S: Into<Cow<'a, str>>>(mut self, socket_name: S) -> Self {
        self.socket_name = Some(socket_name.into());
        self
    }

    /// `[-S socket-path]` - Specify a full alternative path to the server socket
    #[cfg(feature = "tmux_0_8")]
    pub fn socket_path<S: Into<Cow<'a, str>>>(mut self, socket_path: S) -> Self {
        self.socket_path = Some(socket_path.into());
        self
    }

    /// `[-T features]` - Set terminal features for the client
    #[cfg(feature = "tmux_3_2")]
    pub fn features(mut self, features: S) -> Self {
        self.features = Some(features.into());
        self
    }

    pub fn command(mut self, command: TmuxCommand<'a>) -> Self {
        self.command
            .get_or_insert(TmuxCommands::new())
            .push(command);
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(TMUX);

        // `[-2]` - Force tmux to assume the terminal supports 256 colours
        #[cfg(feature = "tmux_0_8")]
        if self.colours256 {
            cmd.push_flag(_2_KEY);
        }

        // `[-8]` - indicates that tmux supports 88 colours
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
        if self.colours88 {
            cmd.push_flag(_8_KEY);
        }

        // `[-d]` - indicates that tmux supports defaults colours
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
        if self.default_colours {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-q]` - prevent the server sending various information messages
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
        if self.prevent_msg {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-C]` - Start in control mode
        #[cfg(feature = "tmux_1_8")]
        if self.control_mode {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-CC]` - Disable echo
        #[cfg(feature = "tmux_1_8")]
        if self.disable_echo {
            cmd.push_flag(CC_UPPERCASE_KEY);
        }

        // `[-D]` - Do not start the tmux server as a daemon. This also turns the exit-empty option off.  With -D, command may not be specified.
        #[cfg(feature = "tmux_3_2")]
        if self.no_daemon {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-l]` - Behave as a login shell
        #[cfg(feature = "tmux_1_0")]
        if self.login_shell {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-N]` - Do not start the server even if the command would normally do so (for example new-session or start-server).
        #[cfg(feature = "tmux_3_2")]
        if self.no_start {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-U]` - Unlock the server
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
        if self.unlock {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-u]` - Write UTF-8 output to the terminal
        #[cfg(feature = "tmux_0_8")]
        if self.force_utf8 {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-v]` - Request verbose logging
        #[cfg(feature = "tmux_0_8")]
        if self.verbose_logging {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-V]` - Report the tmux version
        #[cfg(feature = "tmux_0_8")]
        if self.version {
            cmd.push_flag(V_UPPERCASE_KEY);
        }

        // `[-c shell-command]` - Execute shell-command using the default shell
        #[cfg(feature = "tmux_1_1")]
        if let Some(shell_cmd) = self.shell_cmd {
            cmd.push_option(C_LOWERCASE_KEY, shell_cmd);
        }

        // `[-f file]` - Specify an alternative configuration file
        #[cfg(feature = "tmux_0_8")]
        if let Some(file) = self.file {
            cmd.push_option(F_LOWERCASE_KEY, file);
        }

        // `[-L socket-name]` - Allow a different socket name to be specified
        #[cfg(feature = "tmux_0_8")]
        if let Some(socket_name) = self.socket_name {
            cmd.push_option(L_UPPERCASE_KEY, socket_name);
        }

        // `[-S socket-path]` - Specify a full alternative path to the server socket
        #[cfg(feature = "tmux_0_8")]
        if let Some(socket_path) = self.socket_path {
            cmd.push_option(S_UPPERCASE_KEY, socket_path);
        }

        // `[-T features]` - Set terminal features for the client
        #[cfg(feature = "tmux_3_2")]
        if let Some(features) = self.features {
            cmd.push_option(T_UPPERCASE_KEY, features);
        }

        if let Some(command) = self.command {
            cmd.push_cmds(command);
        }

        cmd
    }

    //// run tmux command
    //pub fn output(&self) -> Result<TmuxOutput, Error> {
    //let mut command = Command::from(self);
    //// NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //command
    //.stdin(Stdio::inherit())
    ////.stdin(Stdio::piped())
    //.stdout(Stdio::piped());
    ////.spawn()
    //dbg!(&command);
    //let output = command.output()?;
    //Ok(TmuxOutput(output))
    //}

    //// run tmux command
    //pub fn spawn(&self) -> Result<Child, Error> {
    //let mut command = Command::from(self);
    //// NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //command
    ////.stdin(Stdio::inherit())
    //.stdin(Stdio::piped())
    //.stdout(Stdio::piped());
    ////.spawn()
    //dbg!(&command);
    //let child = command.spawn()?;
    //Ok(child)
    //}
}

//impl<'a> From<&Tmux<'a>> for Command {
//fn from(tmux: &Tmux) -> Self {
////let mut command = Command::new(TMUX);

//let cmd = match &tmux.0.name {
//Some(cmd) => cmd,
//None => &Cow::Borrowed(TMUX),
//};
//let mut command = Command::new(cmd.as_ref());

////// XXX: ugly?
////if let Some(v) = &tmux.bin_args {
////for a in v {
////command.arg(a.as_ref());
////}
////}

//// XXX: ugly?
//if let Some(v) = &tmux.0.args {
//for a in v {
//command.arg(a.as_ref());
//}
//}

//command
//}
//}
