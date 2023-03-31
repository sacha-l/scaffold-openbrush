Use this folder to create new ink! contracts and modify them for OpenBrush compatibility.

For each new contract you want to create:

1. Run `cargo contract new <contract-name>` (replacing `<contract-name>` with whatever it is you want to name your contract)
1. `cd <contract-name>`
1. Run `cargo add openbrush --git https://github.com/727-Ventures/openbrush-contracts --no-default-features`
1. Add `"openbrush/std",` to the `std` features in the Cargo.toml
1. Enable the default implementation of OpenBrush traits by using [`min_specialization`](https://doc.rust-lang.org/beta/unstable-book/language-features/min-specialization.html). Put this line at the top of the contract's `lib.rs` file: `#![feature(min_specialization)]`
1. Replace the `#[ink::contract]` with `#[openbrush::contract]`

And you're ready to start hacking with ink! and OpenBrush! 🚀