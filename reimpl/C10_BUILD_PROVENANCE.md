# C10: procedencia del binario de comparación

`oracle/c10_build_provenance.py` construye el exportador con Cargo, obtiene los
artefactos locales efectivamente usados a partir de sus mensajes JSON y lee
los dep-info `.d` de rustc. El manifiesto registra cada entrada declarada por
esos dep-info, con sus hashes, además de los `Cargo.toml` de paquetes usados,
el `Cargo.toml` del workspace, `Cargo.lock`, los artefactos y el ejecutable. También registra las versiones detalladas de
Cargo/rustc y las variables de opciones de compilación reconocidas, incluido
`CARGO_PROFILE_DEV_OPT_LEVEL`. Deben coincidir al verificar el sello.
No afirma que los demás archivos Rust del repositorio se hayan compilado.

El wrapper exige un `CARGO_TARGET_DIR` nuevo o vacío, verifica dos builds
consecutivos, ejecuta el exportador y vuelve a
comprobar los hashes. El aceptador exige el grafo y, tras validarlo, pide de
nuevo a Cargo el conjunto de artefactos con el mismo target y lo comprueba otra
vez. Rechaza código, metadatos, archivos dep-info o artefactos modificados y
la falta de una biblioteca local. La prueba real de carga inicial registra 9
artefactos, 396 entradas de dep-info y 10 entradas Cargo en esta versión; esos
números no son constantes y no se usan como criterio de aceptación.

Desde la carpeta final de trabajo y con las herramientas del proyecto en PATH:

```powershell
. D:/AI/Patrician3/scripts/Enter-P3Environment.ps1
$env:CARGO_TARGET_DIR='D:/AI/Patrician3/coordination-runtime/target-c10-final'
$env:CARGO_BUILD_JOBS='2'
$env:CARGO_PROFILE_DEV_OPT_LEVEL='1'
python reimpl/oracle/c10_build_provenance.py --save D:/AI/Patrician3/games/visibility/save/Ein/AUTO.pat --archive D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --output D:/AI/Patrician3/coordination-runtime/c10-native-final --days 30
```

La carpeta de salida y el target deben estar vacíos al empezar; use nombres
nuevos para cada ejecución, sin borrar resultados anteriores. Después se ejecuta
`accept_c10_real_run.py` con sus argumentos habituales; usará `compiled_graph`
y volverá a invocar Cargo con el target guardado en el manifiesto. Conservar
tanto la carpeta de salida como ese target: si se borran los dep-info o los
artefactos, se rechaza el sello. Las rutas de artefactos son absolutas; al
mover la carpeta o integrar en otro worktree hay que reconstruir y repetir la
exportación. Los manifiestos anteriores sin `compiled_graph` ya no se aceptan.

La lista de variables reconocidas no demuestra que todo el entorno externo
del compilador sea hermético. Las dependencias de registro quedan vinculadas por `Cargo.lock` y los hashes
de artefactos locales, pero no se afirma un cotejo byte a byte de todos sus
fuentes. Se rechazan explícitamente artefactos locales de tipos distintos a
biblioteca o al ejemplo C10, hasta incorporar su dep-info. La comprobación no
sustituye un build hermético ni una firma contra manipulación deliberada; no
se deben modificar fuentes durante la compilación o la simulación.
