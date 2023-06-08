# Changelog

## 0.7.0-rc.1

This update refactors all of the clients and changes the way responses are handled. Previously methods that make calls to a third party service would return an `anyhow::Result<T>` over some structured interpretation of the response body (i.e. a deserialized struct, a unit type, or a raw string). Methods now return a `Result<Response<T>, ClientError>` where a `Response` contains the body type that was previously returned along with the status code and headers that were sent from the third party service. When requesting unfolded paginated results the status code and headers returned will be the values returned from the final request made to the service. `ClientError` now provides a more structure error type than the previous `anyhow` error.

### Breaking Changes

* Methods now return `Result<Response<T>, ClientError>` instead of `anyhow::Result<T>`. The body can be accessed via `response.body`.
* Structured errors are now returned instead of `anyhow::Error`.
* Redirects are no longer followed by default. Previously if a 3xx response code was returned by a service, the inner client would follow it. This prevented services from returning meaningful 3xx responses. Methods that receive 3xx responses will now return them immediately and not attempt to follow them.
* Updates to 0.2 `reqwest-middleware` ecosystem.
* OpenTelemetry is no longer a default feature. To enable OpenTelemetry tracing add `reqwest-tracing` with the appropriate version feature enabled to the target project.