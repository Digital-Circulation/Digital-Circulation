// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Digital Circulation.

// Digital Circulation is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Digital Circulation is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Digital Circulation.  If not.

//! Digital Circulation types shared between the runtime and the Node-side code.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

// `v2` is currently the latest stable version of the runtime API.
pub mod v2;

// The 'staging' version is special - while other versions are set in stone,
// the staging version is malleable. Once it's released, it gets the next
// version number.
pub mod vstaging;

// `runtime_api` contains the actual API implementation. It contains stable and
// unstable functions.
pub mod runtime_api;
