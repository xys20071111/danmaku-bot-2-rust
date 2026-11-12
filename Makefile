all:
	cargo build --release
	bash -c "mkdir -p dist/plugins/{danmaku-collector,live-start-notify}"
	cp ./target/release/config-converter ./dist
	cp ./target/release/danmaku-bot-core ./dist
	cp ./target/release/danmaku-collector-backend-psql ./dist/plugins/danmaku-collector/
	cp ./target/release/live-start-notify-backend-ntfy ./dist/plugins/live-start-notify/
	echo '{"author": "xys20071111","name": "danmaku-collector","license": "MIT","main_entry": "danmaku-collector-backend-psql"}' > ./dist/plugins/danmaku-collector/metadata.json
	echo '{"author": "xys20071111","name": "live-start-notify","license": "MIT","main_entry": "live-start-notify-backend-ntfy"}' > ./dist/plugins/live-start-notify/metadata.json

rpi:
	cargo build --release --target aarch64-unknown-linux-gnu
	bash -c "mkdir -p dist_aarch64/plugins/{danmaku-collector,live-start-notify}"
	cp ./target/release/config-converter ./dist_aarch64
	cp ./target/release/danmaku-bot-core ./dist_aarch64
	cp ./target/release/danmaku-collector-backend-psql ./dist_aarch64/plugins/danmaku-collector/
	cp ./target/release/live-start-notify-backend-ntfy ./dist_aarch64/plugins/live-start-notify/
	echo '{"author": "xys20071111","name": "danmaku-collector","license": "MIT","main_entry": "danmaku-collector-backend-psql"}' > ./dist_aarch64/plugins/danmaku-collector/metadata.json
	echo '{"author": "xys20071111","name": "live-start-notify","license": "MIT","main_entry": "live-start-notify-backend-ntfy"}' > ./dist_aarch64/plugins/live-start-notify/metadata.json
