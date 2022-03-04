// --- Tests ---
#[cfg(test)]
mod tests;

// --- NetCup CCP API ---
mod netcup;
pub use netcup::*;

// --- Lib Root definitions ---
type Error = String;

#[allow(dead_code)]
type NetcupClient = Client;
#[allow(dead_code)]
type CCPClient = Client;
