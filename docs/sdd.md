![Aetheric Banner](https://github.com/aetheric-oss/.github/raw/main/assets/doc-banner.png)

# Software Design Document (SDD) - `svc-compliance` 

## :telescope: Overview

This document details the software implementation of svc-compliance.

This service is responsible for some or all of the following:
- Submitting flight plans to the regional aviation authority
- Obtaining flight releases prior to takeoff
- Obtaining temporary flight restrictions
- Validating certification numbers and expiration dates
- Waypoints/Fixes

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Aetheric Realm Team](https://github.com/orgs/aetheric-oss/teams/dev-realm) |
| Stuckee       | A.M. Smith [@amsmith-pro](https://github.com/amsmith-pro)           |
| Status        | Draft                                                             |

## :books: Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/aetheric-oss/se-services/blob/develop/docs/conops.md) | Overview of Aetheric microservices.
[High-Level Interface Control Document (ICD)](https://github.com/aetheric-oss/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Aetheric microservices.
[Requirements - `svc-compliance`](https://nocodb.aetheric.nl/dashboard/#/nc/view/d1bb0a51-e22f-4b91-b1c5-66f11f4f861b) | Requirements and user stories for this microservice.
[Concept of Operations - `svc-compliance`](./conops.md) | Defines the motivation and duties of this microservice.
[Interface Control Document (ICD) - `svc-compliance`](./icd.md) | Defines the inputs and outputs of this microservice.

## :dna: Module Attributes

Attribute | Applies | Explanation
--- | --- | ---
Safety Critical | N | Flights without approvals will not be permitted to occur.
Realtime | N | Flight plan submissions and the receipt of approval may occur seconds or minutes apart, does not need to be instantaneous or occur on a precise schedule.

## :gear: Logic

### Initialization

At initialization this service creates a GRPC server.

The GRPC server expects the following environment variables to be set:
- `DOCKER_PORT_GRPC` (default: `50051`)

This service also expects the following other environment variables to be set:
- `REGION_CODE`
    | Possible Value | Country |
    | --- | --- | 
    | us | United States of America |
    | nl | Netherlands |

### Loop

#### GRPC

As a GRPC server, this service awaits requests and executes handlers. See [interface handlers](#speech_balloon-interface-handlers) for more information.

#### Waypoints

This service is responsible for periodically checking with an external database for updates to waypoints.

```mermaid

sequenceDiagram;

participant gis as svc-gis
participant compliance as svc-compliance
participant authority as Civil Aviation Authority

alt Loop
compliance -->> authority: Get waypoints
authority -->> compliance: Waypoints
Note over compliance: Detect Changes
    alt If Waypoints Change
        compliance -->> gis: update_waypoints(...)
        compliance -->> gis: delete_waypoints(...) (R4)
    end
Note over compliance: Wait N Seconds
end

```

#### No-Fly Zones

This service is responsible for periodically checking with an external database for updates to no-fly zones.


```mermaid

sequenceDiagram;

participant gis as svc-gis
participant compliance as svc-compliance
participant authority as Civil Aviation Authority

alt Loop
compliance -->> authority: Get no-fly zones, NOTAMs
authority -->> compliance: No-Fly Zones
Note over compliance: Detect Changes
    alt If Change
        compliance -->> gis: update_no_fly_zones(...)
        compliance -->> gis: delete_no_fly_zones(...) (R4)
    end
Note over compliance: Wait N Seconds
end

```

### Cleanup

No special cleanup events.

## :speech_balloon: Interface Handlers

The following functions are implemented for each region:
- submit_flight_plan
- request_flight_release

Regions may have unique processes and endpoints for performing these tasks.

:warning: These handlers currently return a "submitted: true" message without connecting to external APIs. This will be updated in later releases, and potentially obscured depending on government requirements. Submitted flight plans are additionally broadcast over an AMQP (RabbitMQ) channel to listeners in R3.
