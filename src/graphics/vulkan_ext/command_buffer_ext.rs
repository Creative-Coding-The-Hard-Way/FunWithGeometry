use ::ash::vk;

use crate::graphics::{
    vulkan::CommandBuffer,
    vulkan_ext::{CommandBufferExtError, CommandResult},
};

/// Command buffer convenience methods.
pub trait CommandBufferExt {
    /// Begin recording commands into the command buffer with the
    /// `ONE_TIME_SUBMIT` flag set.
    ///
    /// # Safety
    ///
    /// Unsafe because the application must ensure this method is called only
    /// once for the given command buffer.
    unsafe fn begin_one_time_submit(&self) -> CommandResult<&Self>;

    /// Finish recording commands into this command buffer.
    ///
    /// # Safety
    ///
    /// Unsafe because the application must ensure this method is called before
    /// submitting the command buffer.
    unsafe fn end_commands(&self) -> CommandResult<()>;

    /// Finish the current renderpass.
    ///
    /// # Safety
    ///
    /// Unsafe because the application must call this method only after starting
    /// a renderpass with this command buffer.
    unsafe fn end_renderpass(&self) -> &Self;
}

impl CommandBufferExt for CommandBuffer {
    unsafe fn begin_one_time_submit(&self) -> CommandResult<&Self> {
        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };
        self.vk_dev
            .logical_device
            .begin_command_buffer(self.raw, &begin_info)
            .map_err(CommandBufferExtError::UnableToBeginCommandBuffer)?;
        Ok(self)
    }

    unsafe fn end_commands(&self) -> CommandResult<()> {
        self.vk_dev
            .logical_device
            .end_command_buffer(self.raw)
            .map_err(CommandBufferExtError::UnableToEndCommandBuffer)?;
        Ok(())
    }

    unsafe fn end_renderpass(&self) -> &Self {
        self.vk_dev.logical_device.cmd_end_render_pass(self.raw);
        self
    }
}
