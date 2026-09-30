use command_extra::CommandExtra;
use std::{collections::BTreeMap, ffi::OsString, process::Command};

#[test]
fn with_envs() {
    fn _owned_map(cmd: Command, envs: BTreeMap<String, String>) -> Command {
        cmd.with_envs(envs)
    }
    fn _borrowed_map(cmd: Command, envs: &BTreeMap<String, String>) -> Command {
        cmd.with_envs(envs)
    }
    fn _map_iter(cmd: Command, envs: &BTreeMap<String, String>) -> Command {
        cmd.with_envs(envs.iter())
    }
    fn _owned_pairs(cmd: Command, envs: Vec<(String, String)>) -> Command {
        cmd.with_envs(envs)
    }
    fn _owned_array(cmd: Command) -> Command {
        cmd.with_envs([("A", "1"), ("B", "2")])
    }
    fn _mixed_types(cmd: Command, envs: Vec<(String, &'static str)>) -> Command {
        cmd.with_envs(envs)
    }
    fn _os_strings(cmd: Command, envs: Vec<(OsString, OsString)>) -> Command {
        cmd.with_envs(envs)
    }
    fn _borrowed_slice(cmd: Command, envs: &[(String, String)]) -> Command {
        cmd.with_envs(envs)
    }
    fn _borrowed_vec(cmd: Command, envs: &Vec<(String, String)>) -> Command {
        cmd.with_envs(envs)
    }
    fn _vec_iter(cmd: Command, envs: Vec<(String, String)>) -> Command {
        cmd.with_envs(envs.iter())
    }
    fn _vec_of_references(cmd: Command, envs: Vec<&(String, String)>) -> Command {
        cmd.with_envs(envs)
    }
    fn _array_iter(cmd: Command) -> Command {
        cmd.with_envs([("A", "1"), ("B", "2")].iter())
    }
}
