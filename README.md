# User agent

获取 user agent，通常用于使用 reqwest 等进行网络请求的场景。

## 领域

- 桌面端：提供了 Linux、Windows 等常见 user agent
- 移动端：提供了 iPhone、安卓 等常见 user agent
- 所有：包含了桌面端和移动端

## features

- `random`（默认）：包含随机获取单条 user agent 的功能。

## 使用

首先加入依赖：

```toml
[dependencies]
user-agent = { git = "https://github.com/axumrs/user-agent" }
```

如果不需要随机获取单条 user agent 功能：

```toml
[dependencies]
user-agent = { git = "https://github.com/axumrs/user-agent", default-features = false }
```

## API 列表

| API                     | 说明                       |
| ----------------------- | -------------------------- |
| `desktop_user_agents()` | 获取桌面端 user agent 列表 |
| `mobile_user_agents()`  | 获取移动端 user agent 列表 |
| `all_user_agents()`     | 获取所有 user agent 列表   |

以下 API 需要启用 `random` feature（默认已启用）

| API                    | 说明                                          |
| ---------------------- | --------------------------------------------- |
| `desktop_user_agent()` | 随机获取一条桌面端 user agent                 |
| `mobile_user_agent()`  | 随机获取一条移动端 user agent                 |
| `user_agent()`         | 随机获取一条 user agent，不区分桌面端和移动端 |
