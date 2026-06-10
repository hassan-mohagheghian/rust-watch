# RustWatch Roadmap

## Completed

### Ingestion Prototype

- HTTP collector built with Axum
- DTO validation implemented
- Domain model introduced
- In-memory queue (mpsc)
- Async worker pipeline
- Multi-agent load generator

---

## Current Focus

### Performance & Pipeline Optimization

- CPU usage optimization under load
- Queue behavior analysis
- Worker throughput measurement

---

## Next Milestone

### Batching Worker (High Priority)

Introduce:

- Time-based batching (flush interval)
- Size-based batching (event threshold)
- Batch processing instead of per-event processing

Expected impact:

- Reduced CPU usage
- Higher throughput
- Foundation for Kafka-style ingestion

---

## Future Milestones

### Distributed Messaging

- Kafka integration
- Event streaming pipeline

---

### Storage Layer

- ClickHouse integration
- Time-series optimization

---

### Observability Layer

- Internal metrics
- System health endpoints
- Performance dashboards

---

### Advanced Features

- Backpressure handling
- Multi-node collectors
- Horizontal scaling
