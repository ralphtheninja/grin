// Copyright 2019 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Basic status view definition

use chrono::prelude::Utc;
use cursive::direction::Orientation;
use cursive::traits::Identifiable;
use cursive::view::View;
use cursive::views::{BoxView, LinearLayout, TextView};
use cursive::Cursive;

use crate::tui::constants::VIEW_BASIC_STATUS;
use crate::tui::types::TUIStatusListener;

use crate::chain::SyncStatus;
use crate::servers::ServerStats;

const NANO_TO_MILLIS: f64 = 1.0 / 1_000_000.0;

pub struct TUIStatusView;

impl TUIStatusListener for TUIStatusView {
	/// Create basic status view
	fn create() -> Box<dyn View> {
		let basic_status_view = BoxView::with_full_screen(
			LinearLayout::new(Orientation::Vertical)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Current Status:               "))
						.child(TextView::new("Starting").with_id("basic_current_status")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Connected Peers:              "))
						.child(TextView::new("0").with_id("connected_peers")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Disk Usage (GB):              "))
						.child(TextView::new("0").with_id("disk_usage")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal).child(TextView::new(
						"--------------------------------------------------------",
					)),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Header Tip Hash:              "))
						.child(TextView::new("  ").with_id("basic_header_tip_hash")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Header Chain Height:          "))
						.child(TextView::new("  ").with_id("basic_header_chain_height")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Header Cumulative Difficulty: "))
						.child(TextView::new("  ").with_id("basic_header_total_difficulty")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Header Tip Timestamp:         "))
						.child(TextView::new("  ").with_id("basic_header_timestamp")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal).child(TextView::new(
						"--------------------------------------------------------",
					)),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Chain Tip Hash:               "))
						.child(TextView::new("  ").with_id("tip_hash")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Chain Height:                 "))
						.child(TextView::new("  ").with_id("chain_height")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Chain Cumulative Difficulty:  "))
						.child(TextView::new("  ").with_id("basic_total_difficulty")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Chain Tip Timestamp:          "))
						.child(TextView::new("  ").with_id("chain_timestamp")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal).child(TextView::new(
						"--------------------------------------------------------",
					)),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Transaction Pool Size:        "))
						.child(TextView::new("0").with_id("tx_pool_size"))
						.child(TextView::new(" ("))
						.child(TextView::new("0").with_id("tx_pool_kernels"))
						.child(TextView::new(")")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("Stem Pool Size:               "))
						.child(TextView::new("0").with_id("stem_pool_size"))
						.child(TextView::new(" ("))
						.child(TextView::new("0").with_id("stem_pool_kernels"))
						.child(TextView::new(")")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal).child(TextView::new(
						"--------------------------------------------------------",
					)),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("  ").with_id("basic_mining_config_status")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("  ").with_id("basic_mining_status")),
				)
				.child(
					LinearLayout::new(Orientation::Horizontal)
						.child(TextView::new("  ").with_id("basic_network_info")),
				), //.child(logo_view)
		);
		Box::new(basic_status_view.with_id(VIEW_BASIC_STATUS))
	}

	/// update
	fn update(c: &mut Cursive, stats: &ServerStats) {
		//find and update here as needed
		let basic_status = {
			match stats.sync_status {
				SyncStatus::Initial => "Initializing".to_string(),
				SyncStatus::NoSync => "Running".to_string(),
				SyncStatus::AwaitingPeers(_) => "Waiting for peers".to_string(),
				SyncStatus::HeaderSync {
					current_height,
					highest_height,
				} => {
					let percent = if highest_height == 0 {
						0
					} else {
						current_height * 100 / highest_height
					};
					format!("Downloading headers: {}%, step 1/4", percent)
				}
				SyncStatus::TxHashsetDownload {
					start_time,
					prev_update_time,
					update_time: _,
					prev_downloaded_size,
					downloaded_size,
					total_size,
				} => {
					if total_size > 0 {
						let percent = if total_size > 0 {
							downloaded_size * 100 / total_size
						} else {
							0
						};
						let start = prev_update_time.timestamp_nanos();
						let fin = Utc::now().timestamp_nanos();
						let dur_ms = (fin - start) as f64 * NANO_TO_MILLIS;

						format!("Downloading {}(MB) chain state for state sync: {}% at {:.1?}(kB/s), step 2/4",
						total_size / 1_000_000,
						percent,
						if dur_ms > 1.0f64 { (downloaded_size - prev_downloaded_size) as f64 / dur_ms as f64 } else { 0f64 },
						)
					} else {
						let start = start_time.timestamp_millis();
						let fin = Utc::now().timestamp_millis();
						let dur_secs = (fin - start) / 1000;

						format!("Downloading chain state for state sync. Waiting remote peer to start: {}s, step 2/4",
										dur_secs,
										)
					}
				}
				SyncStatus::TxHashsetSetup => {
					"Preparing chain state for validation, step 3/4".to_string()
				}
				SyncStatus::TxHashsetValidation {
					kernels,
					kernel_total,
					rproofs,
					rproof_total,
				} => {
					// 10% of overall progress is attributed to kernel validation
					// 90% to range proofs (which are much longer)
					let mut percent = if kernel_total > 0 {
						kernels * 10 / kernel_total
					} else {
						0
					};
					percent += if rproof_total > 0 {
						rproofs * 90 / rproof_total
					} else {
						0
					};
					format!("Validating chain state: {}%, step 3/4", percent)
				}
				SyncStatus::TxHashsetSave => {
					"Finalizing chain state for state sync, step 3/4".to_string()
				}
				SyncStatus::TxHashsetDone => {
					"Finalized chain state for state sync, step 3/4".to_string()
				}
				SyncStatus::BodySync {
					current_height,
					highest_height,
				} => {
					let percent = if highest_height == 0 {
						0
					} else {
						current_height * 100 / highest_height
					};
					format!("Downloading blocks: {}%, step 4/4", percent)
				}
				SyncStatus::Shutdown => "Shutting down, closing connections".to_string(),
			}
		};
		/*let basic_mining_config_status = {
			if stats.mining_stats.is_enabled {
				"Configured as mining node"
			} else {
				"Configured as validating node only (not mining)"
			}
		};
		let (basic_mining_status, basic_network_info) = {
			if stats.mining_stats.is_enabled {
				if stats.is_syncing {
					(
						"Mining Status: Paused while syncing".to_string(),
						" ".to_string(),
					)
				} else if stats.mining_stats.combined_gps == 0.0 {
					(
						"Mining Status: Starting miner and awaiting first solution...".to_string(),
						" ".to_string(),
					)
				} else {
					(
						format!(
							"Mining Status: Mining at height {} at {:.*} GPS",
							stats.mining_stats.block_height, 4, stats.mining_stats.combined_gps
						),
						format!(
							"Cuckoo {} - Network Difficulty {}",
							stats.mining_stats.edge_bits,
							stats.mining_stats.network_difficulty.to_string()
						),
					)
				}
			} else {
				(" ".to_string(), " ".to_string())
			}
		};*/
		c.call_on_id("basic_current_status", |t: &mut TextView| {
			t.set_content(basic_status);
		});
		c.call_on_id("connected_peers", |t: &mut TextView| {
			t.set_content(stats.peer_count.to_string());
		});
		c.call_on_id("disk_usage", |t: &mut TextView| {
			t.set_content(stats.disk_usage_gb.clone());
		});
		c.call_on_id("tip_hash", |t: &mut TextView| {
			t.set_content(stats.chain_stats.last_block_h.to_string() + "...");
		});
		c.call_on_id("chain_height", |t: &mut TextView| {
			t.set_content(stats.chain_stats.height.to_string());
		});
		c.call_on_id("basic_total_difficulty", |t: &mut TextView| {
			t.set_content(stats.chain_stats.total_difficulty.to_string());
		});
		c.call_on_id("chain_timestamp", |t: &mut TextView| {
			t.set_content(stats.chain_stats.latest_timestamp.to_string());
		});
		if let Some(header_stats) = &stats.header_stats {
			c.call_on_id("basic_header_tip_hash", |t: &mut TextView| {
				t.set_content(header_stats.last_block_h.to_string() + "...");
			});
			c.call_on_id("basic_header_chain_height", |t: &mut TextView| {
				t.set_content(header_stats.height.to_string());
			});
			c.call_on_id("basic_header_total_difficulty", |t: &mut TextView| {
				t.set_content(header_stats.total_difficulty.to_string());
			});
			c.call_on_id("basic_header_timestamp", |t: &mut TextView| {
				t.set_content(header_stats.latest_timestamp.to_string());
			});
		}
		c.call_on_id("tx_pool_size", |t: &mut TextView| {
			t.set_content(stats.tx_stats.tx_pool_size.to_string());
		});
		c.call_on_id("stem_pool_size", |t: &mut TextView| {
			t.set_content(stats.tx_stats.stem_pool_size.to_string());
		});
		c.call_on_id("tx_pool_kernels", |t: &mut TextView| {
			t.set_content(stats.tx_stats.tx_pool_kernels.to_string());
		});
		c.call_on_id("stem_pool_kernels", |t: &mut TextView| {
			t.set_content(stats.tx_stats.stem_pool_kernels.to_string());
		});
		/*c.call_on_id("basic_mining_config_status", |t: &mut TextView| {
			t.set_content(basic_mining_config_status);
		});
		c.call_on_id("basic_mining_status", |t: &mut TextView| {
			t.set_content(basic_mining_status);
		});
		c.call_on_id("basic_network_info", |t: &mut TextView| {
			t.set_content(basic_network_info);
		});*/
	}
}
