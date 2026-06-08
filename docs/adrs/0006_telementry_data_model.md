# ADR 0006: Telemetry Data Model

## Status

Accepted

---

## Context

We must define a unified model for:

- Logs
- Metrics
- Traces
- Events

---

## Decision

We define a unified `TelemetryEvent` structure:

- service_name
- timestamp
- event_type
- payload (flexible JSON or binary encoding later)

---

## Alternatives Considered

### 1. Separate models per signal type

- More precise
- Harder pipeline management

### 2. Unified event model (chosen)

- Simplifies ingestion pipeline
- Easier streaming and processing
- Enables future AI processing layer

---

## Consequences

### Positive

- Unified pipeline design
- Easier scaling
- AI-ready structured data

### Negative

- Less strict typing per signal type
- Requires careful schema evolution later
