@echo off
set CMD=%1
@echo on

cargo apk %CMD% -p staff_trainer --lib