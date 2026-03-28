#[cfg(feature = "full")]
pub(crate) mod auth;
mod coordinates;
mod errors;
pub(crate) mod handlers;
mod helpers;
mod mapping;
pub(crate) mod router;
pub(crate) mod startup;
pub(crate) mod types;

#[cfg(test)]
mod tests;
#[cfg(all(test, feature = "full"))]
mod tests_authz;
#[cfg(test)]
mod tests_error_contract;
#[cfg(all(test, feature = "full"))]
mod tests_limits;
#[cfg(test)]
mod tests_roles;
#[cfg(test)]
mod tests_stage1_flow;

pub(crate) use startup::run;
