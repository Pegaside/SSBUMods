@echo off
setlocal enabledelayedexpansion

yamlist disasm motion_list.bin --label Labels.txt --out motion_list.yaml
