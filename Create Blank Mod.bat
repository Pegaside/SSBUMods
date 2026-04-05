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

(
echo @echo off
echo cargo skyline build --release
pause
) > "%BASE_NAME%-%x%\Build Mod.bat"