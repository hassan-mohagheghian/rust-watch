# Architecture

## Overview

RustWatch is built as a distributed pipeline:

Agent → Collector → Kafka → Processor → ClickHouse → Query API

---

## Components

### 1. Agent Layer

- Runs on machines / containers
- Collects telemetry data
- Sends asynchronously

---

### 2. Collector Layer (Rust + Tokio)

- Accepts incoming telemetry
- Applies batching and validation
- Forwards to streaming layer

---

### 3. Streaming Layer (Kafka)

- Provides durability
- Enables backpressure handling
- Supports partitioning and scaling

---

### 4. Processing Layer

- Cleans and transforms data
- Prepares data for storage

---

### 5. Storage Layer (ClickHouse)

- Optimized for analytical queries
- Handles large-scale time-series data

---

### 6. Query API (Axum)

- Exposes REST/gRPC endpoints
- Supports filtering and aggregation