# 🚀 LLM Gateway（Rust）

> 独立部署的 LLM 控制平面（Control Plane），提供模型调度、统一 streaming 协议、流量控制和可观测能力
> 可被任何 Agent 或应用直接调用，无依赖 Golang 或特定 Agent

---

# 🧠 项目定位

LLM Gateway 旨在：

* 屏蔽不同 LLM 模型差异（OpenAI、Claude、本地模型）
* 提供统一的 Streaming 协议（token / tool_call / final / metadata）
* 内置模型调度、路由、限流和容灾
* 支持可观测指标（metrics / tracing / logging）
* 可独立部署，面向多 Agent、多客户端使用

---

# ✨ 核心亮点

1. **模型调度**：按规则、延迟、成本选择最合适的模型
2. **统一协议**：屏蔽各模型差异，所有客户端消费同一流式事件
3. **高性能 streaming**：支持大并发、低延迟、可中断 token 流
4. **可观测**：Prometheus metrics、OpenTelemetry tracing、structured logs
5. **流量控制**：Rate limiting、Quota、API key、多租户
6. **容灾与 fallback**：模型挂掉可自动切换其他可用模型

---

# 🔄 端到端架构图

```text
Client / Agent (Go / Python / JS)
             │
             ▼
       ┌─────────────┐
       │ LLM Gateway │
       │  (Rust)     │
       ├─────────────┤
       │ Routing     │
       │ Stream Eng. │
       │ Adapter     │
       │ Rate Limit  │
       │ Observability │
       └─────┬───────┘
             │
   ┌─────────┼─────────┐
   ▼         ▼         ▼
OpenAI     Claude    Local LLM
```

---

# 🧩 项目目录及模块说明


```text
llm-gateway/
│
├── Cargo.toml
├── README.md
│
├── cmd/
│   └── gateway/
│       └── main.rs          # 启动入口，启动 HTTP Server / Stream Engine
│
├── config/
│   ├── mod.rs                # pub mod config; pub use loader::*;
│   ├── config.rs             # 全局配置结构体
│   └── loader.rs             # 配置加载逻辑
│
├── api/
│   ├── mod.rs                # pub mod handlers; pub mod middleware;
│   ├── handlers/
│   │   ├── mod.rs            # pub mod chat;
│   │   └── chat.rs           # /v1/chat/completions
│   └── middleware/
│       ├── mod.rs            # pub use auth, logging, rate_limit;
│       ├── auth.rs           # API Key 校验
│       ├── logging.rs        # 请求日志
│       └── rate_limit.rs     # 限流中间件
│
├── core/
│   ├── mod.rs                # pub mod gateway; pub mod router; pub mod stream_engine; pub mod context;
│   ├── gateway.rs            # 核心入口，HTTP request → Adapter → Stream Engine
│   ├── router.rs             # Routing Engine：模型选择 / fallback
│   ├── stream_engine.rs      # Streaming 管理、token 流、流中断
│   └── context.rs            # Request context 管理
│
├── adapters/
│   ├── mod.rs                # pub mod openai; pub mod claude; pub mod local;
│   ├── openai/
│   │   ├── mod.rs            # pub use client, mapper, stream;
│   │   ├── client.rs         # OpenAI API HTTP 调用
│   │   ├── mapper.rs         # 请求/响应 → Unified Stream 转换
│   │   └── stream.rs         # OpenAI SSE 解析
│   ├── claude/
│   │   ├── mod.rs            # 预留
│   └── local/
│       ├── mod.rs            # 预留
│
├── protocol/
│   ├── mod.rs                # pub mod request; pub mod response; pub mod stream;
│   ├── request.rs            # UnifiedRequest 结构
│   ├── response.rs           # 非流式响应
│   └── stream.rs             # Unified Stream Event 定义
│
├── services/
│   ├── mod.rs                # pub mod rate_limiter; pub mod quota; pub mod billing;
│   ├── rate_limiter.rs       # QPS / token 限流
│   ├── quota.rs              # 用户级配额管理
│   └── billing.rs            # 消耗统计 / 计费
│
├── observability/
│   ├── mod.rs                # pub mod metrics; pub mod tracing; pub mod logging;
│   ├── metrics.rs            # Prometheus 指标
│   ├── tracing.rs            # OpenTelemetry
│   └── logging.rs            # 结构化日志
│
├── utils/
│   ├── mod.rs                # pub use error, retry, timeout;
│   ├── error.rs              # 自定义错误类型
│   ├── retry.rs              # retry / backoff 封装
│   └── timeout.rs            # 超时封装
│
├── tests/
│   ├── integration/          # 集成测试
│   └── adapters/             # Adapter 单元测试
```

---

# 🔌 Unified Stream 协议（核心）

所有模型输出统一为事件流（可 SSE / Websocket / HTTP chunked）：

```text
event: token
data: {"content":"Hello"}

event: tool_call
data: {
  "name": "search",
  "arguments": {"query": "Tesla 2025 net profit"}
}

event: final
data: {"content":"最终答案"}

event: meta
data: {"model":"gpt-4o-mini","latency_ms":1200,"tokens":345}

event: error
data: {"message":"rate limit exceeded"}
```

---

# 🔄 端到端流程

```text
Client / Agent
     │
     ▼
HTTP POST /v1/chat/completions
     │
     ▼
Gateway core.gateway()
     │
     ├─ Router: 选择模型 / fallback
     ├─ Adapter: 转换请求 / 解析响应
     └─ Stream Engine: token 流 / 中断 / backpressure
     │
     ▼
LLM Provider (OpenAI / Claude / Local)
     │
     ▼
Adapter → Stream Event → Client
```

---

# 🚀 TODO List（开发路线）

## 🥇 Phase 1（MVP）

* [ ] OpenAI Adapter（streaming 支持）
* [ ] Unified Stream 协议实现（token / tool_call / final / meta / error）
* [ ] HTTP server 启动与请求处理
* [ ] token streaming 转发

## 🥈 Phase 2（核心 Infra）

* [ ] Routing Engine（rule-based）
* [ ] Rate Limiter（QPS / token）
* [ ] Request context 管理
* [ ] Observability: metrics / logging

## 🥉 Phase 3（生产级能力）

* [ ] 多模型适配（Claude / 本地）
* [ ] Fallback 容灾
* [ ] Tracing
* [ ] Quota / billing system

## 🏆 Phase 4（高级能力）

* [ ] 智能 routing（cost / latency aware）
* [ ] Prompt cache
* [ ] 多租户支持
* [ ] Dashboard / monitoring UI

---

# 🎯 设计原则（总结）

1. **解耦**：Gateway 独立于 Agent，可被任何客户端使用
2. **Streaming-first**：所有输出优先流式，支持中断和 backpressure
3. **统一协议**：屏蔽不同模型差异，统一 token / tool_call / final / meta / error
4. **可观测**：metrics / tracing / logging 全链路
5. **可扩展**：Adapter 模式 + Routing Engine 支持新模型或策略无缝接入

---
