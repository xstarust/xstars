# xstars CLI

紫微斗数排盘命令行工具，终端方格渲染星盘。

## 用法

```bash
cargo run -p xstars-cli -- <DATE> <TIME> <GENDER> [OPTIONS]
```

### 位置参数

| 参数 | 说明 | 示例 |
|------|------|------|
| DATE | 公历日期 YYYY-MM-DD | `2000-8-16` |
| TIME | 时辰或时间 | `2` / `12:00` / `子` |
| GENDER | 性别 | `男` / `女` / `m` / `f` |

### 选项

```
-s, --school <SCHOOL>    流派 (sanhe/zhongzhou/feixing，默认 sanhe)
-l, --lunar              农历输入
-j, --json               JSON 输出
-L, --lang <LANG>        语言 (zh-CN/zh-TW/en/ja/ko/vi)
-t, --target <DATE>      运限目标日期 YYYY-MM-DD
-i, --interactive        交互模式
--layer <LAYER>          运限显示层 (major/minor/yearly/monthly/daily/hourly)
-h, --help               帮助
```

### 示例

```bash
# 基本排盘
cargo run -p xstars-cli -- 2000-8-16 2 女

# JSON 输出 + 运限
cargo run -p xstars-cli -- 2000-8-16 2 女 -j -t 2026-6-21

# 交互模式
cargo run -p xstars-cli -- 2000-8-16 2 女 -t 2026-6-21 -i

# 飞星派 + 英文
cargo run -p xstars-cli -- 2000-8-16 2 女 -s feixing -L en
```

## 交互模式命令

在交互模式（`-i`）下输入：
- `YYYY-MM-DD` — 更新运限（时辰不变）
- `YYYY-MM-DD/HR` — 更新运限（含时辰）
- `q` / `Ctrl+C` — 退出
