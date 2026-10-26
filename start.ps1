$ErrorActionPreference = 'Stop'
$utf8 = [System.Text.UTF8Encoding]::new($false)
[Console]::InputEncoding = $utf8
[Console]::OutputEncoding = $utf8
$OutputEncoding = $utf8

try {
    $repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
    Set-Location -LiteralPath $repoRoot

    $targetDirectory = 'C:\vt'
    $agent = Join-Path $targetDirectory 'debug\vivy.exe'
    $species = Join-Path $targetDirectory 'debug\coding-agent.exe'
    if (-not (Test-Path -LiteralPath $agent -PathType Leaf) -or
        -not (Test-Path -LiteralPath $species -PathType Leaf)) {
        throw "Build the coding agent first. From this folder run: `$env:CARGO_TARGET_DIR = 'C:\vt'; cargo run -p vivy -- build recipes/coding-agent.yaml"
    }

    $env:CARGO_TARGET_DIR = $targetDirectory
    $desktop = [Environment]::GetFolderPath('Desktop')
    $credentialFile = @(
        (Join-Path $desktop 'keys.txt'),
        (Join-Path $desktop 'key.txt')
    ) | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1

    if ($credentialFile) {
        $values = @([System.IO.File]::ReadAllLines($credentialFile, [System.Text.Encoding]::UTF8) |
            ForEach-Object { $_.Trim() } |
            Where-Object { $_.Length -gt 0 })
        if ($values.Count -ne 3) {
            throw 'The Desktop key file must have three non-empty lines: gateway URL, model ID, and API key.'
        }

        $endpoint = $null
        if (-not [Uri]::TryCreate($values[0], [UriKind]::Absolute, [ref]$endpoint) -or
            $endpoint.Scheme -notin @('https', 'http')) {
            throw 'The gateway URL in the Desktop key file is invalid.'
        }
        if ($endpoint.Scheme -eq 'http' -and
            $endpoint.Host -notin @('localhost', '127.0.0.1', '::1')) {
            throw 'Use HTTPS for a non-local gateway so the API key is not sent in clear text.'
        }

        $rigHome = Join-Path $repoRoot '.workspace\launch-home'
        $env:RIG_HOME = $rigHome
        $env:VIVY_BOOTSTRAP_GATEWAY_URL = $values[0]
        $env:VIVY_BOOTSTRAP_GATEWAY_MODEL = $values[1]
        $env:RIG_SECRET_GATEWAY = $values[2]
        Write-Host 'Loaded the Desktop gateway hint; existing provider settings take priority and the API key stays in process memory.'
    } else {
        if (-not $env:RIG_HOME) {
            $env:RIG_HOME = Join-Path $HOME '.rig'
        }
        # An empty home is usable: /provider adds the first provider and key.
    }

    Write-Host "Starting the coding terminal in $repoRoot"
    & $agent
    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        Write-Host "The coding terminal exited with code $exitCode." -ForegroundColor Yellow
    }
} catch {
    Write-Host "Startup failed: $($_.Exception.Message)" -ForegroundColor Red
    $exitCode = 1
}

Read-Host 'Press Enter to close'
exit $exitCode
