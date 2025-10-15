use std::sync::Arc;

use wayle_traits::ModelMonitoring;

use super::Client;
use crate::Error;

impl ModelMonitoring for Client {
    type Error = Error;
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        todo!()
    }
}
