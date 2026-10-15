//! A signal stops the CLI while its stdin producer stays open.
#![cfg(unix)]

use std::{process::Stdio, time::Duration};
use tokio::io::{AsyncBufReadExt, BufReader};

const TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::test]
async fn a_stopped_import_exits_with_its_stdin_writer_still_open() {
	let _ = moq_tokio::crypto::install_default();
	let directory = tempfile::tempdir().unwrap();
	let socket = directory.path().join("moq.sock");
	let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_moq"));
	for (name, _) in std::env::vars_os() {
		if name.to_string_lossy().starts_with("MOQ_") {
			command.env_remove(name);
		}
	}
	let mut child = command
		.args([
			"--listen-unix-bind",
			socket.to_str().unwrap(),
			"--auth-public",
			"**",
			"--broadcast",
			"demo",
			"import",
			"ts",
		])
		.env("RUST_LOG", "info")
		.stdin(Stdio::piped())
		.stderr(Stdio::piped())
		.kill_on_drop(true)
		.spawn()
		.unwrap();
	// Child::wait closes its own stdin. Keep this writer outside the child so
	// waiting cannot unblock the read and hide a runtime teardown hang.
	let stdin = child.stdin.take().unwrap();
	let mut stderr = BufReader::new(child.stderr.take().unwrap()).lines();
	tokio::time::timeout(TIMEOUT, async {
		while let Some(line) = stderr.next_line().await.unwrap() {
			if line.contains("listening (unix)") {
				return;
			}
		}
		panic!("CLI stopped before listening");
	})
	.await
	.expect("listener never bound");
	let client = moq_tokio::connect::Config::default()
		.init(Default::default())
		.unwrap()
		.with_reconnect(false);
	let url: url::Url = format!("unix://{}", socket.display()).parse().unwrap();
	let connection = tokio::time::timeout(TIMEOUT, client.connect(url).established())
		.await
		.expect("client never connected")
		.unwrap();
	drop(connection);
	let pid = child.id().unwrap().to_string();
	let signaled = std::process::Command::new("kill")
		.args(["-TERM", &pid])
		.status()
		.unwrap();
	assert!(signaled.success());
	let status = tokio::time::timeout(TIMEOUT, child.wait())
		.await
		.expect("CLI waited for stdin EOF after shutdown")
		.unwrap();
	assert!(status.success(), "CLI exited with {status}");
	drop(stdin);
}
