@echo off
REM Local headless plugin host test runner for Windows
REM Usage: scripts\test-headless.bat [plugin_path] [--junit-output path]

setlocal enabledelayedexpansion

set "SCRIPT_DIR=%~dp0"
set "PROJECT_ROOT=%SCRIPT_DIR%.."

REM Default plugin path
set "DEFAULT_PLUGIN_PATH=target\release\Periodic Instruments.vst3"
set "PLUGIN_PATH=%DEFAULT_PLUGIN_PATH%"
set "JUNIT_OUTPUT=test-results-local.xml"

REM Parse arguments
:parse_args
if "%~1"=="" goto :args_done
if "%~1"=="--junit-output" (
    set "JUNIT_OUTPUT=%~2"
    shift
    shift
    goto :parse_args
)
if "%~1"=="--help" if "%~1"=="-h" (
    echo Usage: %~nx0 [plugin_path] [--junit-output path]
    echo   plugin_path: Path to VST3 bundle (default: %DEFAULT_PLUGIN_PATH%)
    echo   --junit-output: Path for JUnit XML output (default: test-results-local.xml)
    exit /b 0
)
if "%~1"=="-*" (
    echo Unknown option: %~1
    exit /b 1
)
set "PLUGIN_PATH=%~1"
shift
goto :parse_args

:args_done

echo ================================================================
echo PERIODIC INSTRUMENTS - LOCAL HEADLESS PLUGIN HOST TEST
echo ================================================================
echo Project root: %PROJECT_ROOT%
echo Plugin path:  %PLUGIN_PATH%
echo JUnit output: %JUNIT_OUTPUT%
echo ================================================================

cd /d "%PROJECT_ROOT%"

REM Check if plugin exists
if not exist "%PLUGIN_PATH%" (
    echo ERROR: Plugin not found at: %PLUGIN_PATH%
    echo Build it first with: cargo build --release
    exit /b 1
)

REM Get git info if available
git rev-parse --git-dir >nul 2>&1
if not errorlevel 1 (
    for /f "delims=" %%a in ('git rev-parse --short HEAD') do set "COMMIT=%%a"
    for /f "delims=" %%a in ('git rev-parse --abbrev-ref HEAD') do set "BRANCH=%%a"
    echo Git commit:  %COMMIT% (%BRANCH%)
)

echo Running headless tests...
echo.

REM Run the test binary
cargo test --features headless --test headless_host -- --test-threads=1 --nocapture --junit-output "%JUNIT_OUTPUT%" "%PLUGIN_PATH%"

set "TEST_EXIT_CODE=%ERRORLEVEL%"

echo.
echo ================================================================
if %TEST_EXIT_CODE% equ 0 (
    echo ^<^< ALL TESTS PASSED ^>^>
) else (
    echo ^<^< SOME TESTS FAILED (exit code: %TEST_EXIT_CODE%) ^>^>
)
echo JUnit XML written to: %JUNIT_OUTPUT%
echo ================================================================

exit /b %TEST_EXIT_CODE%