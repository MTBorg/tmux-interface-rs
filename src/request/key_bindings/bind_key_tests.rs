#[test]
fn bind_key() {
    use crate::{BindKey, BindKeyBuilder, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux bind-key [-nr] [-T key-table] key command [arguments]
        // (alias: bind)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["bind-key", "-n", "-r", "-T", "1", "2", "3", "4"]"#
        );
        Err(Error::Hook)
    }));

    let bind_key = BindKey {
        root: Some(true),
        repeat: Some(true),
        key_table: Some("1"),
        arguments: Some("4"),
    };
    tmux.bind_key(Some(&bind_key), "2", "3").unwrap_err();

    let bind_key = BindKeyBuilder::new()
        .root()
        .repeat()
        .key_table("1")
        .arguments("4")
        .build();
    tmux.bind_key(Some(&bind_key), "2", "3").unwrap_err();
}
