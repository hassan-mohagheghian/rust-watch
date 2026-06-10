# RustWatch Architecture

## Overview

RustWatch is currently a single-node distributed ingestion prototype built in Rust.

It demonstrates a real telemetry ingestion pipeline with async processing and multi-agent load generation.

---

## Current System Architecture

Agent (multi-instance load generator)
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
Event Processing (logging / print)

---

## Core Components

### Agent Layer

- Generates synthetic telemetry events
- Simulates multiple services
- Sends HTTP requests to collector
- Used for load testing

---

### Collector Layer

- Built with Axum
- Receives telemetry events via HTTP
- Validates input data
- Converts DTO → Domain model
- Sends events to internal queue

---

### Queue Layer

- Implemented using `tokio::mpsc`
- Acts as in-memory buffer
- Decouples ingestion from processing

---

### Worker Layer

- Async background task
- Consumes events from queue
- Processes events sequentially
- Currently logs events (placeholder for storage pipeline)

---

## Data Flow

HTTP Request → Validation → Transformation → Queue → Worker → Processing

---

## Current Limitations

- No persistence layer
- No batching
- No backpressure strategy
- Single-node architecture only
- No distributed messaging (Kafka not yet introduced)

---

## Next Architectural Step

- Introduce batching worker
- Add time + size-based buffering
- Optimize CPU usage under load
