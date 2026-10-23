use std::process::Command;

use penrose::{
    core::{hooks::StateHook, State},
    x::XConn,
    Result,
};
use tracing::warn;

/// Executa um script quando o window manager inicia.
pub struct StartupScript {
    path: String,
}

impl StartupScript {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: expand_home(&path.into()),
        }
    }
}

/// Expande `$HOME` ou `~` no começo do caminho (não há shell envolvido).
fn expand_home(path: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    if let Some(rest) = path.strip_prefix("$HOME") {
        format!("{home}{rest}")
    } else if let Some(rest) = path.strip_prefix('~') {
        format!("{home}{rest}")
    } else {
        path.to_string()
    }
}

impl<X: XConn> StateHook<X> for StartupScript {
    fn call(&mut self, _: &mut State<X>, _: &X) -> Result<()> {
        // Falha no script não deve derrubar o WM: só registramos o aviso.
        // O penrose ignora SIGCHLD, então não precisamos esperar o processo filho.
        if let Err(e) = Command::new(&self.path).spawn() {
            warn!(path = %self.path, error = %e, "unable to run startup script");
        }
        Ok(())
    }
}
