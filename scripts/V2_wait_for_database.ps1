# Navigate to the docker_server directory
cd '..\..\docker_server\'

# Bring up Docker services
docker-compose up -d

# Function to check PostgreSQL readiness
function Test-PostgresReadiness {
    $maxAttempts = 30
    $attempt = 0

    do {
        Write-Host "Waiting for PostgreSQL to spin up..."
        Start-Sleep -Seconds 2
        $attempt++
    } until ($attempt -ge $maxAttempts -or (Test-Connection -ComputerName localhost -Port 5432 -Quiet))

    if ($attempt -ge $maxAttempts) {
        Write-Host "Timed out waiting for PostgreSQL to be ready."
        exit 1
    }

    Write-Host "PostgreSQL is now running"
}

# Call the function to check PostgreSQL readiness
Test-PostgresReadiness

# Shut down Docker services
docker-compose down
