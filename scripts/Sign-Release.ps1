[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$EncryptedKeyPath,
    [string]$Package,
    [string]$MinisignPath = 'minisign'
)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Security
$ProjectRoot = Split-Path $PSScriptRoot -Parent
$PublicKey = Join-Path $ProjectRoot 'signing\portage-simple-durability-tooltip.pub'
if (-not $Package) {
    $Packages = @(Get-ChildItem -LiteralPath (Join-Path $ProjectRoot 'dist') -Filter '*.hnmod' -File)
    if ($Packages.Count -ne 1) { throw 'Select exactly one package with -Package.' }
    $Package = $Packages[0].FullName
}
$Package = (Resolve-Path -LiteralPath $Package).Path
$EncryptedKeyPath = (Resolve-Path -LiteralPath $EncryptedKeyPath).Path
$RootPrefix = [IO.Path]::GetFullPath($ProjectRoot).TrimEnd('\') + '\'
if ($EncryptedKeyPath.StartsWith($RootPrefix,[StringComparison]::OrdinalIgnoreCase)) {
    throw 'The private key must be kept outside the repository.'
}
if (-not (Test-Path -LiteralPath $PublicKey)) { throw 'Project public key is missing.' }
$TemporaryKey = Join-Path (Split-Path $EncryptedKeyPath -Parent) ('.sign-' + [Guid]::NewGuid().ToString('N') + '.key')
$Plain = $null
try {
    $Plain = [Security.Cryptography.ProtectedData]::Unprotect(
        [IO.File]::ReadAllBytes($EncryptedKeyPath), $null,
        [Security.Cryptography.DataProtectionScope]::CurrentUser)
    [IO.File]::WriteAllBytes($TemporaryKey, $Plain)
    & $MinisignPath -S -W -s $TemporaryKey -m $Package -t ([IO.Path]::GetFileName($Package))
    if ($LASTEXITCODE -ne 0) { throw 'Signing failed.' }
    & $MinisignPath -V -p $PublicKey -m $Package
    if ($LASTEXITCODE -ne 0) { throw 'Signature verification failed.' }
} finally {
    if ($Plain) { [Array]::Clear($Plain, 0, $Plain.Length) }
    if (Test-Path -LiteralPath $TemporaryKey) { Remove-Item -LiteralPath $TemporaryKey -Force }
}
