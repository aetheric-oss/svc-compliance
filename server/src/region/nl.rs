use crate::grpc::server::grpc_server::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse, Waypoint,
    WaypointsRequest, WaypointsResponse,
};

use crate::region::RegionInterface;

use tonic::{Request, Response, Status};

/// Processes for submission to the Dutch (Netherlands) authorities
pub struct NLImpl {}
impl RegionInterface for NLImpl {
    fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        region_info!("([nl] submit_flight_plan) entry.");
        // TODO R3 implement
        let flight_plan_id = request.into_inner().flight_plan_id;
        Ok(Response::new(FlightPlanResponse {
            flight_plan_id,
            submitted: true,
            result: None,
        }))
    }

    fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        region_info!("([nl] request_flight_release) entry.");
        // TODO R3 implement
        let flight_plan_id = request.into_inner().flight_plan_id;
        Ok(Response::new(FlightReleaseResponse {
            flight_plan_id,
            released: true,
            result: None,
        }))
    }

    fn request_waypoints(
        &self,
        _request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status> {
        region_info!("([nl] request_waypoints) entry.");

        //
        // TODO(R4): This is hardcoded in R3. Eventually an external API should be called.
        //

        // let Ok(fname) = std::env::var("NL_WAYPOINTS_CSV") else {
        //     return Err(Status::internal("(env) NL_WAYPOINTS_CSV not set."));
        // };

        // region_debug!("([nl] request_waypoints) waypoints csv: {:?}", fname);
        // let Ok(waypoints) = super::utils::parse_waypoints_file(&fname, request.filter) else {
        //     return Err(Status::internal("([nl] request_waypoints) failed to parse waypoints file."));
        // };

        // North Holland Waypoints Hardcode
        let waypoints: Vec<Waypoint> = vec![
            Waypoint {
                identifier: "ABA02".to_string(),
                latitude: 52.2576,
                longitude: 4.7740,
            },
            Waypoint {
                identifier: "ABA03".to_string(),
                latitude: 52.2410,
                longitude: 4.7724,
            },
            Waypoint {
                identifier: "ABA05".to_string(),
                latitude: 52.2078,
                longitude: 4.7692,
            },
            Waypoint {
                identifier: "ABA38".to_string(),
                latitude: 52.2244,
                longitude: 4.7708,
            },
            Waypoint {
                identifier: "ANDIK".to_string(),
                latitude: 52.7394,
                longitude: 5.2705,
            },
            Waypoint {
                identifier: "BASNO".to_string(),
                latitude: 52.3502,
                longitude: 4.5750,
            },
            Waypoint {
                identifier: "BETUS".to_string(),
                latitude: 52.6881,
                longitude: 5.1626,
            },
            Waypoint {
                identifier: "EH613".to_string(),
                latitude: 52.2091,
                longitude: 4.4626,
            },
            Waypoint {
                identifier: "EKROS".to_string(),
                latitude: 52.2373,
                longitude: 4.6195,
            },
            Waypoint {
                identifier: "ENKOS".to_string(),
                latitude: 52.6781,
                longitude: 5.2433,
            },
            Waypoint {
                identifier: "HDR01".to_string(),
                latitude: 52.8824,
                longitude: 4.7582,
            },
            Waypoint {
                identifier: "HDR02".to_string(),
                latitude: 52.9353,
                longitude: 4.7947,
            },
            Waypoint {
                identifier: "HDR05".to_string(),
                latitude: 52.9758,
                longitude: 4.8432,
            },
            Waypoint {
                identifier: "HDR18".to_string(),
                latitude: 52.8570,
                longitude: 4.7664,
            },
            Waypoint {
                identifier: "HDR20".to_string(),
                latitude: 52.9352,
                longitude: 4.7949,
            },
            Waypoint {
                identifier: "HDR40".to_string(),
                latitude: 52.9454,
                longitude: 4.8186,
            },
            Waypoint {
                identifier: "HDR41".to_string(),
                latitude: 52.9635,
                longitude: 4.8238,
            },
            Waypoint {
                identifier: "HDR45".to_string(),
                latitude: 52.9425,
                longitude: 4.8239,
            },
            Waypoint {
                identifier: "HDR52".to_string(),
                latitude: 52.8219,
                longitude: 4.7395,
            },
            Waypoint {
                identifier: "HDR57".to_string(),
                latitude: 53.0096,
                longitude: 4.8834,
            },
            Waypoint {
                identifier: "HDR71".to_string(),
                latitude: 52.7890,
                longitude: 4.7773,
            },
            Waypoint {
                identifier: "HDR75".to_string(),
                latitude: 53.0132,
                longitude: 4.8744,
            },
            Waypoint {
                identifier: "HDR96".to_string(),
                latitude: 53.0244,
                longitude: 4.9453,
            },
            Waypoint {
                identifier: "HDR97".to_string(),
                latitude: 53.0205,
                longitude: 4.9520,
            },
            Waypoint {
                identifier: "IVLUT".to_string(),
                latitude: 52.2441,
                longitude: 5.2570,
            },
            Waypoint {
                identifier: "KAG01".to_string(),
                latitude: 52.2793,
                longitude: 4.7104,
            },
            Waypoint {
                identifier: "KAG02".to_string(),
                latitude: 52.2705,
                longitude: 4.6873,
            },
            Waypoint {
                identifier: "KAG03".to_string(),
                latitude: 52.2618,
                longitude: 4.6642,
            },
            Waypoint {
                identifier: "KAG05".to_string(),
                latitude: 52.2443,
                longitude: 4.6180,
            },
            Waypoint {
                identifier: "KAG22".to_string(),
                latitude: 52.2879,
                longitude: 4.7340,
            },
            Waypoint {
                identifier: "KAG62".to_string(),
                latitude: 52.2338,
                longitude: 4.5904,
            },
            Waypoint {
                identifier: "KAG91".to_string(),
                latitude: 52.2077,
                longitude: 4.5208,
            },
            Waypoint {
                identifier: "R1811".to_string(),
                latitude: 52.2908,
                longitude: 4.7773,
            },
            Waypoint {
                identifier: "R1812".to_string(),
                latitude: 52.2891,
                longitude: 4.7373,
            },
            Waypoint {
                identifier: "R1813".to_string(),
                latitude: 52.3314,
                longitude: 4.7400,
            },
            Waypoint {
                identifier: "R1814".to_string(),
                latitude: 52.3184,
                longitude: 4.7969,
            },
            Waypoint {
                identifier: "R2205".to_string(),
                latitude: 52.9264,
                longitude: 4.7839,
            },
            Waypoint {
                identifier: "SPL28".to_string(),
                latitude: 52.3736,
                longitude: 4.7848,
            },
            Waypoint {
                identifier: "SPL42".to_string(),
                latitude: 52.3182,
                longitude: 4.8517,
            },
            Waypoint {
                identifier: "SPL43".to_string(),
                latitude: 52.3187,
                longitude: 4.8789,
            },
            Waypoint {
                identifier: "SPL46".to_string(),
                latitude: 52.3199,
                longitude: 4.9658,
            },
            Waypoint {
                identifier: "SPL56".to_string(),
                latitude: 52.3191,
                longitude: 4.9061,
            },
            Waypoint {
                identifier: "SPL72".to_string(),
                latitude: 52.4148,
                longitude: 5.1348,
            },
            Waypoint {
                identifier: "WP24".to_string(),
                latitude: 52.2891,
                longitude: 5.1232,
            },
            Waypoint {
                identifier: "WP41".to_string(),
                latitude: 52.2617,
                longitude: 5.1187,
            },
            Waypoint {
                identifier: "ZANDA".to_string(),
                latitude: 52.3898,
                longitude: 4.9225,
            },
            Waypoint {
                identifier: "AMSOT".to_string(),
                latitude: 53.3133,
                longitude: 4.5858,
            },
            Waypoint {
                identifier: "ATRIX".to_string(),
                latitude: 53.1385,
                longitude: 4.6246,
            },
            Waypoint {
                identifier: "BAKLU".to_string(),
                latitude: 52.9850,
                longitude: 4.7727,
            },
            Waypoint {
                identifier: "BUROG".to_string(),
                latitude: 53.0448,
                longitude: 4.7781,
            },
            Waypoint {
                identifier: "DISRA".to_string(),
                latitude: 53.3333,
                longitude: 4.4195,
            },
            Waypoint {
                identifier: "EH001".to_string(),
                latitude: 52.2752,
                longitude: 4.7011,
            },
            Waypoint {
                identifier: "EH005".to_string(),
                latitude: 52.2738,
                longitude: 4.6975,
            },
            Waypoint {
                identifier: "EH006".to_string(),
                latitude: 52.5907,
                longitude: 4.5462,
            },
            Waypoint {
                identifier: "EH008".to_string(),
                latitude: 52.2083,
                longitude: 4.7667,
            },
            Waypoint {
                identifier: "EH009".to_string(),
                latitude: 52.2237,
                longitude: 4.5546,
            },
            Waypoint {
                identifier: "EH010".to_string(),
                latitude: 52.3780,
                longitude: 4.7693,
            },
            Waypoint {
                identifier: "EH012".to_string(),
                latitude: 52.4204,
                longitude: 4.7176,
            },
            Waypoint {
                identifier: "EH013".to_string(),
                latitude: 52.5447,
                longitude: 4.7293,
            },
            Waypoint {
                identifier: "EH014".to_string(),
                latitude: 52.3377,
                longitude: 4.8638,
            },
            Waypoint {
                identifier: "EH015".to_string(),
                latitude: 52.5909,
                longitude: 4.7345,
            },
            Waypoint {
                identifier: "EH016".to_string(),
                latitude: 52.3987,
                longitude: 4.4146,
            },
            Waypoint {
                identifier: "EH018".to_string(),
                latitude: 52.3204,
                longitude: 4.8186,
            },
            Waypoint {
                identifier: "EH019".to_string(),
                latitude: 52.3380,
                longitude: 4.8373,
            },
            Waypoint {
                identifier: "EH020".to_string(),
                latitude: 52.3369,
                longitude: 4.9288,
            },
            Waypoint {
                identifier: "EH021".to_string(),
                latitude: 52.4767,
                longitude: 4.6557,
            },
            Waypoint {
                identifier: "EH022".to_string(),
                latitude: 52.6805,
                longitude: 4.5219,
            },
            Waypoint {
                identifier: "EH023".to_string(),
                latitude: 52.2559,
                longitude: 4.8481,
            },
            Waypoint {
                identifier: "EH024".to_string(),
                latitude: 52.2530,
                longitude: 4.9806,
            },
            Waypoint {
                identifier: "EH025".to_string(),
                latitude: 52.2068,
                longitude: 4.7400,
            },
            Waypoint {
                identifier: "EH027".to_string(),
                latitude: 52.2509,
                longitude: 5.0261,
            },
            Waypoint {
                identifier: "EH028".to_string(),
                latitude: 52.3578,
                longitude: 4.4210,
            },
            Waypoint {
                identifier: "EH030".to_string(),
                latitude: 52.2616,
                longitude: 4.8013,
            },
            Waypoint {
                identifier: "EH032".to_string(),
                latitude: 52.2641,
                longitude: 4.5885,
            },
            Waypoint {
                identifier: "EH033".to_string(),
                latitude: 52.3885,
                longitude: 4.7845,
            },
            Waypoint {
                identifier: "EH034".to_string(),
                latitude: 52.5178,
                longitude: 4.5767,
            },
            Waypoint {
                identifier: "EH036".to_string(),
                latitude: 52.2570,
                longitude: 4.8479,
            },
            Waypoint {
                identifier: "EH037".to_string(),
                latitude: 52.2579,
                longitude: 4.7744,
            },
            Waypoint {
                identifier: "EH041".to_string(),
                latitude: 52.3227,
                longitude: 4.4265,
            },
            Waypoint {
                identifier: "EH042".to_string(),
                latitude: 52.2956,
                longitude: 4.9621,
            },
            Waypoint {
                identifier: "EH043".to_string(),
                latitude: 52.4254,
                longitude: 5.1318,
            },
            Waypoint {
                identifier: "EH044".to_string(),
                latitude: 52.4351,
                longitude: 4.8637,
            },
            Waypoint {
                identifier: "EH046".to_string(),
                latitude: 52.2179,
                longitude: 4.7296,
            },
            Waypoint {
                identifier: "EH047".to_string(),
                latitude: 52.4310,
                longitude: 4.7185,
            },
            Waypoint {
                identifier: "EH048".to_string(),
                latitude: 52.2701,
                longitude: 4.7344,
            },
            Waypoint {
                identifier: "EH049".to_string(),
                latitude: 52.2892,
                longitude: 4.6124,
            },
            Waypoint {
                identifier: "EH051".to_string(),
                latitude: 52.2438,
                longitude: 4.6510,
            },
            Waypoint {
                identifier: "EH052".to_string(),
                latitude: 52.3242,
                longitude: 4.8998,
            },
            Waypoint {
                identifier: "EH053".to_string(),
                latitude: 52.3872,
                longitude: 4.7897,
            },
            Waypoint {
                identifier: "EH055".to_string(),
                latitude: 52.3208,
                longitude: 4.8665,
            },
            Waypoint {
                identifier: "EH056".to_string(),
                latitude: 52.3294,
                longitude: 4.6508,
            },
            Waypoint {
                identifier: "EH057".to_string(),
                latitude: 52.3712,
                longitude: 4.6909,
            },
            Waypoint {
                identifier: "EH058".to_string(),
                latitude: 52.3301,
                longitude: 4.6757,
            },
            Waypoint {
                identifier: "EH059".to_string(),
                latitude: 52.3469,
                longitude: 4.8501,
            },
            Waypoint {
                identifier: "EH060".to_string(),
                latitude: 52.3052,
                longitude: 4.9070,
            },
            Waypoint {
                identifier: "EH061".to_string(),
                latitude: 52.2251,
                longitude: 4.8895,
            },
            Waypoint {
                identifier: "EH062".to_string(),
                latitude: 52.2591,
                longitude: 4.7246,
            },
            Waypoint {
                identifier: "EH063".to_string(),
                latitude: 52.2753,
                longitude: 4.7476,
            },
            Waypoint {
                identifier: "EH064".to_string(),
                latitude: 52.2512,
                longitude: 4.7896,
            },
            Waypoint {
                identifier: "EH065".to_string(),
                latitude: 52.2769,
                longitude: 4.7055,
            },
            Waypoint {
                identifier: "EH066".to_string(),
                latitude: 52.2738,
                longitude: 4.6734,
            },
            Waypoint {
                identifier: "EH067".to_string(),
                latitude: 52.3181,
                longitude: 4.6400,
            },
            Waypoint {
                identifier: "EH070".to_string(),
                latitude: 52.3665,
                longitude: 4.8501,
            },
            Waypoint {
                identifier: "EH073".to_string(),
                latitude: 52.2142,
                longitude: 4.8077,
            },
            Waypoint {
                identifier: "EH080".to_string(),
                latitude: 52.2914,
                longitude: 4.7364,
            },
            Waypoint {
                identifier: "EH081".to_string(),
                latitude: 52.4236,
                longitude: 4.8457,
            },
            Waypoint {
                identifier: "EH082".to_string(),
                latitude: 52.4241,
                longitude: 4.9888,
            },
            Waypoint {
                identifier: "EH084".to_string(),
                latitude: 52.3952,
                longitude: 4.7152,
            },
            Waypoint {
                identifier: "EH085".to_string(),
                latitude: 52.2982,
                longitude: 4.6836,
            },
            Waypoint {
                identifier: "EH086".to_string(),
                latitude: 52.3043,
                longitude: 4.6861,
            },
            Waypoint {
                identifier: "EH087".to_string(),
                latitude: 52.4799,
                longitude: 4.7722,
            },
            Waypoint {
                identifier: "EH088".to_string(),
                latitude: 52.4874,
                longitude: 4.9153,
            },
            Waypoint {
                identifier: "EH090".to_string(),
                latitude: 52.4672,
                longitude: 4.5344,
            },
            Waypoint {
                identifier: "EH091".to_string(),
                latitude: 52.4228,
                longitude: 4.4890,
            },
            Waypoint {
                identifier: "EH094".to_string(),
                latitude: 52.4661,
                longitude: 4.6613,
            },
            Waypoint {
                identifier: "EH400".to_string(),
                latitude: 52.9299,
                longitude: 4.7878,
            },
            Waypoint {
                identifier: "EH401".to_string(),
                latitude: 52.9445,
                longitude: 4.8585,
            },
            Waypoint {
                identifier: "EH402".to_string(),
                latitude: 52.9855,
                longitude: 4.7476,
            },
            Waypoint {
                identifier: "EH403".to_string(),
                latitude: 52.9888,
                longitude: 4.7160,
            },
            Waypoint {
                identifier: "EH404".to_string(),
                latitude: 52.9940,
                longitude: 4.6677,
            },
            Waypoint {
                identifier: "EH405".to_string(),
                latitude: 53.0306,
                longitude: 4.7372,
            },
            Waypoint {
                identifier: "EH406".to_string(),
                latitude: 52.9026,
                longitude: 4.6835,
            },
            Waypoint {
                identifier: "EH407".to_string(),
                latitude: 52.9285,
                longitude: 4.6913,
            },
            Waypoint {
                identifier: "EH408".to_string(),
                latitude: 52.9325,
                longitude: 4.8288,
            },
            Waypoint {
                identifier: "EH409".to_string(),
                latitude: 52.5907,
                longitude: 5.0176,
            },
            Waypoint {
                identifier: "EH410".to_string(),
                latitude: 52.4681,
                longitude: 5.0247,
            },
            Waypoint {
                identifier: "EH600".to_string(),
                latitude: 52.5954,
                longitude: 5.1802,
            },
            Waypoint {
                identifier: "EH601".to_string(),
                latitude: 52.6882,
                longitude: 4.8491,
            },
            Waypoint {
                identifier: "EH602".to_string(),
                latitude: 52.6883,
                longitude: 4.6801,
            },
            Waypoint {
                identifier: "EH603".to_string(),
                latitude: 52.6881,
                longitude: 4.5134,
            },
            Waypoint {
                identifier: "EH604".to_string(),
                latitude: 52.5894,
                longitude: 4.8373,
            },
            Waypoint {
                identifier: "EH607".to_string(),
                latitude: 52.5839,
                longitude: 4.6469,
            },
            Waypoint {
                identifier: "EH608".to_string(),
                latitude: 52.5394,
                longitude: 4.7281,
            },
            Waypoint {
                identifier: "EH609".to_string(),
                latitude: 52.2346,
                longitude: 4.5957,
            },
            Waypoint {
                identifier: "EH611".to_string(),
                latitude: 52.3315,
                longitude: 4.6829,
            },
            Waypoint {
                identifier: "EH614".to_string(),
                latitude: 52.2078,
                longitude: 4.5265,
            },
            Waypoint {
                identifier: "EH616".to_string(),
                latitude: 52.2537,
                longitude: 4.6454,
            },
            Waypoint {
                identifier: "EH621".to_string(),
                latitude: 52.4627,
                longitude: 4.7211,
            },
            Waypoint {
                identifier: "EH622".to_string(),
                latitude: 52.4267,
                longitude: 4.7178,
            },
            Waypoint {
                identifier: "EH624".to_string(),
                latitude: 52.3527,
                longitude: 4.5487,
            },
            Waypoint {
                identifier: "EH625".to_string(),
                latitude: 52.4860,
                longitude: 4.7543,
            },
            Waypoint {
                identifier: "EH626".to_string(),
                latitude: 52.3979,
                longitude: 4.7459,
            },
            Waypoint {
                identifier: "EH630".to_string(),
                latitude: 52.4338,
                longitude: 4.7495,
            },
            Waypoint {
                identifier: "EH632".to_string(),
                latitude: 52.2034,
                longitude: 4.7283,
            },
            Waypoint {
                identifier: "EH633".to_string(),
                latitude: 52.2394,
                longitude: 4.7316,
            },
            Waypoint {
                identifier: "EH635".to_string(),
                latitude: 52.2244,
                longitude: 4.7712,
            },
            Waypoint {
                identifier: "EH637".to_string(),
                latitude: 52.3734,
                longitude: 4.7850,
            },
            Waypoint {
                identifier: "EH638".to_string(),
                latitude: 52.2445,
                longitude: 4.7321,
            },
            Waypoint {
                identifier: "EH639".to_string(),
                latitude: 52.3239,
                longitude: 4.9640,
            },
            Waypoint {
                identifier: "EH640".to_string(),
                latitude: 52.3220,
                longitude: 4.9054,
            },
            Waypoint {
                identifier: "EH641".to_string(),
                latitude: 52.3544,
                longitude: 4.4374,
            },
            Waypoint {
                identifier: "EH642".to_string(),
                latitude: 52.3076,
                longitude: 4.5810,
            },
            Waypoint {
                identifier: "EH643".to_string(),
                latitude: 52.3031,
                longitude: 4.4999,
            },
            Waypoint {
                identifier: "EH644".to_string(),
                latitude: 52.5197,
                longitude: 4.6280,
            },
            Waypoint {
                identifier: "EH645".to_string(),
                latitude: 52.5125,
                longitude: 4.7281,
            },
            Waypoint {
                identifier: "EH646".to_string(),
                latitude: 52.4627,
                longitude: 4.7219,
            },
            Waypoint {
                identifier: "EH647".to_string(),
                latitude: 52.3824,
                longitude: 4.7119,
            },
            Waypoint {
                identifier: "EH648".to_string(),
                latitude: 52.5310,
                longitude: 4.9892,
            },
            Waypoint {
                identifier: "EH649".to_string(),
                latitude: 52.4678,
                longitude: 5.0242,
            },
            Waypoint {
                identifier: "EH650".to_string(),
                latitude: 52.4303,
                longitude: 4.9702,
            },
            Waypoint {
                identifier: "EH651".to_string(),
                latitude: 52.3640,
                longitude: 4.8748,
            },
            Waypoint {
                identifier: "EH652".to_string(),
                latitude: 52.3290,
                longitude: 4.8245,
            },
            Waypoint {
                identifier: "EH654".to_string(),
                latitude: 52.3284,
                longitude: 4.8506,
            },
            Waypoint {
                identifier: "EH655".to_string(),
                latitude: 52.3306,
                longitude: 4.9337,
            },
            Waypoint {
                identifier: "EH656".to_string(),
                latitude: 52.3273,
                longitude: 4.9636,
            },
            Waypoint {
                identifier: "EH658".to_string(),
                latitude: 52.2045,
                longitude: 4.7135,
            },
            Waypoint {
                identifier: "EH660".to_string(),
                latitude: 52.2817,
                longitude: 4.7355,
            },
            Waypoint {
                identifier: "EH661".to_string(),
                latitude: 52.3911,
                longitude: 4.9137,
            },
            Waypoint {
                identifier: "EH662".to_string(),
                latitude: 52.3146,
                longitude: 4.7084,
            },
            Waypoint {
                identifier: "GIKOV".to_string(),
                latitude: 53.1097,
                longitude: 4.5368,
            },
            Waypoint {
                identifier: "GOLOR".to_string(),
                latitude: 53.2178,
                longitude: 4.4137,
            },
            Waypoint {
                identifier: "LANSU".to_string(),
                latitude: 52.4140,
                longitude: 4.9467,
            },
            Waypoint {
                identifier: "NAKON".to_string(),
                latitude: 52.7794,
                longitude: 4.4246,
            },
            Waypoint {
                identifier: "NARIX".to_string(),
                latitude: 52.6527,
                longitude: 4.9761,
            },
            Waypoint {
                identifier: "NEXAR".to_string(),
                latitude: 52.7824,
                longitude: 4.7202,
            },
            Waypoint {
                identifier: "NIDOP".to_string(),
                latitude: 52.7593,
                longitude: 4.8838,
            },
            Waypoint {
                identifier: "NIRSI".to_string(),
                latitude: 52.5839,
                longitude: 4.5134,
            },
            Waypoint {
                identifier: "NOPSU".to_string(),
                latitude: 52.5869,
                longitude: 4.9508,
            },
            Waypoint {
                identifier: "OMORU".to_string(),
                latitude: 52.3529,
                longitude: 5.2723,
            },
            Waypoint {
                identifier: "PEROR".to_string(),
                latitude: 53.0351,
                longitude: 5.0296,
            },
            Waypoint {
                identifier: "PEVOS".to_string(),
                latitude: 52.4932,
                longitude: 4.7239,
            },
            Waypoint {
                identifier: "TORGA".to_string(),
                latitude: 52.5986,
                longitude: 5.2032,
            },
            Waypoint {
                identifier: "ULPAT".to_string(),
                latitude: 52.4700,
                longitude: 4.5950,
            },
        ];

        Ok(Response::new(WaypointsResponse { waypoints }))
    }
}
