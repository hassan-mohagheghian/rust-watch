# ADR 0001: System Architecture Style

## Status

Accepted

---

## Context

RustWatch is a distributed observability platform that must handle:

- High-throughput telemetry ingestion
- Horizontal scalability
- Fault tolerance
- Streaming data pipelines

We need to decide the fundamental architecture style.

---

## Decision

We choose a **distributed event-driven microservices architecture**.

System components will be:

- Agent (data collection)
- Collector (ingestion layer)
- Streaming backbone (Kafka)
- Processing layer
- Storage layer (ClickHouse)
- Query API

---

## Alternatives Considered

### 1. Monolithic architecture

- Simpler initial implementation
- Poor scalability
- Not suitable for distributed ingestion workloads

### 2. Modular monolith

- Better separation than monolith
- Still limits horizontal scaling of ingestion pipeline

### 3. Event-driven microservices (chosen)

- High scalability
- Fault isolation
- Natural fit for telemetry streaming systems

---

## Consequences

### Positive

- Scalable architecture
- Independent services
- Aligns with industry observability systems

### Negative

- Higher complexity
- Requires distributed system expertise
- More infrastructure overhead

---

## Rationale

Observability systems are inherently distributed and event-driven. This model best reflects real-world systems like Datadog, Prometheus pipelines, and OpenTelemetry-based infrastructures.
