# ADR 0002: Service Communication Model

## Status

Accepted

---

## Context

RustWatch services must communicate efficiently under high load with:

- Low latency
- High throughput
- Backpressure handling
- Fault tolerance

---

## Decision

We adopt a **hybrid communication model**:

- HTTP/gRPC for synchronous ingestion (Agent → Collector)
- Kafka for asynchronous internal streaming

---

## Alternatives Considered

### 1. Pure HTTP

- Simple
- No durability guarantees
- Not suitable for high-scale ingestion

### 2. Pure message queue (Kafka only)

- High durability
- Adds complexity for edge agents
- Not ideal for direct ingestion

### 3. Hybrid model (chosen)

- HTTP/gRPC at edge
- Kafka internally
- Best balance of simplicity and scalability

---

## Consequences

### Positive

- Flexible ingestion layer
- Scalable backend pipeline
- Clear separation of concerns

### Negative

- More moving parts
- Requires protocol consistency design
