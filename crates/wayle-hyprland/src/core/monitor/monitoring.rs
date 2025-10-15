use std::sync::Arc;

use wayle_traits::ModelMonitoring;

use super::Monitor;
use crate::Error;

impl ModelMonitoring for Monitor {
    type Error = Error;
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        todo!()
    }
}
