$ErrorActionPreference = 'Stop'

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
        New-Item -ItemType Directory -Path $rigHome -Force | Out-Null
        $baseUrlToml = ConvertTo-Json -InputObject $values[0] -Compress
        $modelToml = ConvertTo-Json -InputObject $values[1] -Compress
        $settings = "[gateway]`nbase_url = $baseUrlToml`nmodel = $modelToml`n"
        $encoding = [System.Text.UTF8Encoding]::new($false)
        [System.IO.File]::WriteAllText((Join-Path $rigHome 'settings.toml'), $settings, $encoding)
        $env:RIG_HOME = $rigHome
        $env:RIG_SECRET_GATEWAY = $values[2]
        Write-Host 'Loaded gateway settings from the Desktop key file; the API key stays in process memory.'
    } else {
        if (-not $env:RIG_HOME) {
            $env:RIG_HOME = Join-Path $HOME '.rig'
        }
        if (-not $env:RIG_SECRET_GATEWAY) {
            throw 'Set RIG_SECRET_GATEWAY or put three non-empty lines (gateway URL, model ID, API key) in Desktop\keys.txt.'
        }
        if (-not (Test-Path -LiteralPath (Join-Path $env:RIG_HOME 'settings.toml') -PathType Leaf)) {
            throw 'Gateway settings.toml was not found under RIG_HOME.'
        }
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
