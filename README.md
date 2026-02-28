# terminal-runner

Rust + TUI (ターミナルUI) で動く横スクロールのランナーゲームです。

## 必要環境

- Rust (cargo, rustc)

## 実行

```bash
cd terminal-runner
cargo run
```

## 設定

- ゲーム設定は [terminal-runner/config/game.json](terminal-runner/config/game.json) に集約されています。
- 例: `gravity`, `jump_velocity`, `max_jumps`, `spawn_interval_min`, `spawn_interval_max`

## 操作

- `←` / `→` / `A` / `D`: 左右移動
- `Space` / `↑` / `W`: ジャンプ（空中でも1回、計2段ジャンプ）
- `R`: ゲームオーバー後にリスタート
- `Q` / `Esc`: 終了

## ルール

- プレイヤー `@` を操作して障害物 `#` を避けます。
- 地面 `=` に沿って障害物が左へ流れてきます。
- 障害物を通過するとスコアが増えます。