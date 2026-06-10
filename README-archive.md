## ~~網頁應用 (Web Applications)~~

[🤯 主要的現況]：

- 沒有 HMR (Hot Module Replacement) 的功能 ← 開發體驗會不好
- WASM (WebAssembly) 不能直接操作 DOM ← 效能無法到極致

Leptos

- [應用程式 (Application)](./web/Application.md)
- 元件 (Components)
  - 模板語法 (Template Syntax)
  - Reactivity
    - Signals / Computeds / Effects
  - Class and Style
    - Tailwind CSS
    - Scoped CSS
    - Icon
  - Props / Event handler props
  - Children
- 路由器 (Router)
- 狀態管理 (State Management)
- Data Fetching
  - Streams API
- Form Validation
- Web APIs + `leptos::document();` + `web-sys`
- 製作 UI 元件
- `leptos-use`
- 資料視覺化 (Data Visualization)
  - `charming` (ECharts)
- 多語言支援與國際化 (Internationalization)
- 伺服器端算繪 (Server-side Rendering)
  - `axum`
  - 後設資料 (Metadata)
- 雲端服務部署
  - 容器化 + Caddy Server + Docker
  - 部署到 Render
- 雲端平台部署
  - 容器化全代管部署
    - Google Cloud Run
    - Azure Container Apps
    - AWS Fargate
  - Kubernetes + Helm
    - Google Kubernetes Engine
    - Azure Kubernetes Service
    - Amazon Elastic Kubernetes Service
- 本地部署 (輕量化) + Kubernetes K3s
  - Raspberry Pi 5 Model B
- 跨平台應用 `tauri`
  - Pull to Refresh
  - Plugins
    - Store
    - Notifications
    - Deep Linking
    - NFC
    - Barcode Scanner
  - Android + 發布至 Google Play (Closed Testing)
  - Windows + 發布至 Microsoft Store (Package Flights)
  - iOS + 發布至 App Store (Apple TestFlight)
  - macOS + 發布至 App Store (Apple TestFlight)

Leptos Template<br>
Leptos + Axum Template<br>
Leptos + Tauri Template

---

## ~~伺服器應用 (Server Applications)~~

[🤯 主要的現況]：

- Actix Web ← 較適合更好的擴展
- Bun Runtime ← HTTP/WS 效能不會輸太多且開發起來較快

Axum

- [應用程式 (Application)](./server/Application.md)
- [路由 (Routing)](./server/Routing.md)
- [中介軟體 (Middleware)](./server/Middleware.md)
- [身分驗證 (Authentication)](./server/Authentication.md)
- [MongoDB 文件資料庫 (MongoDB Document Database)](./server/MongoDB.md)
- [MinIO S3 物件儲存 (MinIO S3 Object Storage)](./server/MinIO.md)
- [Redis 鍵值對資料庫 (Redis Key-Value Database)](./server/Redis.md)
- [資料流 (Streaming)](./server/Streaming.md)
- [WebSocket 雙向通訊 (WebSocket Full-duplex Interaction)](./server/WebSocket.md)
- 訊息佇列 (Message Queues) + `rdkafka`
- 同構 (Isomorphic) + `leptos`
- 人工智慧代理 (AI Agents) + Rig `rig-core`
  - Gemini (DeepMind) / ChatGPT (OpenAI) / Claude (Anthropic)
    - Insomnia App
    - `leptos` + stream
    - `qdrant-client` + Qdrant
    - Chat Memory
    - Tool Calling Agents
  - Ollama
    - Large language model + `gemma3:4b`
    - Embedding model + `embeddinggemma:300m`
- 提供服務使用 GraphQL (`async-graphql`, `async-graphql-axum`)
- Email + `lettre`
  - Email Builder + `mrml` + MJML
  - Template Engine + `tera`
- 多語言支援與國際化 (Internationalization)
- 全系統搜尋使用 Elasticsearch (`elasticsearch`)
- 內部自有服務通訊使用 gRPC (`tonic`)
- 雲端服務部署
  - 容器化 + Docker
  - 部署到 Render + MongoDB Atlas + IDrive e2
  - 部署到 Redis Cloud + Render's Background Workers
  - 部署到 Qdrant Managed Cloud
  - 申請設定 Gmail/Outlook SMTP Server
- 雲端平台部署
  - 容器化全代管部署
    - Google Cloud Run
    - Azure Container Apps
    - AWS Fargate
  - 人工智慧代理遷移進平台
    - Gemini (DeepMind) on Google Cloud
    - ChatGPT (OpenAI) on Microsoft Azure
    - Claude (Anthropic) on AWS
  - Kubernetes + Helm
    - Google Kubernetes Engine
    - Azure Kubernetes Service
    - Amazon Elastic Kubernetes Service
  - IaC
    - Pulumi + Pulumi Gestalt
- 本地部署 (輕量化) + Kubernetes K3s
  - Raspberry Pi 5 Model B

Axum Template<br>
Axum + Leptos Template<br>
Axum + GraphQL Template<br>
Tonic Template
