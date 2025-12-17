# Fusabi Hibana Sources Type Provider

A type provider for Fusabi that generates types for Hibana observability agent data sources.

## Overview

Hibana is a Fusabi-powered observability agent that collects metrics, logs, traces, and events from various sources. This provider generates strongly-typed Fusabi types for configuring these data sources.

## Features

- **Metrics Sources**: Prometheus scraping, StatsD, system metrics, host metrics
- **Logs Sources**: File logs, Syslog, Journald, Docker, Kubernetes logs
- **Traces Sources**: OTLP, Jaeger, Zipkin
- **Events Sources**: eBPF, Audit logs, CloudWatch, EventBridge
- **Common Types**: TLS configuration, retry policies, authentication, buffering

## Usage

```fusabi
type provider HibanaSources from "embedded" {
    namespace = "HibanaSources"
}
```

## Generated Type Modules

### Common Types

- `TlsConfig`: TLS/SSL configuration
- `RetryConfig`: Retry and backoff configuration
- `BufferConfig`: Buffering configuration
- `AuthConfig`: Authentication configuration

### Metrics Sources

- `PrometheusScrape`: Prometheus metrics scraping configuration
  - `endpoint`: Prometheus endpoint URL
  - `interval`: Scrape interval in seconds
  - `labels`: Optional static labels
  - `timeout`: Request timeout
  - `scrapeProtocol`: Protocol version
  - `honorLabels`: Honor labels from target
  - `tlsConfig`: TLS configuration

- `StatsDSource`: StatsD metrics receiver
  - `address`: Listen address
  - `port`: Listen port
  - `protocol`: UDP or TCP
  - `metricsPrefix`: Optional prefix for all metrics
  - `parseMetricTags`: Enable tag parsing
  - `aggregationInterval`: Aggregation interval

- `SystemMetrics`: System metrics collector
  - `interval`: Collection interval
  - `collectCpu`, `collectMemory`, `collectDisk`, `collectNetwork`, `collectProcesses`: Feature flags
  - `namespacePrefix`: Metric namespace prefix

- `HostMetrics`: Host-level metrics collector
  - `interval`: Collection interval
  - `rootPath`: Root filesystem path
  - `collectors`: List of enabled collectors
  - `filters`: Collector-specific filters

### Logs Sources

- `FileLog`: File-based log collection
  - `path`: File path or glob pattern
  - `encoding`: File encoding
  - `multiline`: Multiline configuration
  - `includeMetadata`: Include file metadata
  - `startPosition`: Start reading from beginning or end
  - `glob`: Enable glob pattern matching
  - `exclude`: Exclude patterns
  - `maxLineBytes`: Maximum line size

- `MultilineConfig`: Multiline log handling
  - `pattern`: Regex pattern
  - `negate`: Negate pattern match
  - `match`: Match behavior (after/before)
  - `maxLines`: Maximum lines per event
  - `timeout`: Multiline timeout

- `Syslog`: Syslog receiver
  - `address`: Listen address
  - `port`: Listen port
  - `protocol`: TCP or UDP
  - `mode`: RFC3164 or RFC5424
  - `maxMessageSize`: Maximum message size
  - `frameDelimiter`: Frame delimiter

- `Journald`: systemd journal collector
  - `currentBootOnly`: Only collect from current boot
  - `units`: Filter by systemd units
  - `includeKernel`: Include kernel messages
  - `batchSize`: Batch size
  - `sinceNow`: Start from now
  - `journalDirectory`: Journal directory path

- `Docker`: Docker log collector
  - `dockerHost`: Docker daemon host
  - `includeContainers`, `excludeContainers`: Container filters
  - `includeLabels`, `excludeLabels`: Label filters
  - `partialEventMarkerField`: Partial event marker
  - `autoPartialMerge`: Auto-merge partial events

- `KubernetesLogs`: Kubernetes log collector
  - `namespaces`, `excludeNamespaces`: Namespace filters
  - `labelSelector`: Label selector
  - `fieldSelector`: Field selector
  - `annotationFields`: Annotation field mappings
  - `selfNodeName`: Node name for filtering

