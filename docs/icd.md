![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# Interface Control Document (ICD) - `svc-compliance`

## :telescope: Overview

This document defines the gRPC and REST interfaces unique to the
`svc-compliance` microservice.

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Stuckee       | A.M. Smith [@ServiceDog](https://github.com/servicedog)           |
| Status        | Draft                                                             |

## :books: Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md) | Interfaces and frameworks common to all Arrow microservices.
[Requirements - `svc-compliance`](https://nocodb.arrowair.com/dashboard/#/nc/view/d1bb0a51-e22f-4b91-b1c5-66f11f4f861b) | Requirements and user stories for this microservice.
[Concept of Operations - `svc-compliance`](./conops.md) | Defines the motivation and duties of this microservice.
[Software Design Document (SDD) - `svc-compliance`](./sdd.md) | Specifies the internal activity of this microservice.

## :hammer: Frameworks

See the [High-Level Services ICD](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md).

## REST

This microservice implements no additional REST endpoints beyond the common REST interfaces (see High-Level ICD).

## :speech_balloon: gRPC

### Files

These interfaces are defined in a protocol buffer file,
[`grpc.proto`](../proto/grpc.proto).

### Integrated Authentication & Encryption

See [High-Level Services ICD](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md).

### gRPC Server Methods ("Services")

gRPC server methods are called "services", an unfortunate name clash with the broader concept of web services.

| Service | Description |
| ---- | ---- |
| `IsReady` | Returns a message indicating if this service is ready for requests.<br>Similar to a health check, if a server is not "ready" it could be considered dead by the client making the request.
| submitFlightPlan | Submit a flight plan to the regional authority.
| requestFlightRelease | Submit a flight release (pre-takeoff) request.
| requestWaypoints | Request a list of waypoints within the bounds of provided coordinates.
| requestRestrictions | Request a list of no-fly zones and temporary flight restrictions within the bounds of provided coordinates.
