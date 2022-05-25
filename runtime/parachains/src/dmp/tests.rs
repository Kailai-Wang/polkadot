// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use crate::mock::{new_test_ext, Configuration, Dmp, MockGenesisConfig, Paras, System};
use hex_literal::hex;
use parity_scale_codec::{Decode, Encode};
use primitives::v2::BlockNumber;

pub(crate) fn run_to_block(to: BlockNumber, new_session: Option<Vec<BlockNumber>>) {
	while System::block_number() < to {
		let b = System::block_number();
		Paras::initializer_finalize(b);
		Dmp::initializer_finalize();
		if new_session.as_ref().map_or(false, |v| v.contains(&(b + 1))) {
			Dmp::initializer_on_new_session(&Default::default(), &Vec::new());
		}
		System::on_finalize(b);

		System::on_initialize(b + 1);
		System::set_block_number(b + 1);

		Paras::initializer_finalize(b + 1);
		Dmp::initializer_initialize(b + 1);
	}
}

fn default_genesis_config() -> MockGenesisConfig {
	MockGenesisConfig {
		configuration: crate::configuration::GenesisConfig {
			config: crate::configuration::HostConfiguration {
				max_downward_message_size: 1024,
				..Default::default()
			},
		},
		..Default::default()
	}
}

fn queue_downward_message(
	para_id: ParaId,
	msg: DownwardMessage,
) -> Result<(), QueueDownwardMessageError> {
	Dmp::queue_downward_message(&Configuration::config(), para_id, msg)
}

fn dmq_contents_bounded(para_id: ParaId, count: usize) -> Vec<InboundDownwardMessage<BlockNumber>> {
	Dmp::dmq_contents_bounded(para_id, count)
}

#[test]
fn clean_dmp_works() {
	let a = ParaId::from(1312);
	let b = ParaId::from(228);
	let c = ParaId::from(123);

	new_test_ext(default_genesis_config()).execute_with(|| {
		// enqueue downward messages to A, B and C.
		queue_downward_message(a, vec![1, 2, 3]).unwrap();
		queue_downward_message(b, vec![4, 5, 6]).unwrap();
		queue_downward_message(c, vec![7, 8, 9]).unwrap();

		let notification = crate::initializer::SessionChangeNotification::default();
		let outgoing_paras = vec![a, b];
		Dmp::initializer_on_new_session(&notification, &outgoing_paras);

		assert!(
			<Dmp as Store>::DownwardMessageQueueFragments::get(QueueFragmentId(a, 0)).is_empty()
		);
		assert!(
			<Dmp as Store>::DownwardMessageQueueFragments::get(QueueFragmentId(b, 0)).is_empty()
		);
		assert!(
			!<Dmp as Store>::DownwardMessageQueueFragments::get(QueueFragmentId(c, 0)).is_empty()
		);
	});
}

#[test]
fn dmq_length_and_head_updated_properly() {
	let a = ParaId::from(1312);
	let b = ParaId::from(228);

	new_test_ext(default_genesis_config()).execute_with(|| {
		assert_eq!(Dmp::dmq_length(a), 0);
		assert_eq!(Dmp::dmq_length(b), 0);

		queue_downward_message(a, vec![1, 2, 3]).unwrap();

		assert_eq!(Dmp::dmq_length(a), 1);
		assert_eq!(Dmp::dmq_length(b), 0);
		assert!(!Dmp::dmq_mqc_head(a).is_zero());
		assert!(Dmp::dmq_mqc_head(b).is_zero());
	});
}

#[test]
fn dmp_mqc_head_fixture() {
	let a = ParaId::from(2000);

	new_test_ext(default_genesis_config()).execute_with(|| {
		run_to_block(2, None);
		assert!(Dmp::dmq_mqc_head(a).is_zero());
		queue_downward_message(a, vec![1, 2, 3]).unwrap();

		run_to_block(3, None);
		queue_downward_message(a, vec![4, 5, 6]).unwrap();

		assert_eq!(
			Dmp::dmq_mqc_head(a),
			hex!["88dc00db8cc9d22aa62b87807705831f164387dfa49f80a8600ed1cbe1704b6b"].into(),
		);
	});
}

#[test]
fn check_processed_downward_messages() {
	let a = ParaId::from(1312);

	new_test_ext(default_genesis_config()).execute_with(|| {
		// processed_downward_messages=0 is allowed when the DMQ is empty.
		assert!(Dmp::check_processed_downward_messages(a, 0).is_ok());

		queue_downward_message(a, vec![1, 2, 3]).unwrap();
		queue_downward_message(a, vec![4, 5, 6]).unwrap();
		queue_downward_message(a, vec![7, 8, 9]).unwrap();

		// 0 doesn't pass if the DMQ has msgs.
		assert!(!Dmp::check_processed_downward_messages(a, 0).is_ok());
		// a candidate can consume up to 3 messages
		assert!(Dmp::check_processed_downward_messages(a, 1).is_ok());
		assert!(Dmp::check_processed_downward_messages(a, 2).is_ok());
		assert!(Dmp::check_processed_downward_messages(a, 3).is_ok());
		// there is no 4 messages in the queue
		assert!(!Dmp::check_processed_downward_messages(a, 4).is_ok());
	});
}

