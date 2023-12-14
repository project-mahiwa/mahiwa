# dev

## Design

https://www.figma.com/file/FDTRV5hqFygwRyJWC2XBKU/Mahiwa-GUI%E3%82%A2%E3%83%97%E3%83%AA?type=design&node-id=0%3A1&mode=design&t=tbvAPPO5aoq10Gsa-1

## Install

https://tauri.app/v1/guides/getting-started/prerequisites  
をもとに必要なソフトウェアの用意

```
make init
```

Rust側は実行時に自動解決

## Run

```
make build
```

## Build

```
make build
```

- src-tauri/target/release/bundle/deb/mahiwa_0.1.0_amd64.deb
- src-tauri/target/release/bundle/appimage/mahiwa_0.1.0_amd64.AppImage

https://tauri.app/v1/guides/building/

AppImageの動作確認手順

```
chmod +x mahiwa_0.1.0_amd64.AppImage
./mahiwa_0.1.0_amd64.AppImage
```

## Update

1. src-tauir/cargo.tomlのバージョンを上げる
2. GitHubでtagを切ってバージョンを上げる
3. GitHub Actionsで自動ビルド
