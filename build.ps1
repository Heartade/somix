# Update PATH (it's cursed on Windows)
function Update-Path {
  $Env:Path =
    [System.Environment]::GetEnvironmentVariable("Path","Machine") +
    ";" + [System.Environment]::GetEnvironmentVariable("Path","User");
}

# Install Rustup for Windows if not installed
try {
  Get-Command rustup;
  Write-Output "Rustup is already installed.";
} catch {
  Write-Output "Rustup is not installed. Installing...";
  winget install Rustlang.Rustup;
  Update-Path;
}

rustup toolchain install stable;
rustup target add wasm32-unknown-unknown;

# Install Trunk for Windows beta
try {
  Get-Command trunk;
  Write-Output "Trunk is already installed.";
} catch {
  Write-Output "Trunk is not installed. Installing...";
  cargo install --locked trunk;
  Update-Path;
}

trunk serve;
