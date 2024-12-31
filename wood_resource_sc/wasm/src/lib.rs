// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            5
// Async Callback:                       1
// Total number of exported functions:   8

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    wood
    (
        init => init
        upgrade => upgrade
        stakeWinter => stake_winter
        generateResources => generate_resources
        claimResources => claim_resources
        issue => issue_token
        getTokenId => token_id
    )
}

multiversx_sc_wasm_adapter::async_callback! { wood }
