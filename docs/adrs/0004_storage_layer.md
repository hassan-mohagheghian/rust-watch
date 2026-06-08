# ADR 0004: Storage Layer Choice

## Status

Accepted

---

## Context

Telemetry data requires:

- High write throughput
- Time-series queries
- Aggregations over large datasets

---

## Decision

We choose **ClickHouse** as the primary storage engine.

---

## Alternatives Considered

### 1. PostgreSQL

- Strong general DB
- Poor performance at telemetry scale

### 2. Elasticsearch

- Good for logs
- Expensive at scale
- Complex operations

### 3. ClickHouse (chosen)

- Columnar storage
- Extremely fast analytics queries
- Designed for observability workloads

---

## Consequences

### Positive

- High-performance analytics
- Efficient storage
- Industry-proven at scale

### Negative

- Learning curve
- Requires schema design discipline
