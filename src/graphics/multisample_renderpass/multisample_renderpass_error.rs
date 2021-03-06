use ::thiserror::Error;

use crate::graphics::vulkan::errors::VulkanError;

#[derive(Debug, Error)]
pub enum MultisampleRenderpassError {
    #[error("Unable to pick a supported depth format")]
    UnableToPickDepthFormat,

    #[error(transparent)]
    UnexpectedVulkanError(#[from] VulkanError),
}
