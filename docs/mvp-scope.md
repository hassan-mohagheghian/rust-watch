# RustWatch MVP Scope

## Current Stage Definition

RustWatch is currently in the **Ingestion Pipeline MVP stage**.

This stage focuses on validating core ingestion architecture before introducing distributed systems components like Kafka or ClickHouse.

---

## MVP Goals (Current)

The current MVP is defined as:

### 1. Telemetry Ingestion

- HTTP ingestion endpoint (`/ingest`)
- Accept structured telemetry events
- Validate incoming payloads

---

### 2. Data Model

- DTO layer for external input
- Domain model (`TelemetryEvent`)
- Transformation layer between DTO → Domain

---

### 3. Internal Processing Pipeline

- In-memory queue using `tokio::mpsc`
- Async worker processing events
- Decoupling ingestion from processing

---

### 4. Load Generation

- Multi-agent simulation system
- Each agent generates continuous telemetry
- Current load: ~250 events/sec total

---

## What is NOT in MVP (Explicit Non-Goals)

The following are intentionally excluded at this stage:

- Kafka or any distributed message broker
- ClickHouse or persistent storage
- Dashboards or UI layer
- Multi-node deployment
- AI-based analytics
- OpenTelemetry integration

---

## MVP Success Criteria

The MVP is considered successful when:

- System can sustain ~250 events/sec load
- Queue decouples ingestion from processing
- Worker processes events asynchronously
- System remains stable under sustained load
- Architecture is ready for batching upgrade

---

## Next Evolution After MVP

Once stable, the system will evolve into:

- Batching worker (high priority)
- Backpressure handling
- Kafka integration (future phase)
- Storage layer (ClickHouse)
