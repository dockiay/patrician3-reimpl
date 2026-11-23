# Patrician III en streaming: el juego corre en ESTE PC (p3-stream.exe, sin ventana) y el navegador (iPhone Safari incluido) solo
# recibe video y sonido y envia raton/teclado/toques. No se descarga nada del juego. URL https por tunel Cloudflare "quick".
# La contrasena es obligatoria y queda en clave.txt (la creas una vez; borrala para generar otra). Uso privado: no repartas URL ni clave.
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $here
# Si hay un p3-stream-nuevo.zip (version nueva del servidor) se descomprime encima del .exe y se borra.
$nuevo = Join-Path $here 'p3-stream-nuevo.zip'
if (Test-Path $nuevo) { Get-Process p3-stream -ErrorAction SilentlyContinue | Stop-Process -Force; Expand-Archive -Force $nuevo $here; Remove-Item $nuevo }
# Carpeta con Patrician3.exe, el .cpr y Musik\ (solo se LEE). Las partidas se guardan aparte, en .\partidas.
if (-not $env:P3_GAME_DIR) { $env:P3_GAME_DIR = 'D:\AI\Patrician3\games\iso-english\app' }
$exe = Join-Path $env:P3_GAME_DIR 'Patrician3.exe'
$cpr = Join-Path $env:P3_GAME_DIR 'p2arch0_eng.cpr'
if (-not (Test-Path $exe)) { throw "No encuentro $exe (define P3_GAME_DIR)" }
if (-not (Test-Path $cpr)) { throw "No encuentro $cpr" }
New-Item -ItemType Directory -Force (Join-Path $here 'partidas'), (Join-Path $here '.tools') | Out-Null
$cf = Join-Path $here '.tools\cloudflared.exe'
if (-not (Test-Path $cf)) {
  Write-Host 'Descargando cloudflared.exe (una vez) de github.com/cloudflare/cloudflared ...'
  Invoke-WebRequest 'https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe' -OutFile $cf
}
$keyFile = Join-Path $here 'clave.txt'
if (-not (Test-Path $keyFile)) {
  $chars = (48..57) + (65..90) + (97..122) | % { [char]$_ }
  (-join (1..20 | % { $chars | Get-Random })) | Set-Content -NoNewline $keyFile
}
$env:P3_PASSWORD = (Get-Content $keyFile -Raw).Trim()
$port = 8090
$extra = @()
$moderna = if ($env:P3_MODERNA) { $env:P3_MODERNA } else { '1920x1080' }   # resolucion de la partida (los menus del original son 800x600); P3_MODERNA=1280x720 para menos carga
if ($moderna -ne 'no') { $extra += '--moderna', $moderna }
if ($env:P3_KBPS) { $extra += '--kbps', $env:P3_KBPS }
# Idioma (Opciones -> Language, se guarda en partidas\idioma.txt y se aplica al reiniciar el servidor): p2arch0_spa.cpr e idioma\spa.pack
# se buscan en la carpeta del juego, en partidas y en P3_IDIOMAS (por defecto D:\AI\P3-Prueba, donde los tiene p3-juego).
$idiomas = if ($env:P3_IDIOMAS) { $env:P3_IDIOMAS } else { 'D:\AI\P3-Prueba' }
if (Test-Path $idiomas) { $extra += '--idiomas', "`"$idiomas`"" }
# Primera vez: el idioma elegido en p3-juego.
$eleccion = Join-Path $here 'partidas\idioma.txt'
if (-not (Test-Path $eleccion) -and (Test-Path (Join-Path $idiomas 'idioma.txt'))) { Copy-Item (Join-Path $idiomas 'idioma.txt') $eleccion }
if ($env:P3_IDIOMA) { $extra += '--idioma', $env:P3_IDIOMA }
$log = Join-Path $here '.tools\servidor.log'
$keep = [bool]$env:P3_SEGUNDO_PLANO   # sin pausa: deja servidor y tunel corriendo y sale
$srv = Start-Process (Join-Path $here 'p3-stream.exe') -ArgumentList (@('--exe', "`"$exe`"", '--cpr', "`"$cpr`"", '--juego', "`"$here\partidas`"", '--puerto', $port) + $extra) -PassThru -WindowStyle Minimized -RedirectStandardError $log
$tlog = Join-Path $here '.tools\tunnel.log'; Remove-Item $tlog -ErrorAction SilentlyContinue
$tun = Start-Process $cf -ArgumentList 'tunnel', '--url', "http://127.0.0.1:$port", '--no-autoupdate', '--logfile', $tlog -PassThru -WindowStyle Minimized
try {
  $url = $null
  for ($i = 0; $i -lt 60 -and -not $url; $i++) {
    Start-Sleep 1
    if (Test-Path $tlog) { $m = Select-String -Path $tlog -Pattern 'https://[a-z0-9-]+\.trycloudflare\.com' | select -First 1; if ($m) { $url = $m.Matches[0].Value } }
  }
  if (-not $url) { throw "No consegui la URL del tunel; mira $tlog" }
  Set-Content (Join-Path $here 'url.txt') $url
  Write-Host ''
  Write-Host "  URL:         $url" -ForegroundColor Green
  Write-Host "  Usuario:     (cualquiera)" -ForegroundColor Green
  Write-Host "  Contrasena:  $env:P3_PASSWORD" -ForegroundColor Green
  Write-Host ''
  Write-Host 'Abre la URL en Safari, escribe la contrasena y pulsa Jugar. La partida sigue en este PC aunque cierres el navegador.'
  Write-Host 'La URL cambia en cada arranque (la contrasena no). Pulsa Intro aqui para parar el servidor y el tunel.'
  if (-not $keep) { Read-Host | Out-Null }
} finally {
  if (-not $keep) {
    Stop-Process -Id $tun.Id -Force -ErrorAction SilentlyContinue
    Stop-Process -Id $srv.Id -Force -ErrorAction SilentlyContinue
  }
}
