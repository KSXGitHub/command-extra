# Command Extra

[![Test](https://github.com/KSXGitHub/command-extra/workflows/Test/badge.svg)](https://github.com/KSXGitHub/command-extra/actions?query=workflow%3ATest)
[![Travis Build Status](https://img.shields.io/travis/KSXGitHub/command-extra/master?label=build&logo=travis)](https://travis-ci.org/KSXGitHub/command-extra)
[![Crates.io Version](https://img.shields.io/crates/v/command-extra?logo=rust)](https://crates.io/crates/command-extra)
[![Documentation](https://docs.rs/command-extra/badge.svg)](https://docs.rs/command-extra)

Additional methods for `std::process::Command`.

## Motivation

Default `Command` mutation methods take a mutable reference and return a mutable reference, making sharing code verbose:

```rust
fn shared_command() -> Command {
    let mut command = Command::new("command");
    command
        .current_dir("work-dir")
        .env("FOO", "foo")
        .arg("bar");
    command
}
```

With `CommandExtra`, the above code can be shorter:

```rust
fn shared_command() -> Command {
    Command::new("command")
        .with_current_dir("work-dir")
        .with_env("FOO", "foo")
        .with_arg("bar")
}
```

## License

[MIT](https://git.io/JTmyt) © [Hoàng Văn Khải](https://KSXGitHub.github.io/)
