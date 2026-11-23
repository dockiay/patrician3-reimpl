# Patrician III web: URL https publica (tunel Cloudflare "quick", sin cuenta ni instalacion) PROTEGIDA CON CONTRASENA.
# El juego no se sube a ningun hosting: el PC lo sirve, el tunel solo reenvia (el trafico pasa por Cloudflare, cifrado).
# Uso privado: el .wasm lleva el codigo del EXE traducido. No compartas la URL ni la contrasena.
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $here
if (-not $env:P3_GAME_DIR) { $env:P3_GAME_DIR = 'D:\AI\Patrician3\games\lab-english' }
# Idioma: p2arch0_spa.cpr e idioma\spa.pack se ofrecen tambien desde P3_IDIOMAS (por defecto D:\AI\P3-Prueba, donde los tiene p3-juego).
if (-not $env:P3_IDIOMAS -and (Test-Path 'D:\AI\P3-Prueba')) { $env:P3_IDIOMAS = 'D:\AI\P3-Prueba' }
if (-not (Test-Path "$env:P3_GAME_DIR\Patrician3.exe")) { throw "No encuentro Patrician3.exe en $env:P3_GAME_DIR (define P3_GAME_DIR)" }
if (-not ((Test-Path www\p3_web.wasm) -and (Test-Path www\p3-saves-worker.js))) { Write-Host 'Compilando p3_web.wasm (varios minutos)...'; bash build.sh; if ($LASTEXITCODE) { throw 'Fallo build.sh' } }
$tools = Join-Path $here '.tools'; New-Item -ItemType Directory -Force $tools | Out-Null
$cf = Join-Path $tools 'cloudflared.exe'
if (-not (Test-Path $cf)) {
  Write-Host 'Descargando cloudflared.exe (una vez) de github.com/cloudflare/cloudflared ...'
  Invoke-WebRequest 'https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe' -OutFile $cf
}
$chars = (48..57) + (65..90) + (97..122) | % { [char]$_ }
$env:P3_PASSWORD = -join (1..20 | % { $chars | Get-Random })
$port = 8081
$srv = Start-Process python -ArgumentList 'serve.py', $port -PassThru -WindowStyle Minimized
$log = Join-Path $tools 'tunnel.log'; Remove-Item $log -ErrorAction SilentlyContinue
$tun = Start-Process $cf -ArgumentList 'tunnel', '--url', "http://127.0.0.1:$port", '--no-autoupdate', '--logfile', $log -PassThru -WindowStyle Minimized
try {
  $url = $null
  for ($i = 0; $i -lt 60 -and -not $url; $i++) {
    Start-Sleep 1
    if (Test-Path $log) { $m = Select-String -Path $log -Pattern 'https://[a-z0-9-]+\.trycloudflare\.com' | select -First 1; if ($m) { $url = $m.Matches[0].Value } }
  }
  if (-not $url) { throw "No consegui la URL del tunel; mira $log" }
  Write-Host ''
  Write-Host "  URL:         $url" -ForegroundColor Green
  Write-Host "  Usuario:     (cualquiera)" -ForegroundColor Green
  Write-Host "  Contrasena:  $env:P3_PASSWORD" -ForegroundColor Green
  Write-Host ''
  Write-Host 'Abre la URL en Safari del iPhone, escribe la contrasena, pulsa "Cargar desde el PC" y luego "Jugar".'
  Write-Host 'La URL y la contrasena cambian en cada arranque. Pulsa Intro aqui para parar el servidor y el tunel.'
  Read-Host | Out-Null
} finally {
  Stop-Process -Id $tun.Id -Force -ErrorAction SilentlyContinue
  Stop-Process -Id $srv.Id -Force -ErrorAction SilentlyContinue
}
