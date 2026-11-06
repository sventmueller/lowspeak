//! "Start at Login" through SMAppService; macOS keeps the registration itself.

use anyhow::{Result, anyhow};
use objc2_service_management::{SMAppService, SMAppServiceStatus};

pub fn is_enabled() -> bool {
    // SAFETY: querying the main app's own service registration.
    unsafe { SMAppService::mainAppService().status() == SMAppServiceStatus::Enabled }
}

pub fn set_enabled(enabled: bool) -> Result<()> {
    // SAFETY: registering or unregistering the main app's own service.
    let result = unsafe {
        let service = SMAppService::mainAppService();
        if enabled {
            service.registerAndReturnError()
        } else {
            service.unregisterAndReturnError()
        }
    };
    result.map_err(|error| anyhow!("{}", error.localizedDescription()))
}
