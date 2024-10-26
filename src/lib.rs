//! state-shift is a procedural macro crate designed to:
//! - hide away the complexities come with type-state pattern,
//! - make your code more readable and maintainable,
//! - and still benefit from the power of type-state pattern.
//!
//! Type-state is a design pattern that leverages the type system to enforce valid states and transitions at compile time.
//! This crate provides attribute macros to transform structures and methods into type-safe stateful components,
//! ensuring that methods are only callable in valid states, and enforcing transitions between them.
//!
//! Macros:
//!
//! - `#[require]`: Enforces that a method can only be called when the provided state is active..
//! - `#[switch_to]`: Modifies the return type of methods to switch between states.
//! - `#[states]`: Defines the valid states for a given type and generates corresponding marker structs and trait implementations.
//! - `#[type_state]`: Transforms the struct into type-state compatible form, using state slots and default states.

extern crate proc_macro;

mod helper;
mod require;
mod states;
mod switch_to;
mod type_state;

use helper::{extract_macro_args, is_single_letter};
use require::generate_impl_block_for_method_based_on_require_args;
use states::states_inner;
use switch_to::switch_to_inner;
use type_state::type_state_inner;

use proc_macro::TokenStream;

/// Turns your struct into type-state compatible version.
///
/// Usage: `#[type_state(state_slots = 3, default_state = Initial)]`
///
/// Arguments:
/// - `state_slots` -> if you want to track multiple states at the same time
/// - `default_state` -> the initial state of your struct, you must provide a one of the states defined in the `#[states]` macro
///
/// also protects your struct from getting initialized with random types/states
/// by enforcing sealed-trait bounds on the states.
#[proc_macro_attribute]
pub fn type_state(args: TokenStream, input: TokenStream) -> TokenStream {
    type_state_inner(args, input)
}

/// Denotes which states will be used for the type-state pattern.
///
/// Usage: `#[states(State1, State2, ...)]`
///
/// What it does:
/// - defines the set of states that a type can transition between,
/// - generates marker structs for these states
/// - seals these traits and structs with `Sealer` trait for each state,
/// - provides the necessary `struct_name` information to `#[require]` macro
///
/// Also:
/// - consumes `#[require]` macro and does the things mentioned in `#[require]` macro's inline docs, which are:
/// - generates a specific `impl` block for each method,
/// - adds the required types and generics to the generated `impl` blocks,
/// - adds the hidden `_state` field to the `Self { }` struct, so you don't have to worry about anything regarding type-state-pattern
#[proc_macro_attribute]
pub fn states(attr: TokenStream, item: TokenStream) -> TokenStream {
    states_inner(attr, item)
}

/// Denotes which state is required for this method to be called.
///
/// Usage:
/// - `#[require(State1)]`
/// - or with multiple state slots: `#[require(State1, State2, ...)]`
///
/// This macro is consumed by the `#[states]` macro, and it basically guiding `#[states]` macro to:
/// - generate a specific `impl` block for each method,
/// - add the required types and generics to the generated `impl` blocks,
/// - add the hidden `_state` field to the `Self { }` struct, so you don't have to worry about anything regarding type-state-pattern
///
/// hence, it is empty, because it delegates its job to `#[states]` macro
/// the reason for that delegation is: `#[require]` macro needs the below from the encapsulating `impl` block for the methods
/// - name of the impl block (name of the struct)
/// - generics
/// - lifetimes
#[proc_macro_attribute]
pub fn require(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!()
}

/// Denotes to which state will the object transition into after this method
///
/// Usage:
/// - `#[switch_to(State1)]`
/// - or with multiple state slots: `#[switch_to(State1, State2, ...)]`
///
/// This macro is consumed by the `#[states]` macro, and it basically guiding `#[states]` macro to:
/// - overwrite the return type of the methods generated by the `#[states]` macro
///
/// hence, it is empty, because it delegates its job to `#[states]` macro
/// the reason for that delegation is: `#[switch_to]` macro needs the below from the encapsulating `impl` block for the methods
/// - name of the impl block (name of the struct)
#[proc_macro_attribute]
pub fn switch_to(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!()
}
