use crate::FluxError;
use std::io::Read;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const STDOUT_LIMIT: usize = 1024 * 1024;
const STDERR_LIMIT: usize = 256 * 1024;
const TIMEOUT: Duration = Duration::from_secs(5 * 60);

#[derive(Debug)]
pub(super) struct ProcessOutput {
    pub(super) status: ExitStatus,
    pub(super) stderr: Vec<u8>,
}

pub(super) fn run(command: &mut Command, label: &str) -> Result<ProcessOutput, FluxError> {
    run_with_limits(command, label, TIMEOUT, STDOUT_LIMIT, STDERR_LIMIT)
}

fn run_with_limits(
    command: &mut Command,
    label: &str,
    timeout: Duration,
    stdout_limit: usize,
    stderr_limit: usize,
) -> Result<ProcessOutput, FluxError> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command.spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| FluxError::Backend(format!("{label} stdout is unavailable")))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| FluxError::Backend(format!("{label} stderr is unavailable")))?;
    let overflow = Arc::new(AtomicBool::new(false));
    let stdout_reader = read_capped(stdout, stdout_limit, Arc::clone(&overflow));
    let stderr_reader = read_capped(stderr, stderr_limit, Arc::clone(&overflow));
    let started = Instant::now();
    let status = loop {
        if overflow.load(Ordering::SeqCst) {
            kill_process_group(&mut child);
            let _ = child.wait();
            break Err(FluxError::Backend(format!(
                "{label} exceeded the stdout/stderr output limit \
                 ({stdout_limit}/{stderr_limit} bytes)"
            )));
        }
        if started.elapsed() >= timeout {
            kill_process_group(&mut child);
            let _ = child.wait();
            break Err(FluxError::Backend(format!(
                "{label} timed out after {}s",
                timeout.as_secs_f64()
            )));
        }
        if let Some(status) = child.try_wait()? {
            break Ok(status);
        }
        thread::sleep(Duration::from_millis(10));
    };
    let _stdout = stdout_reader
        .join()
        .map_err(|_| FluxError::Backend(format!("{label} stdout reader panicked")))??;
    let stderr = stderr_reader
        .join()
        .map_err(|_| FluxError::Backend(format!("{label} stderr reader panicked")))??;
    Ok(ProcessOutput {
        status: status?,
        stderr,
    })
}

fn read_capped<R>(
    mut reader: R,
    limit: usize,
    overflow: Arc<AtomicBool>,
) -> thread::JoinHandle<Result<Vec<u8>, FluxError>>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut output = Vec::new();
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            let remaining = limit.saturating_sub(output.len());
            output.extend_from_slice(&buffer[..count.min(remaining)]);
            if count > remaining {
                overflow.store(true, Ordering::SeqCst);
            }
        }
        Ok(output)
    })
}

fn kill_process_group(child: &mut std::process::Child) {
    #[cfg(unix)]
    unsafe {
        if libc::kill(-(child.id() as i32), libc::SIGKILL) == 0 {
            return;
        }
    }
    let _ = child.kill();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_flooded_output() {
        let error = run_with_limits(
            Command::new("sh").args(["-c", "head -c 8192 /dev/zero"]),
            "flood fixture",
            Duration::from_secs(5),
            1024,
            1024,
        )
        .expect_err("flood");
        assert!(error.to_string().contains("output limit"));
    }

    #[test]
    fn kills_timed_out_process_group() {
        let error = run_with_limits(
            Command::new("sh").args(["-c", "sleep 5"]),
            "timeout fixture",
            Duration::from_millis(50),
            1024,
            1024,
        )
        .expect_err("timeout");
        assert!(error.to_string().contains("timed out"));
    }
}
