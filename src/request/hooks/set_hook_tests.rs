#[test]
fn set_hook() {
    use crate::{Error, SetHook, SetHookBuilder, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-hook [-agRu] [-t target-session] hook-name command
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-hook", "-a", "-g", "-R", "-u", "-t", "1", "2", "3"]"#
        );
        Err(Error::Hook)
    }));
    let set_hook = SetHook {
        append: Some(true),
        global: Some(true),
        run: Some(true),
        unset: Some(true),
        target_session: Some(&TargetSession::Raw("1")),
    };
    tmux.set_hook(Some(&set_hook), "2", "3").unwrap_err();

    let set_hook = SetHookBuilder::new()
        .append()
        .global()
        .run()
        .unset()
        .target_session(&TargetSession::Raw("1"))
        .build();
    tmux.set_hook(Some(&set_hook), "2", "3").unwrap_err();
}
