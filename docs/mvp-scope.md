# MVP Scope

## Goal

Build a minimal but end-to-end distributed telemetry pipeline.

---

## Phase 1: Agent

- Collect logs
- Collect system metrics
- Send telemetry data to backend

---

## Phase 2: Collector Service

- HTTP/gRPC ingestion API
- Async processing using Tokio
- Basic buffering and batching

---

## Phase 3: Streaming Layer

- Kafka integration
- Partitioned event streams
- Retry and failure handling

---

## Phase 4: Storage Layer

- ClickHouse schema design
- Time-series optimized storage
- Basic retention strategy

---

## Phase 5: Query API

- REST API for logs/metrics/traces
- Time-range filtering
- Basic aggregation support

---

## MVP Definition of Done

The system is complete when:

- Data flows end-to-end
- Metrics/logs can be queried
- Pipeline handles failures gracefully
- System is containerized (Docker)
