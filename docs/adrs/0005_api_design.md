# ADR 0005: API Design Strategy

## Status

Accepted

---

## Context

We need a unified API for:

- Querying logs
- Fetching metrics
- Retrieving traces

---

## Decision

We implement:

- REST API (primary)
- gRPC (internal/high-performance use cases)

---

## Alternatives Considered

### 1. REST only

- Simple
- Not optimal for internal services

### 2. gRPC only

- High performance
- Less accessible for external users

### 3. Hybrid (chosen)

- REST for usability
- gRPC for performance-critical paths

---

## Consequences

### Positive

- Flexible API layer
- Developer-friendly
- High-performance internal communication

### Negative

- Dual API maintenance
