param (
    [Parameter(Mandatory = $true)]
    [int]$day_number,

    [int]$year_number = 24
)

# Pad the day number to 2 digits
$day_number_padded = $day_number.ToString("D2")

# Copy template files
$template_path = "./template"
$destination_path = "a20$year_number/src/bin/${year_number}day$day_number_padded/"

mkdir $destination_path > $null
Copy-Item -Recurse -Force $template_path/* $destination_path