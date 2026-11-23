@echo off
rem URL https publica con contrasena (tunel Cloudflare). Ver Servir-Publico.ps1 y README-MOVIL.md
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0Servir-Publico.ps1"
if errorlevel 1 pause
