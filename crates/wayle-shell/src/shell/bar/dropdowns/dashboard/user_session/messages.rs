use std::sync::Arc;

use wayle_config::ConfigService;

pub(crate) struct UserSessionInit {
    pub username: String,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum UserSessionInput {
    Lock,
    Logout,
    PowerOff,
}

#[derive(Debug)]
pub(crate) enum UserSessionCmd {
    FaceChanged(bool),
}
