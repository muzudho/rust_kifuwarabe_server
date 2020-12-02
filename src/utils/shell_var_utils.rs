use client_handle_impl::SHELL_VAR_MAP;
use models::shell_var::*;

pub struct ShellVarUtil {}
impl ShellVarUtil {
    pub fn insert(connection_number: i64, shell_var: ShellVar) {
        SHELL_VAR_MAP
            .try_write()
            .unwrap()
            .insert(connection_number, shell_var);
    }
}
