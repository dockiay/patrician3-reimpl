@echo off
rem Compila el juego desde el codigo y deja la carpeta "jugar" lista, sin escribir ordenes.
rem La mayoria de la gente no necesita esto: en la pagina de versiones de GitHub hay un ZIP ya compilado.
setlocal
set "HERE=%~dp0"
where cargo >nul 2>nul
if errorlevel 1 (
  echo Falta Rust, el programa con el que se compila.
  echo Instalalo desde https://rustup.rs ^(siguiente, siguiente^) y vuelve a abrir este archivo.
  start "" https://rustup.rs
  pause & exit /b 1
)
echo Compilando. La primera vez puede tardar bastante; no cierres esta ventana...
pushd "%HERE%reimpl"
cargo build --release --locked -p p3-client --bin p3-juego
if errorlevel 1 (popd & echo. & echo La compilacion ha fallado. Mira el error de arriba. & pause & exit /b 1)
popd
if not exist "%HERE%jugar" mkdir "%HERE%jugar"
copy /y "%HERE%reimpl\target\release\p3-juego.exe" "%HERE%jugar\" >nul
copy /y "%HERE%distribucion\*" "%HERE%jugar\" >nul
echo.
echo Listo. Se abre la carpeta "jugar": doble clic en "Jugar".
start "" "%HERE%jugar"
