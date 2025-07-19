# Set the target folder path and results file
$folderPath = "./target/release"
$resultsFile = "benchmark_results.txt"

# Clear or create the results file
Set-Content -Path $resultsFile -Value ""

# Get all .exe files in the folder
$executables = Get-ChildItem -Path $folderPath -Filter *.exe -File

$total = 0

# Loop through each executable and benchmark using hyperfine
foreach ($exe in $executables) {
    $exePath = $exe.FullName
    Write-Host "Benchmarking: $($exe.Name)"

    # Run hyperfine and capture output to temp file
    hyperfine --warmup 1 --runs 5 --export-json temp_result.json "$exePath" 2>$null

    # avoid fs stuff with the json file
    sleep 0.1

    if (Test-Path "temp_result.json") {
        # Read and parse JSON result
        $json = Get-Content "temp_result.json" | ConvertFrom-Json
        $averageTime = $json.results[0].mean

        $total = $total + $averageTime

        # Format output
        $line = "{0,-30} Average time: {1:N6} seconds" -f $exe.Name, $averageTime

        # Write to console and file
        Write-Host $line
        Add-Content -Path $resultsFile -Value $line

        # Clean up
        Remove-Item "temp_result.json"
    } else {
        Write-Host "Failed to benchmark $($exe.Name)"
    }
}

Write-Host $total
Add-Content -Path $resultsFile -Value $total
