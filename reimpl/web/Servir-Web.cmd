@echo off
rem WEB-6: compila (si falta dist\) y sirve el juego web en http://localhost:8765/
cd /d "%~dp0"
if not exist dist\build.json python build_web.py %*
start "" http://localhost:8765/
python serve.py
