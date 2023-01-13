//! Commands to control chat.

use super::BROADCAST_CHAT_COMMAND;

const CHAT_COMMAND_MACRO: u16 = 0;
const CHAT_COMMAND_BEGIN_CHAT: u16 = 1;
const CHAT_COMMAND_REPLY: u16 = 2;
const CHAT_COMMAND_CANCEL: u16 = 3;

pub fn run_macro(id: u8) {
    BROADCAST_CHAT_COMMAND.run((CHAT_COMMAND_MACRO, id as u16))
}

pub fn open() {
    BROADCAST_CHAT_COMMAND.run((CHAT_COMMAND_BEGIN_CHAT, 0))
}

pub fn close() {
    BROADCAST_CHAT_COMMAND.run((CHAT_COMMAND_CANCEL, 0))
}

pub fn reply() {
    BROADCAST_CHAT_COMMAND.run((CHAT_COMMAND_REPLY, 0))
}
