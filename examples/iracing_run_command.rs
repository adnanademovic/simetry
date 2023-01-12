use simetry::iracing::commands;

fn main() {
    commands::pit::refueling::on_with_liters(31.try_into().unwrap())
}
