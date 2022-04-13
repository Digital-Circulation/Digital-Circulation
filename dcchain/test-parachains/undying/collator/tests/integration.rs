// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! Integration test that ensures that we can build and include dcchain
//! blocks of the `Undying` dcchain.

const PUPPET_EXE: &str = env!("CARGO_BIN_EXE_undying_collator_puppet_worker");

// If this test is failing, make sure to run all tests with the `real-overseer` feature being enabled.
#[substrate_test_utils::test]
async fn collating_using_undying_collator() {
	use digital_circulation_primitives::v2::Id as ParaId;
	use sp_keyring::AccountKeyring::*;

	let mut builder = sc_cli::LoggerBuilder::new("");
	builder.with_colors(false);
	builder.init().expect("Set up logger");

	let para_id = ParaId::from(100);

	let alice_config = digital_circulation_test_service::node_config(
		|| {},
		tokio::runtime::Handle::current(),
		Alice,
		Vec::new(),
		true,
	);

	// start alice
	let alice = digital_circulation_test_service::run_validator_node(alice_config, Some(PUPPET_EXE.into()));

	let bob_config = digital_circulation_test_service::node_config(
		|| {},
		tokio::runtime::Handle::current(),
		Bob,
		vec![alice.addr.clone()],
		true,
	);

	// start bob
	let bob = digital_circulation_test_service::run_validator_node(bob_config, Some(PUPPET_EXE.into()));

	let collator = test_dcchain_undying_collator::Collator::new(1_000, 1);

	// register dcchain
	alice
		.register_dcchain(para_id, collator.validation_code().to_vec(), collator.genesis_head())
		.await
		.unwrap();

	// run the collator node
	let mut charlie = digital_circulation_test_service::run_collator_node(
		tokio::runtime::Handle::current(),
		Charlie,
		|| {},
		vec![alice.addr.clone(), bob.addr.clone()],
		collator.collator_key(),
	);

	charlie
		.register_collator(
			collator.collator_key(),
			para_id,
			collator.create_collation_function(charlie.task_manager.spawn_handle()),
		)
		.await;

	// Wait until the dcchain has 4 blocks produced.
	collator.wait_for_blocks(4).await;

	// Wait until the collator received `12` seconded statements for its collations.
	collator.wait_for_seconded_collations(12).await;
}
