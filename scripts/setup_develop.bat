@echo off
setlocal

REM exit to root
cd ..

REM Check if git is installed
git --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Git is not installed. Please install Git and try again.
    exit /b 1
)

REM Create the secret directory and client_secrets.json file
if not exist secret\oauth (
    mkdir secret\oauth
)
(
echo {
echo     "installed":{
echo         "client_id":"ENTER SECRETS",
echo         "project_id":"ioshcloudloader",
echo         "auth_uri":"https://accounts.google.com/o/oauth2/auth",
echo         "token_uri":"https://oauth2.googleapis.com/token",
echo         "auth_provider_x509_cert_url":"https://www.googleapis.com/oauth2/v1/certs",
echo         "client_secret":"ENTER SECRETS",
echo         "redirect_uris":["http://127.0.0.1:3001/callback"]
echo     }
echo }
) > secret\oauth\client_secrets.json

REM Create a virtual environment for pre-commit
python -m venv venv
if %errorlevel% neq 0 (
    echo Python is not installed or there was an error creating the virtual environment. Please ensure Python is installed and try again.
    exit /b 1
)

REM Activate the virtual environment
call venv\Scripts\activate

REM Install pre-commit
pip install pre-commit
if %errorlevel% neq 0 (
    echo There was an error installing pre-commit. Please check your Python and pip installations and try again.
    exit /b 1
)

REM Install the pre-commit hooks
pre-commit install
if %errorlevel% neq 0 (
    echo There was an error installing the pre-commit hooks. Please try again.
    exit /b 1
)

REM Build all crates of Rust
cargo build

echo Setup complete. You can now start working on the project.

endlocal
