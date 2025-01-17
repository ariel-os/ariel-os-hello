#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

use ariel_os::debug::{exit, log::info, ExitCode};

#[cfg(not(test))]
#[ariel_os::thread(autostart)]
fn main() {
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );
    exit(ExitCode::SUCCESS);
}

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    #[test]
    async fn trivial() {
        assert!(false);
    }
}