#[test]
fn dmq_pruning() {
	let a = ParaId::from(1312);

	new_test_ext(default_genesis_config()).execute_with(|| {
		assert_eq!(Dmp::dmq_length(a), 0);

		queue_downward_message(a, vec![1, 2, 3]).unwrap();
		queue_downward_message(a, vec![4, 5, 6]).unwrap();
		queue_downward_message(a, vec![7, 8, 9]).unwrap();
		assert_eq!(Dmp::dmq_length(a), 3);

		// pruning 0 elements shouldn't change anything.
		Dmp::prune_dmq(a, 0);
		assert_eq!(Dmp::dmq_length(a), 3);

		Dmp::prune_dmq(a, 2);
		assert_eq!(Dmp::dmq_length(a), 1);
	});
}

#[test]
fn queue_downward_message_critical() {
	let a = ParaId::from(1312);

	let mut genesis = default_genesis_config();
	genesis.configuration.config.max_downward_message_size = 7;

	new_test_ext(genesis).execute_with(|| {
		let smol = [0; 3].to_vec();
		let big = [0; 8].to_vec();

		// still within limits
		assert_eq!(smol.encode().len(), 4);
		assert!(queue_downward_message(a, smol).is_ok());

		// that's too big
		assert_eq!(big.encode().len(), 9);
		assert!(queue_downward_message(a, big).is_err());
	});
}

#[derive(Encode, Decode)]
struct Message(u32);

#[test]
fn dmq_contents_is_bounded() {
	let a = ParaId::from(1312);

	new_test_ext(default_genesis_config()).execute_with(|| {
		let max_queue_size = QUEUE_FRAGMENT_CAPACITY * (u8::MAX as u32);

		// Fill queue.
		for i in 0..max_queue_size {
			assert!(queue_downward_message(a, Message(i).encode()).is_ok());
		}

		let messages =  Dmp::dmq_contents_bounded(a, usize::MAX);
		let max_response_len: usize = (QUEUE_FRAGMENT_CAPACITY * MAX_FRAGMENTS_PER_QUERY).try_into().unwrap();
		assert_eq!(messages.len(), max_response_len);

		let messages = Dmp::dmq_contents(a);
		assert_eq!(messages.len(), max_response_len);
	});
}

#[test]
fn queue_downward_message_fragment_ordering() {
	let a = ParaId::from(1312);

	let mut genesis = default_genesis_config();
	genesis.configuration.config.max_downward_message_size = 100;

	new_test_ext(genesis).execute_with(|| {
		let max_queue_size = QUEUE_FRAGMENT_CAPACITY * (u8::MAX as u32);

		// Fill queue.
		for i in 0..max_queue_size {
			assert!(queue_downward_message(a, Message(i).encode()).is_ok());
		}

		let mut all_messages = Vec::new();

		loop {
			let messages = dmq_contents_bounded(a, 1000);
			if messages.len() == 0 {
				break
			}

			Dmp::prune_dmq(a, messages.len() as u32);
			all_messages.extend(messages);
		}

		// Check message ordering.
		assert!(all_messages.windows(2).all(|e| {
			let left = Message::decode(&mut e[0].msg.as_ref()).unwrap().0;
			let right = Message::decode(&mut e[1].msg.as_ref()).unwrap().0;
			left < right
		}));
	});
}

#[test]
fn queue_downward_message_queue_full() {
	let a = ParaId::from(1337);

	let mut genesis = default_genesis_config();
	genesis.configuration.config.max_downward_message_size = 17;

	new_test_ext(genesis).execute_with(|| {
		let max_queue_size = QUEUE_FRAGMENT_CAPACITY * (u8::MAX as u32);

		for i in 0..max_queue_size {
			assert!(queue_downward_message(a, Message(i).encode()).is_ok());
		}

		assert_eq!(max_queue_size, Dmp::dmq_length(a));

		let msg = Message(0);
		// This one should fail, the queue is now full.
		assert!(queue_downward_message(a, msg.encode()).is_err());

		// Now lets fetch all messages using different chunk sizes (0 to 4 * QUEUE_FRAGMENT_SIZE - 1).
		let mut chunk_size = 1;
		// let queue_len = Dmp::dmq_length(a) as u64;
		let mut sum = 0;
		let mut count = 0;
		loop {
			let page_size = chunk_size % (QUEUE_FRAGMENT_CAPACITY * 4) as usize;
			let mut messages = dmq_contents_bounded(a, page_size);

			if page_size > 0 && messages.len() == 0 {
				break
			}
			count += messages.len() as u32;

			sum += messages
				.iter_mut()
				.map(|e| Message::decode(&mut e.msg.as_ref()).unwrap().0 as u64)
				.sum::<u64>();

			chunk_size += 1;
			Dmp::prune_dmq(a, messages.len() as u32);
		}
		// Check if we collected and pruned all messages.
		let expected_count = max_queue_size;
		assert_eq!(count, expected_count);
		assert_eq!(0, Dmp::dmq_length(a));

		// Check if the messages we retrieved are the expected ones.
		let expected_sum = (max_queue_size as u64 * (max_queue_size as u64 - 1)) / 2;
		assert_eq!(sum, expected_sum);
	});
}

#[test]
fn verify_dmq_mqc_head_is_externally_accessible() {
	use hex_literal::hex;
	use primitives::v2::well_known_keys;

	let a = ParaId::from(2020);

	new_test_ext(default_genesis_config()).execute_with(|| {
		let head = sp_io::storage::get(&well_known_keys::dmq_mqc_head(a));
		assert_eq!(head, None);

		queue_downward_message(a, vec![1, 2, 3]).unwrap();

		let head = sp_io::storage::get(&well_known_keys::dmq_mqc_head(a));
		assert_eq!(
			head,
			Some(hex!["434f8579a2297dfea851bf6be33093c83a78b655a53ae141a7894494c0010589"].to_vec())
		);
	});
}
