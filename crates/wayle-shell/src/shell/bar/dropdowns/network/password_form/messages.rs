#[derive(Debug)]
pub(crate) enum PasswordFormInput {
    Show {
        ssid: String,
        security_label: String,
        signal_icon: &'static str,
        error_message: Option<String>,
    },
    ConnectClicked,
    CancelClicked,
}

#[derive(Debug)]
pub(crate) enum PasswordFormOutput {
    Connect { password: String },
    Cancel,
}
