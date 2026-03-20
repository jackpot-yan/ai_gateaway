# 🚀 LLM Gateway（Rust）

> A production-grade LLM control plane for routing, streaming, and observability

---

# 🧠 项目定位（升级版）

LLM Gateway 是一个**独立部署的 LLM 基础设施服务**，用于：

* 统一接入 OpenAI API / Claude / 本地模型
* 提供**模型调度（routing）能力**
* 提供**流量控制（rate limit / quota）**
* 提供**统一 streaming 协议**
* 提供**全链路可观测**

---

## 🎯 核心目标

```text
让任何 Agent / App / SDK：
无需关心模型差异 / 成本 / 稳定性
只需调用一个统一接口
```

---

# ✨ 核心亮点（真正区别“普通代理”）

---

## 🧠 1. 模型调度引擎（核心价值）

不仅仅是调用模型，而是：

```text
选择“用哪个模型”
```

---

### 示例能力：

```text
简单问题 → 小模型（低成本）
复杂推理 → 大模型（高质量）
```

---

### 支持策略：

* rule-based routing
* latency-aware routing
* cost-aware routing
* fallback routing

---

---

## 🔥 2. Unified Streaming 协议（跨模型标准化）

统一输出：

```text
token / tool_call / final / error
```

👉 屏蔽：

* OpenAI delta 格式
* Claude streaming 格式
* 本地模型差异

---

---

## ⚡ 3. 高性能流式网关（Rust 优势）

* 高并发 streaming
* 低延迟 token forwarding
* 支持 thousands of concurrent sessions

---

---

## 🔌 4. 多模型接入（Adapter 架构）

支持扩展：

```text
OpenAI / Claude / Local LLM / vLLM
```

---

---

## 📊 5. 可观测性（Infra 必备）

提供：

* latency（模型响应时间）
* token usage（成本）
* error rate
* model-level metrics

---

---

## 🧱 6. 流量控制（生产关键）

支持：

* rate limiting（QPS）
* quota（token级别）
* API key 管理
* 多租户隔离

---

---

## 🔄 7. 容灾与降级（Reliability）

```text
OpenAI 挂了 → 自动 fallback Claude
```

---

---

## 🧩 8. 与 Agent 解耦（关键设计）

```text
Agent ≠ Gateway
```

👉 Gateway 不关心：

* Agent loop
* tool 执行
* 上下文管理

---

👉 Gateway 只负责：

```text
“如何调用模型”
```

---

# 🏗️ 架构设计（真正完整）

---

## 🔥 系统架构

```text
                ┌──────────────────────┐
                │   Client / Agent     │
                │ (Go / Python / JS)   │
                └──────────┬───────────┘
                           │
                           ▼
                ┌──────────────────────┐
                │    LLM Gateway       │
                │   (Rust Service)     │
                │                      │
                │  ┌────────────────┐  │
                │  │ Routing Engine │  │
                │  └────────────────┘  │
                │  ┌────────────────┐  │
                │  │ Rate Limiter   │  │
                │  └────────────────┘  │
                │  ┌────────────────┐  │
                │  │ Adapter Layer  │  │
                │  └────────────────┘  │
                │  ┌────────────────┐  │
                │  │ Stream Engine  │  │
                │  └────────────────┘  │
                │  ┌────────────────┐  │
                │  │ Observability  │  │
                │  └────────────────┘  │
                └──────────┬───────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
   OpenAI API        Claude API         Local Model
```

---

# 🔥 Unified Stream 协议（核心）

---

## 🎯 设计目标

跨模型统一输出，供任何客户端消费：

```text
event + data（SSE / HTTP streaming）
```

---

## 📡 事件定义

---

### 1️⃣ Token

```text
event: token
data: {"content": "Hello"}
```

---

### 2️⃣ Tool Call（透传）

```text
event: tool_call
data: {
  "name": "search",
  "arguments": {...}
}
```

---

---

### 3️⃣ Final

```text
event: final
data: {
  "content": "最终答案"
}
```

---

---

### 4️⃣ Error

```text
event: error
data: {
  "message": "rate limit exceeded"
}
```

---

---

### 5️⃣ Metadata（新增，Infra级）

```text
event: meta
data: {
  "model": "gpt-4o-mini",
  "latency_ms": 1200,
  "tokens": 345
}
```

---

# 🔄 Streaming Pipeline（关键机制）

---

```text
LLM Provider
   ↓
Adapter（解析原始格式）
   ↓
Unified Stream Event
   ↓
Stream Engine（调度/控制）
   ↓
Client（SSE / HTTP）
```

---

# 🧠 核心模块设计

---

## 🧩 1. Routing Engine（核心）

决定：

```text
用哪个模型
```

---

输入：

* 请求内容
* 用户配置
* 成本策略

---

输出：

```text
model = "gpt-4" / "gpt-3.5" / "local"
```

---

---

## 🔌 2. Adapter Layer

职责：

```text
模型协议 → 统一协议
```

---

---

## ⚡ 3. Stream Engine

职责：

* 管理 streaming 生命周期
* 控制 backpressure
* 处理中断

---

---

## 🚦 4. Rate Limiter

支持：

* QPS
* token/s
* 用户级限制

---

---

## 📊 5. Observability

输出：

* metrics（Prometheus）
* tracing（OpenTelemetry）
* logs

---

# 🗂️ 项目目录（完整工程级）

---

```text
llm-gateway/
│
├── cmd/
│   └── gateway/
│       └── main.rs
│
├── api/
│   ├── handlers/
│   │   └── chat.rs
│   ├── routes.rs
│   └── middleware/
│       ├── auth.rs
│       ├── logging.rs
│       └── rate_limit.rs
│
├── core/
│   ├── gateway.rs
│   ├── router.rs           # 🔥 Routing Engine
│   ├── stream_engine.rs    # 🔥 Streaming 核心
│   └── context.rs
│
├── adapters/
│   ├── mod.rs
│   ├── openai/
│   ├── claude/
│   └── local/
│
├── protocol/
│   ├── request.rs
│   ├── response.rs
│   └── stream.rs           # 🔥 Unified Stream
│
├── services/
│   ├── rate_limiter.rs
│   ├── quota.rs
│   └── billing.rs
│
├── observability/
│   ├── metrics.rs
│   ├── tracing.rs
│   └── logging.rs
│
├── config/
│   ├── config.rs
│   └── loader.rs
│
├── tests/
│
└── Cargo.toml
```

---

# 🔌 API 设计（对外）

---

## Endpoint

```http
POST /v1/chat/completions
```

---

## 请求

```json
{
  "model": "auto",
  "messages": [...],
  "tools": [...],
  "stream": true
}
```

---

👉 `model=auto` → 启用 routing

---

## 响应（stream）

```text
event: token
event: tool_call
event: final
event: meta
```

---

# 🚀 TODO List（Infra级）

---

## 🥇 Phase 1（打通）

* [ ] OpenAI Adapter（streaming）
* [ ] Unified Stream 实现
* [ ] 基础 HTTP 服务
* [ ] token 转发

---

## 🥈 Phase 2（核心能力）

* [ ] Routing Engine（rule-based）
* [ ] Rate Limiting
* [ ] Metrics

---

## 🥉 Phase 3（生产级）

* [ ] 多模型支持
* [ ] fallback
* [ ] tracing
* [ ] quota system

---

## 🏆 Phase 4（高级）

* [ ] 智能 routing（cost/latency）
* [ ] prompt cache
* [ ] 多租户系统
* [ ] dashboard

---
