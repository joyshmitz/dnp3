use crate::app::enums::FunctionCode;
use crate::app::variations::{Group12Var1, Group41Var1, Group41Var2, Group41Var3, Group41Var4};
use crate::outstation::traits::{BroadcastAction, OperateType};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Control {
    G12V1(Group12Var1, u16),
    G41V1(Group41Var1, u16),
    G41V2(Group41Var2, u16),
    G41V3(Group41Var3, u16),
    G41V4(Group41Var4, u16),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Event {
    BeginControls,
    Select(Control),
    Operate(Control, OperateType),
    EndControls,
    BroadcastReceived(FunctionCode, BroadcastAction),
}

#[derive(Clone)]
pub(crate) struct EventHandle {
    events: Arc<Mutex<VecDeque<Event>>>,
}

impl EventHandle {
    pub(crate) fn new() -> Self {
        EventHandle {
            events: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub(crate) fn push(&self, event: Event) {
        self.events.lock().unwrap().push_back(event);
    }

    pub(crate) fn pop(&self) -> Option<Event> {
        self.events.lock().unwrap().pop_front()
    }
}