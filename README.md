Core Concept

This project involves building a simple web service (e.g., a user profile API) where every incoming request is automatically instrumented to generate distributed traces and performance metrics, which are then exported to an observability backend.
How It Works

    Axum Router: Handles HTTP requests (GET /users/:id, POST /users, etc.)

    OpenTelemetry Integration: Uses tower-http tracing middleware and opentelemetry crates

    Trace Propagation: Automatically handles W3C trace context headers for distributed tracing

    Exporters: Sends telemetry data to:

        Jaeger (for traces visualization)

        Stdout (for development)

        Prometheus (for metrics, optional)