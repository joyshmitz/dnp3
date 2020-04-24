use crate::app::header::ResponseHeader;
use crate::app::measurement::*;
use crate::app::parse::bytes::Bytes;
use crate::app::parse::parser::HeaderCollection;
use crate::master::runner::TaskError;
use crate::master::types::CommandTaskError;

pub trait ResponseHandler: Send {
    fn handle(&mut self, source: u16, header: ResponseHeader, headers: HeaderCollection);
}

pub trait RequestCompletionHandler: Send {
    fn on_complete(&mut self, result: Result<(), TaskError>);
}

pub trait CommandTaskHandler: RequestCompletionHandler {
    /// Invoked when the command task succeeds or fails
    fn on_command_complete(&mut self, result: Result<(), CommandTaskError>);
}

impl<T> RequestCompletionHandler for T
where
    T: CommandTaskHandler,
{
    /// If an error occurs, we forward it to `on_command_complete`
    /// successful completion, means that the other completion
    /// handler was already invoked
    fn on_complete(&mut self, result: Result<(), TaskError>) {
        if let Err(err) = result {
            self.on_command_complete(Err(err.into()));
        }
    }
}

pub trait AssociationHandler: ResponseHandler {
    // TODO - add additional methods
}

pub trait ReadTaskHandler: ResponseHandler + RequestCompletionHandler {}

impl<T> ReadTaskHandler for T where T: ResponseHandler + RequestCompletionHandler {}

pub trait MeasurementHandler {
    fn handle_binary(&mut self, x: impl Iterator<Item = (Binary, u16)>);
    fn handle_double_bit_binary(&mut self, x: impl Iterator<Item = (DoubleBitBinary, u16)>);
    fn handle_binary_output_status(&mut self, x: impl Iterator<Item = (BinaryOutputStatus, u16)>);
    fn handle_counter(&mut self, x: impl Iterator<Item = (Counter, u16)>);
    fn handle_frozen_counter(&mut self, x: impl Iterator<Item = (FrozenCounter, u16)>);
    fn handle_analog(&mut self, x: impl Iterator<Item = (Analog, u16)>);
    fn handle_analog_output_status(&mut self, x: impl Iterator<Item = (AnalogOutputStatus, u16)>);
    fn handle_octet_string<'a>(&mut self, x: impl Iterator<Item = (Bytes<'a>, u16)>);
}

#[derive(Copy, Clone)]
pub struct NullHandler;

impl NullHandler {
    pub fn boxed() -> Box<NullHandler> {
        Box::new(Self {})
    }
}

impl ResponseHandler for NullHandler {
    fn handle(&mut self, _source: u16, _header: ResponseHeader, _headers: HeaderCollection) {}
}

impl AssociationHandler for NullHandler {}

impl RequestCompletionHandler for NullHandler {
    fn on_complete(&mut self, _result: Result<(), TaskError>) {}
}
