use libpulse_binding::context::subscribe::Operation;

use crate::services::audio::backend::types::{InternalCommand, InternalCommandSender};

pub(crate) async fn handle_server_change(operation: Operation, command_tx: &InternalCommandSender) {
    if operation == Operation::Changed {
        let _ = command_tx.send(InternalCommand::RefreshServerInfo);
    }
}
