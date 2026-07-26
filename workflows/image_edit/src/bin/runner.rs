use lightflow::runner::{read_request_from_stdin, write_response_to_stdout};
use std::error::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("FLUX image-edit runner: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let request = read_request_from_stdin()?;
    request.validate_for(
        lightflow_flux_image_edit::WORKFLOW_ID,
        lightflow_flux_image_edit::WORKFLOW_VERSION,
    )?;
    write_response_to_stdout(&lightflow_flux_image_edit::execute_with_models(
        &request.inputs,
        &request.models,
    )?)?;
    Ok(())
}
