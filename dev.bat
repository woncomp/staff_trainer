@echo off
set CMD=%1
@echo on

cargo %CMD% --example dev -F dynamic_linking