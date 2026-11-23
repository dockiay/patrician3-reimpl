@echo off
rem Sirve Patrician III web en tu PC y lo hace accesible SOLO desde tus dispositivos Tailscale (https privado). Uso privado: no publicar.
rem Variable opcional: P3_GAME_DIR = carpeta con Patrician3.exe y el .cpr (por defecto la copia en ingles del repo).
cd /d "%~dp0"
if "%P3_GAME_DIR%"=="" set "P3_GAME_DIR=D:\AI\Patrician3\games\lab-english"
rem Idioma: p2arch0_spa.cpr e idioma\spa.pack tambien desde P3_IDIOMAS (por defecto D:\AI\P3-Prueba).
if "%P3_IDIOMAS%"=="" if exist "D:\AI\P3-Prueba" set "P3_IDIOMAS=D:\AI\P3-Prueba"
if not exist "%P3_GAME_DIR%\Patrician3.exe" (echo No encuentro Patrician3.exe en %P3_GAME_DIR% ^(define P3_GAME_DIR^) & pause & exit /b 1)
if not exist www\p3_web.wasm (echo Compilando p3_web.wasm... & bash build.sh || (pause & exit /b 1))
set "TS=tailscale"
where tailscale >nul 2>nul || if exist "%ProgramFiles%\Tailscale\tailscale.exe" set "TS=%ProgramFiles%\Tailscale\tailscale.exe"
"%TS%" version >nul 2>nul
if errorlevel 1 (
  echo Tailscale no esta instalado: solo se podra abrir en este PC ^(http://localhost:8080^). Mira README-MOVIL.md.
) else (
  "%TS%" serve --bg --https=443 http://127.0.0.1:8080
  echo.
  echo Abre en el movil ^(con Tailscale activado^) la direccion https de arriba.
)
start "" http://localhost:8080
python serve.py 8080
