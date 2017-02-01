# rshyeong

[난해한 혀엉... 언어](https://gist.github.com/xnuk/d9f883ede568d97caa158255e4b4d069)의
Rust 구현입니다.

## 빌드하기

먼저 [rustup](https://rustup.rs/)으로 Cargo를 설치하고 다음 명령어를 입력해
빌드와 설치를 합니다.

```
cargo install --git https://github.com/VBChunguk/hyeong-rs.git
```

큰 유리수 처리를 원한다면 `big-rational` feature를 켜서 빌드합니다.

```
cargo install --features big-rational --git https://github.com/VBChunguk/hyeong-rs.git
```

## 실행하기

Cargo로 설치하면 홈 디렉토리 아래의 `.cargo/bin`에 바이너리가 들어갑니다.
rustup을 설치했다면 이 디렉토리는 `PATH`에 등록되어 있을 것이므로
그냥 실행하면 됩니다.

```
rshyeong --help
```

---

MIT or Apache-2.0

[![Build Status](https://travis-ci.org/VBChunguk/hyeong-rs.svg?branch=master)](https://travis-ci.org/VBChunguk/hyeong-rs)
