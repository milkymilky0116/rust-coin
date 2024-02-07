# Rust Coin

This Project is re-write project of [Nomadcoder's Nomad coin in golang](https://nomadcoders.co/nomadcoin/lobby) with rust language.

## Achieved

- Section 4: Blockchain

  - [x] #4.1 Our First Block
  - [x] #4.2 Our First Blockchain
  - [x] #4.3 Singleton Pattern
  - [x] #4.4 - 4.5 Refactoring

- Section 5: Explorer

  - in this section, i'm using [Axum](https://github.com/tokio-rs/axum) for web framework, [Askama](https://github.com/djc/askama/tree/main) for template engine.

  - Since _Askama_ behaves differently compared to _Golang Template engine_, I couldn't achieve 100% replicate from original source code. (like head, footer partial)

  - [x] #5.1 Rendering Templates
  - [x] #5.2 Rendering Blocks
  - [x] #5.3 Using Partials
  - [x] #5.4 Adding A Block
  - [x] #5.5 Refactoring

- Section 6: REST API

  - [x] #6.0 Setup
  - [x] #6.4 NewServeMux
  - [x] #6.6 Atoi
  - [x] #6.7 Error Handling

  - Thanks to axum's awesomeness, i don't have to implemenent header middleware to add 'application/json'.

- Section 7: CLI

  - [x] #7.0 Introduction
  - [x] #7.1 Parsing Commands
  - [x] #7.2 FlagSet
  - [x] #7.3 Flag

  - In this section, i'm using [clap](https://github.com/clap-rs/clap) for parsing command-line.
