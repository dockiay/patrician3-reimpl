@echo off
rem Vuelve a preguntar donde esta instalado tu juego.
start "" powershell -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File "%~dp0Jugar.ps1" -Configurar
