use rs_qq::client::{msg::MsgElem, handler::Msg};
use std::collections::HashMap;
use walle_core::{MessageSegment, Event};

pub(crate) trait Parse<T> {
    fn parse(self) -> T;
}

impl Parse<MessageSegment> for MsgElem {
    fn parse(self) -> MessageSegment {
        match self {
            Self::Text { content } => MessageSegment::Text {
                text: content,
                extend: HashMap::new(),
            },
            _ => unimplemented!(), //todo
        }
    }
}

impl Parse<MsgElem> for MessageSegment {
    fn parse(self) -> MsgElem {
        match self {
            Self::Text { text, .. } => MsgElem::Text { content: text },
            _ => unimplemented!(), //todo
        }
    }
}

impl Parse<Event> for Msg {
    fn parse(self) -> Event {
        unimplemented!()
    }
}

impl Parse<Msg> for Event {
    fn parse(self) -> Msg {
        unimplemented!()
    }
}