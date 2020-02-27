use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;

/// Structure for showing options
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct ShowOptions<'a, T: Display> {
    /// [-A] - includes options inherited from a parent set of options
    pub include_inherited: Option<bool>,
    /// [-g] - global session or window options are listed
    pub global: Option<bool>,
    /// [-H] - includes hooks (omitted by default)
    pub hooks: Option<bool>,
    /// [-p] - show window options
    pub pane: Option<bool>,
    /// [-q] - no error will be returned if `option` is unset
    pub quiet: Option<bool>,
    /// [-s] - show the server options
    pub server: Option<bool>,
    /// [-v] - shows only the option value
    pub value: Option<bool>,
    /// [-w] - show the window options
    pub window: Option<bool>,
    /// [-t target-pane] - target session or window name
    pub target: Option<&'a T>,
    /// [option] - specify option name
    pub option: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct ShowOptions<'a, T: Display> {
    /// [-g] - global session or window options are listed
    pub global: Option<bool>,
    /// [-q] - no error will be returned if `option` is unset
    pub quiet: Option<bool>,
    /// [-s] - show the server options
    pub server: Option<bool>,
    /// [-v] - shows only the option value
    pub value: Option<bool>,
    /// [-w] - show the window options
    pub window: Option<bool>,
    /// [-t target-session | target-window] - target session or window name
    pub target: Option<&'a T>,
    /// [option] - specify option name
    pub option: Option<&'a str>,
}

impl<'a, T: Display + Default> ShowOptions<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct ShowOptionsBuilder<'a, T: Display> {
    pub include_inherited: Option<bool>,
    pub global: Option<bool>,
    pub hooks: Option<bool>,
    pub pane: Option<bool>,
    pub quiet: Option<bool>,
    pub server: Option<bool>,
    pub value: Option<bool>,
    pub window: Option<bool>,
    pub target: Option<&'a T>,
    pub option: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> ShowOptionsBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn include_inherited(&mut self) -> &mut Self {
        self.include_inherited = Some(true);
        self
    }

    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    pub fn hooks(&mut self) -> &mut Self {
        self.hooks = Some(true);
        self
    }

    pub fn pane(&mut self) -> &mut Self {
        self.pane = Some(true);
        self
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = Some(true);
        self
    }

    pub fn server(&mut self) -> &mut Self {
        self.server = Some(true);
        self
    }

    pub fn value(&mut self) -> &mut Self {
        self.value = Some(true);
        self
    }

    pub fn window(&mut self) -> &mut Self {
        self.window = Some(true);
        self
    }

    pub fn target(&mut self, target: &'a T) -> &mut Self {
        self.target = Some(target);
        self
    }

    pub fn option(&mut self, option: &'a str) -> &mut Self {
        self.option = Some(option);
        self
    }

    pub fn build(&self) -> ShowOptions<'a, T> {
        ShowOptions {
            include_inherited: self.include_inherited,
            global: self.global,
            hooks: self.hooks,
            pane: self.pane,
            quiet: self.quiet,
            server: self.server,
            value: self.value,
            window: self.window,
            target: self.target,
            option: self.option,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct ShowOptionsBuilder<'a, T: Display> {
    pub global: Option<bool>,
    pub quiet: Option<bool>,
    pub server: Option<bool>,
    pub value: Option<bool>,
    pub window: Option<bool>,
    pub target: Option<&'a T>,
    pub option: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> ShowOptionsBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn global(&mut self) -> &mut Self {
        self.global = Some(true);
        self
    }

    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = Some(true);
        self
    }

    pub fn server(&mut self) -> &mut Self {
        self.server = Some(true);
        self
    }

    pub fn option_value(&mut self) -> &mut Self {
        self.option_value = Some(true);
        self
    }

    pub fn window(&mut self) -> &mut Self {
        self.window = Some(true);
        self
    }

    pub fn target(&mut self, target: &'a T) -> &mut Self {
        self.target = Some(target);
        self
    }

    pub fn option(&mut self, option: &'a str) -> &mut Self {
        self.option = Some(option);
        self
    }

    pub fn build(&self) -> ShowOptions<'a, T> {
        ShowOptions {
            global: self.global,
            quiet: self.quiet,
            server: self.server,
            option_value: self.option_value,
            window: self.window,
            target: self.target,
            option: self.option,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SHOW_OPTIONS: &'static str = "show-options";

    // XXX: better result type?
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
    /// (alias: show)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn show_options<T: Display>(
        &mut self,
        show_options: Option<&ShowOptions<T>>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(show_options) = show_options {
            if show_options.include_inherited.unwrap_or(false) {
                args.push(A_KEY);
            }
            if show_options.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if show_options.hooks.unwrap_or(false) {
                args.push(H_KEY);
            }
            if show_options.pane.unwrap_or(false) {
                args.push(p_KEY);
            }
            if show_options.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if show_options.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            if show_options.value.unwrap_or(false) {
                args.push(v_KEY);
            }
            if show_options.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            if let Some(target) = show_options.target {
                s = target.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = show_options.option {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SHOW_OPTIONS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
    /// (alias: show)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn show_options<T: Display>(
        &mut self,
        show_options: Option<&ShowOptions<T>>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(show_options) = show_options {
            if show_options.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if show_options.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if show_options.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            if show_options.option_value.unwrap_or(false) {
                args.push(v_KEY);
            }
            if show_options.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            if let Some(target) = show_options.target {
                s = target.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = show_options.option {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SHOW_OPTIONS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
