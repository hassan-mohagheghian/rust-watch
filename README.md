# RustWatch

## A Distributed Observability & Telemetry Platform Built with Rust

> High-performance telemetry ingestion, processing, and querying for modern distributed systems.

---

## Overview

RustWatch is a distributed observability platform for collecting, processing, and querying telemetry data (logs, metrics, traces, events) from modern infrastructure systems.

It is designed as a **systems engineering project**, focused on:

- Distributed systems design
- High-performance Rust services
- Streaming pipelines
- Scalable data infrastructure

---

## Why RustWatch?

Modern systems are becoming:

- Highly distributed
- Microservice-heavy
- Cloud-native
- AI-driven

This creates massive demand for observability systems capable of handling:

- High event throughput
- Real-time analytics
- Fault tolerance
- Scalable ingestion pipelines

RustWatch is a learning + engineering project to explore how such systems are built from first principles.

---

## Core Components

- **Telemetry Agent** → collects logs, metrics, traces
- **Collector Layer** → Rust-based ingestion services
- **Streaming Layer** → Kafka event pipeline
- **Storage Layer** → ClickHouse analytical database
- **Query API** → unified access to telemetry data

---

## Architecture

See `/docs/architecture.md`

---

## MVP Scope

See `/docs/mvp-scope.md`

---

## Roadmap

See `/docs/roadmap.md`

---

## License

MIT
