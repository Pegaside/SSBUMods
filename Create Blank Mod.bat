@echo off
setlocal enabledelayedexpansion

set "REPO_URL=https://github.com/LilyLavender/lily-srs-template"
set "BASE_NAME=new-mod"

set /a x=1

:findname
if exist "%BASE_NAME%-%x%" (
    set /a x+=1
    goto findname
)

git clone "%REPO_URL%" "%BASE_NAME%-%x%"
rmdir /s /q "%BASE_NAME%-%x%\.git"

(
echo @echo off
echo cargo skyline build --release
echo if errorlevel 1 goto end
echo.
echo copy /y "target\aarch64-skyline-switch\release\libsmashline_test.nro" "%%~dp0plugin.nro"
echo :end
echo pause
) > "%BASE_NAME%-%x%\Build Mod.bat"