### Traces Sources

- `OtlpTrace`: OpenTelemetry Protocol trace receiver
  - `endpoint`: OTLP endpoint
  - `protocol`: gRPC or HTTP
  - `headers`: Custom headers
  - `timeout`: Request timeout
  - `compression`: Compression algorithm
  - `tlsConfig`: TLS configuration
  - `retryConfig`: Retry configuration

- `Jaeger`: Jaeger trace receiver
  - `endpoint`: Jaeger endpoint
  - `protocol`: Thrift or gRPC
  - `agentHost`, `agentPort`: Agent connection
  - `sampler`: Sampling configuration
  - `tags`: Static tags

- `Zipkin`: Zipkin trace receiver
  - `endpoint`: Zipkin endpoint
  - `port`: Listen port
  - `collectorEndpoint`: Collector endpoint
  - `maxPayloadSize`: Maximum payload size
  - `v2Format`: Use Zipkin v2 format

- `SamplerConfig`: Trace sampling configuration
  - `samplerType`: Sampler type (const, probabilistic, etc.)
  - `param`: Sampler parameter
  - `samplingServerUrl`: Remote sampling server
  - `maxOperations`: Maximum operations

### Events Sources

- `EbpfSource`: eBPF program event source
  - `programPath`: eBPF program path
  - `programType`: Program type
  - `attachPoint`: Attach point
  - `mapNames`: eBPF map names
  - `pollInterval`: Polling interval
  - `kernelVersion`: Required kernel version

- `Audit`: Linux audit log collector
  - `socketPath`: Audit socket path
  - `auditdPath`: auditd log file path
  - `rules`: Audit rules
  - `resolveIds`: Resolve UIDs/GIDs
  - `bufferSize`: Buffer size

- `CloudWatch`: AWS CloudWatch logs collector
  - `region`: AWS region
  - `logGroupName`: Log group name
  - `logStreamName`: Log stream name
  - `filterPattern`: CloudWatch filter pattern
  - `startTime`: Start time
  - `pollInterval`: Polling interval
  - `awsProfile`: AWS profile

- `EventBridge`: AWS EventBridge event source
  - `region`: AWS region
  - `eventBusName`: Event bus name
  - `ruleNames`: Event rule names
  - `eventPattern`: Event pattern filter
  - `awsProfile`: AWS profile

## Example Configuration

```fusabi
type provider HibanaSources from "embedded" {
    namespace = "HibanaSources"
}

let prometheusSource: HibanaSources.Metrics.PrometheusScrape = {
    endpoint = "http://localhost:9090/metrics",
    interval = 15,
    labels = Some({
        environment = "production",
        datacenter = "us-east-1"
    }),
    timeout = Some(10),
    tlsConfig = None
}

let fileLogSource: HibanaSources.Logs.FileLog = {
    path = "/var/log/app/*.log",
    encoding = Some("utf-8"),
    multiline = Some({
        pattern = "^\\d{4}-\\d{2}-\\d{2}",
        negate = Some(true),
        match = Some("after"),
        maxLines = Some(100),
        timeout = Some(5)
    }),
    includeMetadata = Some(true),
    startPosition = Some("end"),
    glob = Some(true),
    exclude = Some(["/var/log/app/*.tmp"]),
    maxLineBytes = Some(1048576)
}

let otlpTrace: HibanaSources.Traces.OtlpTrace = {
    endpoint = "localhost:4317",
    protocol = "grpc",
    headers = Some({
        "x-api-key" = "secret"
    }),
    timeout = Some(30),
    compression = Some("gzip"),
    tlsConfig = None,
    retryConfig = Some({
        enabled = true,
        initialInterval = Some(1000),
        maxInterval = Some(30000),
        maxElapsedTime = Some(300000),
        multiplier = Some(2.0)
    })
}
```

## Testing

Run the test suite:

```bash
cargo test -p fusabi-provider-hibana-sources
```

## License

MIT
