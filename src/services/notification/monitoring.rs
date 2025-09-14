use super::error::Error;
use crate::services::traits::ServiceMonitoring;

pub(crate) struct NotificationServiceMonitor;

impl ServiceMonitoring for NotificationServiceMonitor {
    type Error = Error;
    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
