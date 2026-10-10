@echo off
setlocal enabledelayedexpansion
title reaperKev - Instalador y Runner de 1-Clic

echo ========================================================
echo         reaperKev: Control por Voz para REAPER
echo ========================================================
echo.

:: 1. Compilar workspace
echo [*] Compilando componentes de reaperKev...
cargo build
if %ERRORLEVEL% neq 0 (
    echo [!] Error durante la compilacion de Cargo.
    pause
    exit /b %ERRORLEVEL%
)

:: 2. Localizar carpeta de configuracion de REAPER (UserPlugins)
set "REAPER_DIR="

if exist "%APPDATA%\REAPER\reaper.exe" (
    set "REAPER_DIR=%APPDATA%\REAPER"
) else if exist "%APPDATA%\REAPER" (
    set "REAPER_DIR=%APPDATA%\REAPER"
) else if exist "C:\Program Files\REAPER (x64)\Plugins" (
    set "REAPER_DIR=C:\Program Files\REAPER (x64)"
) else if exist "D:\REAPER" (
    set "REAPER_DIR=D:\REAPER"
)

if "%REAPER_DIR%"=="" (
    echo [?] No se detecto instalacion estandar de REAPER en %%APPDATA%%\REAPER.
    echo [*] Creando directorio predeterminado en: %APPDATA%\REAPER\UserPlugins
    set "REAPER_DIR=%APPDATA%\REAPER"
)

set "USER_PLUGINS=%REAPER_DIR%\UserPlugins"
if not exist "%USER_PLUGINS%" (
    echo [*] Creando carpeta UserPlugins en %USER_PLUGINS%...
    mkdir "%USER_PLUGINS%"
)

:: 3. Copiar la extension nativa reaper_kev.dll
echo [*] Instalando reaper_kev.dll en: %USER_PLUGINS%\reaper_kev.dll
copy /y "target\debug\reaper_kev.dll" "%USER_PLUGINS%\reaper_kev.dll" >nul
if %ERRORLEVEL% neq 0 (
    echo [!] Advertencia: No se pudo copiar reaper_kev.dll. Si REAPER esta abierto, cierralo y vuelve a ejecutar este instalador.
) else (
    echo [OK] reaper_kev.dll instalada exitosamente en REAPER UserPlugins.
)

echo.
echo ========================================================
echo [*] Despliegue completado con exito.
echo.
echo Instrucciones de uso:
echo   1. Abre REAPER (la extension reaper_kev.dll se cargara automaticamente).
echo   2. Ejecuta el daemon de audio con: target\debug\reaper-audio-daemon.exe
echo   3. Usa Alt + V o el pedal MIDI (Sustain CC64) para hablar.
echo ========================================================
echo.

choice /m "¿Deseas iniciar reaper-audio-daemon ahora mismo?"
if %ERRORLEVEL% equ 1 (
    echo [*] Iniciando reaper-audio-daemon...
    start "" "target\debug\reaper-audio-daemon.exe"
)

pause
