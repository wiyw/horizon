//! Thin executable for the shared server CLI and supervisor library.
fn main() -> std::process::ExitCode {
    horizon_platform::hooks::tailcat::set_assets(horizon_addresses::TAILCAT_HELPER_ASSETS);
    horizon_platform::run_main(horizon_server::main)
}
