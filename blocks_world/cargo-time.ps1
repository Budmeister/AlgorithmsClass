# Create an empty array to store the results
$results = @()

# Step 1: Build the project using cargo
Write-Host "Building the project using cargo..."
Measure-Command {
    cargo build
} | Select-Object TotalMilliseconds | ForEach-Object {
    Write-Host "Build time: $($_.TotalMilliseconds) milliseconds"
}

# Step 2: Loop through all files in the test_cases folder
$testCasesFolder = "./test_cases"
$testFiles = Get-ChildItem $testCasesFolder -File

foreach ($testFile in $testFiles) {
    $testFilePath = $testFile.FullName
    Write-Host "Running cargo run with input from $testFilePath..."

    # Create a temporary file for stdout redirection
    $tempFile = [System.IO.Path]::GetTempFileName()
    
    $runTime = Measure-Command {
        Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run" -RedirectStandardInput $testFilePath -RedirectStandardOutput $tempFile -Wait
    } | Select-Object TotalMilliseconds | ForEach-Object {
        Write-Host "Run time for $($testFile.Name): $($_.TotalMilliseconds) milliseconds"
        
        # Store the result in the results array
        $results += [PSCustomObject]@{
            TestFileName = $testFile.Name
            RunTimeMilliseconds = $_.TotalMilliseconds
        }
    }

    # Clean up the temporary file
    Remove-Item $tempFile
}

# Step 3: Save the results to a CSV file
$results | Export-Csv -Path "analysis/test_times.csv" -NoTypeInformation

Write-Host "Test times have been saved to 'analysis/test_times.csv'"
