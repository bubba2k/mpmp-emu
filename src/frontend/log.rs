use std::time::SystemTime;

pub enum MessageType {
    Error,
    Warning,
    Info,
}

pub struct Message {
    pub timestamp: SystemTime,
    pub message_type: MessageType,
    pub message_string: String,
}

impl Message {
    pub fn new(message_type: MessageType, message_string: String) -> Self {
        Message {
            timestamp: SystemTime::now(),
            message_type,
            message_string,
        }
    }
}

pub struct Log {
    pub messages: Vec<Message>,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            messages: Vec::new(),
        }
    }
}

impl Log {
    pub fn log(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
