#[test]
fn has_session() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux has-session [-t target-session]
        // (alias: has)
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("has-session");
        #[cfg(feature = "use_cmd_alias")]
        s.push("has");
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_session = &TargetSession::Raw("1").to_string();
    tmux.has_session(Some(&target_session)).unwrap_err();
}
