<div align="center">
    <a href="https://mahiwa.usuyuki.net" target="_blank">
    <picture>
<!-- SvelteKitで使ってるロゴを拝借する -->
      <img width="300" src="src/lib/assets/logo/mahiwa-logo-v1.png" alt="Mahiwa logo">
    </picture>
  </a>
<h1>Mahiwa</h1>
<p>
MAHIWA can be written in Any High-level language and run on MicroController, which Integrates the latest functionality with WebAssembly.
</p>

  <p>
    <a href="https://github.com/project-mahiwa/mahiwa/actions/workflows/release.yml">
      <img src="https://github.com/project-mahiwa/mahiwa/actions/workflows/release.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/project-mahiwa/mahiwa/actions/workflows/lint.yml">
      <img src="https://github.com/project-mahiwa/mahiwa/actions/workflows/lint.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/project-mahiwa/mahiwa/actions/workflows/staticAnalysis.yml">
      <img src="https://github.com/project-mahiwa/mahiwa/actions/workflows/staticAnalysis.yml/badge.svg" alt="Build Status">
    </a>
  </p>
</div>

<br />

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
