use std::sync::Arc;

use wayle_traits::ModelMonitoring;

use super::Workspace;
use crate::Error;

impl ModelMonitoring for Workspace {
    type Error = Error;
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        todo!()
    }
}
