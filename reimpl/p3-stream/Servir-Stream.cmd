@echo off
rem Juego en streaming desde este PC: URL https publica (tunel Cloudflare) protegida con contrasena. Ver Servir-Stream.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0Servir-Stream.ps1"
if errorlevel 1 pause
