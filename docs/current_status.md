# Current System Status

## Stage

RustWatch is currently in the **Ingestion Pipeline Prototype Stage**.

---

## Implemented Features

### Agent System

- Multiple agent instances supported
- Generates synthetic telemetry data
- Current load: ~50 events/sec per agent

---

### Collector API

- HTTP ingestion endpoint (`/ingest`)
- Input validation layer
- DTO handling
- Transformation to domain model

---

### Internal Pipeline

- In-memory queue using `tokio::mpsc`
- Async worker task
- Event processing loop

---

## Load Testing Setup

Current configuration:

- 5 agent instances
- 50 events/sec per agent
- Total load: ~250 events/sec
- Single collector instance

---

## Observed Behavior

- High CPU usage under sustained load
- Worker processes events sequentially
- Queue buffers incoming traffic
- No batching currently implemented

---

## System Constraints

- No persistence layer
- No distributed queue system
- No horizontal scaling
- No batch processing yet

---

## Purpose of Current Stage

This stage validates:

- ingestion pipeline correctness
- async queue behavior
- worker processing model
- system behavior under sustained load
