use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    BreakpointEvent, Capabilities, CapabilitiesEvent, ContinuedEvent, ExitedEvent,
    InvalidatedEvent, LoadedSourceEvent, MemoryEvent, ModuleEvent, OutputEvent, ProcessEvent,
    ProgressEndEvent, ProgressStartEvent, ProgressUpdateEvent, StoppedEvent, TerminatedEvent,
    ThreadEvent,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Message {
    Event(Box<Events>),
    Response(Response),
    Request(Request),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Request {
    pub seq: u64,
    pub command: String,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub arguments: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Response {
    pub seq: u64,
    pub request_seq: u64,
    pub success: bool,
    pub command: String,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub body: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "event", content = "body")]
#[serde(rename_all = "camelCase")]
pub enum Events {
    Initialized(Option<Capabilities>),
    Stopped(StoppedEvent),
    Continued(ContinuedEvent),
    Exited(ExitedEvent),
    Terminated(Option<TerminatedEvent>),
    Thread(ThreadEvent),
    Output(OutputEvent),
    Breakpoint(BreakpointEvent),
    Module(ModuleEvent),
    LoadedSource(LoadedSourceEvent),
    Process(ProcessEvent),
    Capabilities(CapabilitiesEvent),
    ProgressStart(ProgressStartEvent),
    ProgressUpdate(ProgressUpdateEvent),
    ProgressEnd(ProgressEndEvent),
    Invalidated(InvalidatedEvent),
    Memory(MemoryEvent),
    #[serde(untagged)]
    Other(HashMap<String, Value>),
}

fn deserialize_empty_object<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value == Value::Object(serde_json::Map::new()) {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}
