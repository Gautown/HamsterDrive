# Download Source Han Sans Font Script
Write-Host "Downloading Source Han Sans Font..."

# Create temporary directory
$tempDir = "$env:TEMP\HamsterDrive_Fonts"
if (!(Test-Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
}

# Download Source Han Sans
$fontUrl = "https://github.com/adobe-fonts/source-han-sans/raw/release/OTF/SimplifiedChinese/SourceHanSansSC-Regular.otf"
$fontPath = "$tempDir\SourceHanSansSC-Regular.otf"

try {
    Write-Host "Downloading: $fontUrl"
    Invoke-WebRequest -Uri $fontUrl -OutFile $fontPath
    Write-Host "Download completed!"
    
    # Copy to project fonts directory
    $projectFontDir = "g:\GitHub\HamsterDrive\fonts"
    Copy-Item -Path $fontPath -Destination $projectFontDir
    Write-Host "Font file copied to project directory: $projectFontDir"
    
    Write-Host "Source Han Sans downloaded and installed successfully!"
} catch {
    Write-Host "Download failed: $($_.Exception.Message)"
    Write-Host "Please manually download Source Han Sans font file and place it in the project's fonts directory"
}