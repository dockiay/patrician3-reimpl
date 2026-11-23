@echo off
rem Abre Patrician III. La primera vez pregunta donde esta instalado tu juego.
start "" powershell -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File "%~dp0Jugar.ps1"
