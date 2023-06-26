![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

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
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Stuckee       | A.M. Smith [@ServiceDog](https://github.com/servicedog)           |
| Status        | Draft                                                             |

## :books: Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Arrow microservices.
[Requirements - `svc-compliance`](https://nocodb.arrowair.com/dashboard/#/nc/view/d1bb0a51-e22f-4b91-b1c5-66f11f4f861b) | Requirements and user stories for this microservice.
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

As a GRPC server, this service awaits requests and executes handlers.

### Cleanup

No special cleanup events.

## :speech_balloon: gRPC Handlers

The following functions are implemented for each region:
- submit_flight_plan
- request_flight_release

Regions may have unique processes and endpoints for performing these tasks.

:warning: In R2 these handlers simply return a "submitted: true" message without connecting to external APIs. This will be updated in later releases, and potentially obscured depending on government requirements.
