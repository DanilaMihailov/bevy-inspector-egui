.PHONY: build

TARGET_DIR_WEB=target_web
CRATE_NAME=bevy-inspector-egui-demo
FLAGS=--release
OUT_DIR=docs

WASM_FILE=target_web/wasm32-unknown-unknown/release/${CRATE_NAME}.wasm

serve: optimize
	basic-http-server ${OUT_DIR}

build:
	cargo build --target-dir ${TARGET_DIR_WEB} --target wasm32-unknown-unknown ${FLAGS}
	@du -h ${WASM_FILE}

optimize: build
	wasm-bindgen ${WASM_FILE} --out-dir ${OUT_DIR} --target web --out-name wasm --no-typescript --remove-name-section --remove-producers-section
	@du -h ${OUT_DIR}/wasm_bg.wasm
	wasm-opt ${OUT_DIR}/wasm_bg.wasm -o ${OUT_DIR}/wasm_bg.wasm -O2
	@du -h ${OUT_DIR}/wasm_bg.wasm

clean:
	rm ${OUT_DIR}/wasm*