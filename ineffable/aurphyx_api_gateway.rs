use warp::{Filter, Rejection, http::StatusCode};
use std::convert::Infallible;
use std::collections::HashMap;
use tokio::sync::RwLock;
use lazy_static::lazy_static;
use governor::{Quota, RateLimiter, clock::DefaultClock, state::keyed::DashMapStateStore};
use std::num::NonZeroU32;
use std::time::Duration;

// Roles & permissions map - simple example
lazy_static! {
    static ref RBAC_PERMISSIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("admin", vec!["consensus", "read", "write", "slashing"]);
        m.insert("validator", vec!["read", "write"]);
        m.insert("client", vec!["read"]);
        m
    };
}

// Mock token to role map (replace with secure DB/Cache lookup)
lazy_static! {
    static ref TOKEN_ROLE_MAP: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert("token_admin".to_string(), "admin");
        m.insert("token_validator".to_string(), "validator");
        m.insert("token_client".to_string(), "client");
        m
    };
}

// Rate limiter keyed by token (can be IP or token id)
type KeyedRateLimiter = RateLimiter<String, DashMapStateStore<String>, DefaultClock>;

// Create a global rate limiter with token key and 10 requests per second quota
lazy_static! {
    static ref RATE_LIMITER: KeyedRateLimiter =
        RateLimiter::keyed(Quota::per_second(NonZeroU32::new(10).unwrap()));
}

// Authentication+Authorization Filter
fn with_auth_and_rbac(required_perm: &'static str) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::header("authorization")
        .and_then(move |auth_header: String| {
            let required_perm = required_perm.to_string();
            async move {
                // Parse token (e.g., "Bearer token_xyz"), simplistic here
                let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
                if let Some(&role) = TOKEN_ROLE_MAP.get(token) {
                    // Check permission for role
                    if let Some(perms) = RBAC_PERMISSIONS.get(role) {
                        if perms.contains(&required_perm.as_str()) {
                            // Rate limit check
                            if RATE_LIMITER.check_key(&token).is_ok() {
                                Ok(())
                            } else {
                                Err(warp::reject::custom(RateLimitExceeded))
                            }
                        } else {
                            Err(warp::reject::custom(ForbiddenPermission))
                        }
                    } else {
                        Err(warp::reject::custom(ForbiddenPermission))
                    }
                } else {
                    Err(warp::reject::custom(Unauthorized))
                }
            }
        })
}

// Custom rejects
#[derive(Debug)]
struct Unauthorized;
#[derive(Debug)]
struct ForbiddenPermission;
#[derive(Debug)]
struct RateLimitExceeded;

impl warp::reject::Reject for Unauthorized {}
impl warp::reject::Reject for ForbiddenPermission {}
impl warp::reject::Reject for RateLimitExceeded {}

async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<Unauthorized>() {
        code = StatusCode::UNAUTHORIZED;
        message = "Unauthorized";
    } else if let Some(_) = err.find::<ForbiddenPermission>() {
        code = StatusCode::FORBIDDEN;
        message = "Forbidden";
    } else if let Some(_) = err.find::<RateLimitExceeded>() {
        code = StatusCode::TOO_MANY_REQUESTS;
        message = "Rate limit exceeded";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    Ok(warp::reply::with_status(message, code))
}

// Example protected route
fn consensus_route() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path!("consensus" / "status")
        .and(with_auth_and_rbac("read"))
        .map(|| "Consensus status: all systems go")
}

#[tokio::main]
async fn main() {
    let routes = consensus_route()
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}