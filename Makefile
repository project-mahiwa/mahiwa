init:
	pnpm install
dev:
	pnpm tauri dev
build:
	pnpm tauri build
update:
	pnpm upgrade
	(cd src-tauri && cargo update)
