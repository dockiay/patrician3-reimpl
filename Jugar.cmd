@echo off
rem Doble clic para jugar. La primera vez descarga el juego ya compilado de GitHub
rem (no trae Patrician III: te preguntara donde tienes instalada tu copia).
setlocal
set "HERE=%~dp0"
powershell -NoProfile -ExecutionPolicy Bypass -File "%HERE%distribucion\Descargar.ps1" "%HERE%jugar"
if errorlevel 1 (echo. & pause)
