# Descarga el paquete compilado de la ultima version y abre el asistente "Jugar".
param([Parameter(Mandatory = $true)][string]$Destino)
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
$repo = 'dockiay/patrician3-reimpl'
try {
    $launcher = Get-ChildItem -LiteralPath $Destino -Recurse -Filter 'Jugar.cmd' -ErrorAction SilentlyContinue |
        Where-Object { Test-Path -LiteralPath (Join-Path $_.DirectoryName 'p3-juego.exe') } | Select-Object -First 1
    if (-not $launcher) {
        Write-Host 'Descargando el juego compilado de GitHub (solo la primera vez)...'
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -Headers @{ 'User-Agent' = 'p3-jugar' }
        $asset = $release.assets | Where-Object { $_.name -like '*-windows.zip' } | Select-Object -First 1
        if (-not $asset) { throw "La version $($release.tag_name) todavia no tiene el paquete para Windows. Prueba mas tarde o usa Compilar." }
        $zip = Join-Path ([IO.Path]::GetTempPath()) $asset.name
        Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $zip -UseBasicParsing
        Write-Host 'Descomprimiendo...'
        New-Item -ItemType Directory -Force -Path $Destino | Out-Null
        Expand-Archive -LiteralPath $zip -DestinationPath $Destino -Force
        Remove-Item -LiteralPath $zip -ErrorAction SilentlyContinue
        $launcher = Get-ChildItem -LiteralPath $Destino -Recurse -Filter 'Jugar.cmd' |
            Where-Object { Test-Path -LiteralPath (Join-Path $_.DirectoryName 'p3-juego.exe') } | Select-Object -First 1
        if (-not $launcher) { throw 'El paquete descargado no trae el juego.' }
    }
    Start-Process -FilePath $launcher.FullName -WorkingDirectory $launcher.DirectoryName
} catch {
    Write-Host ''
    Write-Host "No se ha podido preparar el juego: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Puedes bajarlo a mano de https://github.com/$repo/releases/latest (el ZIP que acaba en -windows.zip)."
    exit 1
}
