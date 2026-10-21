# ASCII only: this validator must itself be readable by Windows PowerShell 5.1.
param([Parameter(Mandatory=$true)][string]$BundleRoot)
$ErrorActionPreference = 'Stop'
if ($PSVersionTable.PSEdition -ne 'Desktop' -or $PSVersionTable.PSVersion.Major -ne 5 -or $PSVersionTable.PSVersion.Minor -ne 1) {
    throw 'Validation requires Windows PowerShell 5.1.'
}
$scriptPath = Join-Path $BundleRoot 'portable.ps1'
$tokens = $null
$parseErrors = $null
[void][System.Management.Automation.Language.Parser]::ParseFile($scriptPath, [ref]$tokens, [ref]$parseErrors)
if ($parseErrors.Count -gt 0) {
    throw ($parseErrors | Out-String)
}
# LibraryOnly avoids registration, process launch, UAC and all user-data changes.
. $scriptPath -LibraryOnly
Assert-Payload
foreach ($view in @('Registry64', 'Registry32')) {
    if (-not (Test-Path -LiteralPath (Join-Path $BundleRoot $dllNames[$view]) -PathType Leaf)) {
        throw "Missing registered DLL payload: $view"
    }
}
Write-Output 'Windows PowerShell 5.1: syntax, library load and payload checks passed.'
