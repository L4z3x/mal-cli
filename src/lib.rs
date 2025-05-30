/*
App State
- The actual app state

Mutations
- Actually mutates the app
- Keeps a log for backtracking

Actions                         --|
actions to call from mutations    |
                                  |--> Accessible
                                  |
Getters                         --|
get values from the app state

Shared Application State
Flow of User Input -> Handle Events
Rendering UI
Routing

*/

/// Authorization
pub mod auth;

/// API request functions
pub mod api;

/// UI
pub mod ui;

/// Config
pub mod config;

/// App
pub mod app;

/// Network
pub mod network;

/// Events
pub mod event;

/// Handlers
pub mod handlers;

/// Cli
pub mod cli;

pub mod logging;
