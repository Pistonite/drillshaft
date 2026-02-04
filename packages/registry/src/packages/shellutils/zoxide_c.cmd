@echo off
for /f "delims=" %%i in ('zoxide query %1') do cd /d "%%i"
