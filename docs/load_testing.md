# Load Testing Strategy

## Current Setup

RustWatch is tested using multiple agent instances.

---

## Configuration

- 5 agents running concurrently
- 50 events/sec per agent
- Total load: ~250 events/sec

---

## Purpose

This load is used to evaluate:

- CPU utilization
- queue behavior
- worker throughput
- system stability

---

## Agent Behavior

Each agent:

- generates telemetry events continuously
- sends HTTP requests to collector
- simulates real service traffic

---

## Observed Effects

At current load:

- CPU usage increases significantly
- worker processes events continuously
- queue buffers incoming load
- no batching leads to per-event overhead

---

## Recommended Load Range

For current architecture:

- 100–300 events/sec → stable testing range
- >500 events/sec → CPU saturation begins
- >1000 events/sec → inefficient without batching

---

## Next Step

Introduce batching to:

- reduce per-event overhead
- improve throughput efficiency
- stabilize CPU usage
