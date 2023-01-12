use simetry::iracing_basic_solution::broadcast;

fn main() {
    broadcast::Message::PitCommand(broadcast::PitCommand::Fuel(Some(31))).broadcast()
}
