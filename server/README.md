![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# svc-compliance Service

![GitHub stable release (latest by date)](https://img.shields.io/github/v/release/Arrow-air/svc-compliance?sort=semver&color=green) ![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/Arrow-air/svc-compliance?include_prereleases) [![Coverage Status](https://coveralls.io/repos/github/Arrow-air/svc-compliance/badge.svg?branch=develop)](https://coveralls.io/github/Arrow-air/svc-compliance)
![Sanity Checks](https://github.com/arrow-air/svc-compliance/actions/workflows/sanity_checks.yml/badge.svg?branch=develop) ![Python PEP8](https://github.com/arrow-air/svc-compliance/actions/workflows/python_ci.yml/badge.svg?branch=develop) ![Rust Checks](https://github.com/arrow-air/svc-compliance/actions/workflows/rust_ci.yml/badge.svg?branch=develop) 
![Arrow DAO Discord](https://img.shields.io/discord/853833144037277726?style=plastic)

## Overview

svc-compliance is responsible for all communication with the regional civil aviation authority (CAA).

This includes:
- Submitting flight plans
- Requesting flight releases
- Obtaining [temporary flight restrictions](https://www.faa.gov/uas/getting_started/temporary_flight_restrictions) (TFRs)
- Obtaining no-fly and [no-drone](https://www.faa.gov/uas/resources/community_engagement/no_drone_zone) zones
- Obtaining [NOTAMs](https://www.faa.gov/about/initiatives/notam/what_is_a_notam)
- Obtaining [waypoints](https://www.faa.gov/air_traffic/flight_info/aeronav/aero_data/loc_id_search/fixes_waypoints/)
