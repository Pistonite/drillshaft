@echo off
for /f "delims=" %%i in ('zoxide query -i %1') do cd /d "%%i"
