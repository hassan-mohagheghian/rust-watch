# ADR 0003: Streaming Layer Choice

## Status

Accepted

---

## Context

We need a durable, scalable streaming system for:

- Log ingestion
- Metric events
- Trace pipelines

---

## Decision

We use **Apache Kafka** as the streaming backbone.

---

## Alternatives Considered

### 1. RabbitMQ

- Easier setup
- Not optimized for large-scale streaming

### 2. NATS

- Lightweight
- Less mature ecosystem for analytics pipelines

### 3. Kafka (chosen)

- Industry standard
- High throughput
- Strong partitioning model
- Replay capability

---

## Consequences

### Positive

- Production-grade streaming backbone
- Horizontal scalability
- Strong ecosystem

### Negative

- Operational complexity
- Requires cluster management
