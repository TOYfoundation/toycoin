# Local quality gate for Windows (PowerShell): same checks as CI.
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")
function Run($cmd) {
    Write-Host "==> $cmd"
    Invoke-Expression $cmd
    if ($LASTEXITCODE -ne 0) { throw "failed: $cmd" }
}
Run "cargo fmt --all -- --check"
Run "cargo clippy --workspace --all-targets -- -D warnings"
Run "cargo test --workspace"
Write-Host "OK: all checks passed"
