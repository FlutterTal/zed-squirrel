use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct SquirrelExtension {}

impl zed::Extension for SquirrelExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which("squirrel-language-server").ok_or_else(|| "Please install the Squirrel LSP and make sure it's present on $PATH".to_string())?;

        /*let lsp_settings = zed::settings::LspSettings::for_worktree("squirrel_language_server", &worktree);
        let mut args = None;

        if let Ok(lsp_settings) = lsp_settings {
            if let Some(binary) = lsp_settings.binary {
                args = binary.arguments;
            }
        }*/

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default()
        })
    }
}

zed::register_extension!(SquirrelExtension);
