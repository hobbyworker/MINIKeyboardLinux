use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use domain::ports::{KeyboardDeviceRepository, LayerActions, LocalFuture};
use domain::{
    CommandSet, DeviceLogEvent, KeyboardConfiguration, KeyboardDeviceError, KeyboardIdentity, KeyboardLayout,
    LayerLighting, LogContent, RecordFormat,
};

use crate::channel::{ChannelProvider, ReportChannel, sleep};
use crate::log::DeviceLogStore;
use crate::protocols::plan::WritePlan;
use crate::protocols::{k884x, numbers, records};

/// Waits the given time; tests wait none.
pub type Pause = Rc<dyn Fn(Duration) -> LocalFuture<'static, ()>>;

/// Talks to whichever keyboard the channel provider has open, in the commands it takes.
pub struct KeyboardDevice {
    channels: Rc<dyn ChannelProvider>,
    log: DeviceLogStore,
    identify_timeout: Duration,
    reply_timeout: Duration,
    pause: Pause,
}

impl KeyboardDevice {
    /// The cross-platform app waits 10 ms for the identification answer; here a slow keyboard gets 600 ms,
    /// and every other answer a second.
    pub fn new(channels: Rc<dyn ChannelProvider>, log: DeviceLogStore) -> Self {
        Self::with_timing(channels, log, Duration::from_millis(600), Duration::from_secs(1), Rc::new(sleep))
    }

    pub fn with_timing(
        channels: Rc<dyn ChannelProvider>,
        log: DeviceLogStore,
        identify_timeout: Duration,
        reply_timeout: Duration,
        pause: Pause,
    ) -> Self {
        Self { channels, log, identify_timeout, reply_timeout, pause }
    }

    fn channel(&self) -> Result<Rc<dyn ReportChannel>, KeyboardDeviceError> {
        self.channels.open_channel().ok_or(KeyboardDeviceError::NotConnected)
    }

    async fn send(&self, payload: &[u8], channel: &dyn ReportChannel) -> Result<(), KeyboardDeviceError> {
        let report = channel.framing().frame(payload);
        self.log.append(LogContent::Output(report.clone()));
        let result = channel.write(report).await;
        if let Err(KeyboardDeviceError::WriteFailed { code }) = result {
            self.log.append(LogContent::Event(DeviceLogEvent::WriteFailed { code }));
        }
        result
    }

    async fn reply(&self, channel: &dyn ReportChannel, timeout: Duration) -> Option<Vec<u8>> {
        let reply = channel.next_reply(timeout).await?;
        self.log.append(LogContent::Input(reply.clone()));
        Some(reply)
    }

    async fn ask_identity(&self) -> Result<Option<KeyboardIdentity>, KeyboardDeviceError> {
        let channel = self.channel()?;
        channel.discard_pending_replies();
        self.send(&k884x::IDENTIFY, channel.as_ref()).await?;
        let Some(reply) = self.reply(channel.as_ref(), self.identify_timeout).await else { return Ok(None) };
        k884x::identity(&reply).map(Some).ok_or(KeyboardDeviceError::InvalidReply)
    }

    async fn read_actions(
        &self,
        layer: usize,
        format: RecordFormat,
        layout: &KeyboardLayout,
    ) -> Result<LayerActions, KeyboardDeviceError> {
        let request = k884x::read_records(layer, format).ok_or(KeyboardDeviceError::Unsupported)?;
        let channel = self.channel()?;
        channel.discard_pending_replies();
        self.send(&request, channel.as_ref()).await?;
        let mut actions = LayerActions::new();
        for record_id in 1..=k884x::read_reply_count(format) {
            let reply = self.reply(channel.as_ref(), self.reply_timeout).await.ok_or(KeyboardDeviceError::NoReply)?;
            let record_id = u8::try_from(record_id).expect("at most 25 records");
            if let Some(control) = numbers::control_in_record(record_id, layout)
                && let Some(action) = records::decode(k884x::record(&reply), format)
            {
                actions.insert(control, action);
            }
        }
        Ok(actions)
    }

    async fn read_lighting(&self, layer: usize) -> Result<LayerLighting, KeyboardDeviceError> {
        let channel = self.channel()?;
        channel.discard_pending_replies();
        self.send(&k884x::read_lighting(layer), channel.as_ref()).await?;
        let reply = self.reply(channel.as_ref(), self.reply_timeout).await.ok_or(KeyboardDeviceError::NoReply)?;
        k884x::lighting_reply(&reply).ok_or(KeyboardDeviceError::InvalidReply)
    }

    async fn write(
        &self,
        configuration: &KeyboardConfiguration,
        commands: CommandSet,
        layout: &KeyboardLayout,
    ) -> Result<(), KeyboardDeviceError> {
        let plan = WritePlan::new(configuration, commands, layout)?;
        let channel = self.channel()?;
        for step in plan.steps {
            self.send(&step.payload, channel.as_ref()).await?;
            if !step.pause_after.is_zero() {
                (self.pause)(step.pause_after).await;
            }
        }
        Ok(())
    }
}

impl KeyboardDeviceRepository for KeyboardDevice {
    fn ask_identity(&self) -> LocalFuture<'_, Result<Option<KeyboardIdentity>, KeyboardDeviceError>> {
        Box::pin(self.ask_identity())
    }

    fn read_actions<'a>(
        &'a self,
        layer: usize,
        format: RecordFormat,
        layout: &'a KeyboardLayout,
    ) -> LocalFuture<'a, Result<LayerActions, KeyboardDeviceError>> {
        Box::pin(self.read_actions(layer, format, layout))
    }

    fn read_lighting(&self, layer: usize) -> LocalFuture<'_, Result<LayerLighting, KeyboardDeviceError>> {
        Box::pin(self.read_lighting(layer))
    }

    fn write<'a>(
        &'a self,
        configuration: &'a KeyboardConfiguration,
        commands: CommandSet,
        layout: &'a KeyboardLayout,
    ) -> LocalFuture<'a, Result<(), KeyboardDeviceError>> {
        Box::pin(self.write(configuration, commands, layout))
    }
}

impl fmt::Debug for KeyboardDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyboardDevice")
            .field("identify_timeout", &self.identify_timeout)
            .field("reply_timeout", &self.reply_timeout)
            .finish_non_exhaustive()
    }
}
