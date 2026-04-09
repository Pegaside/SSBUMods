@echo off
cargo skyline build --release
if errorlevel 1 goto end

copy /y "target\aarch64-skyline-switch\release\libsmashline_test.nro" "%~dp0plugin.nro"
:end
pause
