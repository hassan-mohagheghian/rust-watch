# RustWatch System Context

## System Purpose

RustWatch is a telemetry ingestion system designed to explore:

- high-throughput ingestion
- async processing pipelines
- distributed systems design principles
- Rust-based backend architecture

---

## Current System Type

RustWatch is currently a:

> Single-node asynchronous ingestion system with simulated distributed load.

---

## System Architecture

Agent Layer (multi-instance load generator)
↓
HTTP Collector (Axum)
↓
Validation Layer
↓
DTO → Domain Transformation
↓
In-memory Queue (tokio mpsc)
↓
Async Worker
↓
Event Processing (logging / placeholder storage)



---

## Key Design Principles

### 1. Decoupled ingestion and processing

HTTP layer does NOT process events directly.
It only validates and enqueues them.

---

### 2. Async-first architecture

All ingestion and processing is non-blocking using Tokio runtime.

---

### 3. Shared domain model

A single `TelemetryEvent` model is used across system boundaries.

---

### 4. In-memory buffering (current phase)

Queue is implemented using `tokio::mpsc`:

- temporary buffer
- not persistent
- single-node only

---

## Load Model

Current system is tested using:

- 5 agent instances
- 50 events/sec per agent
- ~250 events/sec total load

---

## Observed System Behavior

- CPU usage increases significantly under load
- Worker processes events sequentially
- Queue absorbs bursts of traffic
- No batching or backpressure mechanisms yet

---

## System Boundaries

### Included

- HTTP ingestion
- async queue
- worker processing
- synthetic load generation

### Excluded

- distributed messaging systems
- persistent storage
- observability dashboards
- multi-node orchestration

---

## System Evolution Path

### Current Stage

- ingestion pipeline prototype

### Next Stage

- batching + performance optimization

### Future Stages

- Kafka-based streaming
- ClickHouse storage layer
- distributed collectors
- observability platform expansion
