@echo off

@(
    ECHO ================================================================
    ECHO ================================================================
    ECHO ====================== NEW SERVER SESSION ======================
    ECHO ================= %date% %time% ==================
    ECHO ================================================================
    ECHO ================================================================

)>>server_error.log

.\SpaceFrontiers.exe server --logger_enabled 2>&1 2>>server_error.log