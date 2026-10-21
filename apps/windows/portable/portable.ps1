# 青简解压版入口。注册操作只针对本目录，拒绝覆盖其他安装目录。
param(
    [ValidateSet('Enable', 'Disable', 'Start', 'Settings', 'Status')]
    [string]$Action = 'Status',
    [switch]$LibraryOnly
)

$ErrorActionPreference = 'Stop'
$bundleRoot = [IO.Path]::GetFullPath($PSScriptRoot)
$serviceClsid = '{4FDCA82D-E923-49BF-9E75-BB906B93B8BB}'
$dllNames = @{
    Registry64 = 'qingjian_tsf-custom3-portable3.dll'
    Registry32 = 'qingjian_tsf-custom3-portable3-x86.dll'
}

function Test-SamePath([string]$Left, [string]$Right) {
    if (-not $Left -or -not $Right) { return $false }
    return [string]::Equals([IO.Path]::GetFullPath($Left.Trim('"')), [IO.Path]::GetFullPath($Right.Trim('"')), [StringComparison]::OrdinalIgnoreCase)
}

function Get-Registrations {
    foreach ($view in @('Registry64', 'Registry32')) {
        foreach ($hive in @('LocalMachine', 'CurrentUser')) {
            $base = [Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]$hive, [Microsoft.Win32.RegistryView]$view)
            $key = $null
            try {
                $key = $base.OpenSubKey("Software\Classes\CLSID\$serviceClsid\InprocServer32")
                if ($null -ne $key) {
                    $value = [string]$key.GetValue('')
                    if ($value) { [pscustomobject]@{ Hive = $hive; View = $view; Path = $value } }
                }
            } finally {
                if ($null -ne $key) { $key.Dispose() }
                $base.Dispose()
            }
        }
    }
}

function Assert-OwnedRegistrations($Entries) {
    foreach ($entry in $Entries) {
        $expected = Join-Path $bundleRoot $dllNames[$entry.View]
        if (-not (Test-SamePath $entry.Path $expected)) {
            throw "检测到其他位置的青简注册：$($entry.Path)`n请先备份原配置，通过原安装版的卸载程序或原便携包的 Disable.cmd 停用，再启用本包。本脚本不会覆盖其他版本。"
        }
    }
}

function Assert-Payload {
    foreach ($name in @('qingjian-portable.txt', 'qingjian-server.exe', 'qingjian-settings.exe', $dllNames.Registry64, $dllNames.Registry32, 'data\generated\dict.qj')) {
        if (-not (Test-Path -LiteralPath (Join-Path $bundleRoot $name) -PathType Leaf)) {
            throw "缺少 $name。请先完整解压，不能从压缩包中直接运行。"
        }
    }
}

function Test-Administrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    return ([Security.Principal.WindowsPrincipal]::new($identity)).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Show-Result([string]$Message, [bool]$Failed = $false) {
    Add-Type -AssemblyName System.Windows.Forms
    $icon = if ($Failed) { [Windows.Forms.MessageBoxIcon]::Error } else { [Windows.Forms.MessageBoxIcon]::Information }
    [void][Windows.Forms.MessageBox]::Show($Message, '青简便携版', [Windows.Forms.MessageBoxButtons]::OK, $icon)
}

function Invoke-Registration([string]$View, [bool]$Remove) {
    $regsvr = if ($View -eq 'Registry64') { 'System32\regsvr32.exe' } else { 'SysWOW64\regsvr32.exe' }
    $dll = Join-Path $bundleRoot $dllNames[$View]
    $arguments = if ($Remove) { '/u /s "{0}"' -f $dll } else { '/s "{0}"' -f $dll }
    $process = Start-Process -FilePath (Join-Path $env:SystemRoot $regsvr) -ArgumentList $arguments -WindowStyle Hidden -Wait -PassThru
    if ($process.ExitCode -ne 0) { throw "regsvr32 ($View) 失败，退出码 $($process.ExitCode)。请确认权限和完整解压。" }
}

function Grant-ProgramReadAccess {
    # 仅授予程序与随包资源读取权限，不给 userdata 下的配置、密钥和学习数据加权限。
    $icacls = Join-Path $env:SystemRoot 'System32\icacls.exe'
    & $icacls $bundleRoot /grant '*S-1-15-2-1:RX' /Q | Out-Null
    if ($LASTEXITCODE -ne 0) { throw '无法设置目录读取权限，请解压到本机 NTFS 目录。' }
    foreach ($file in Get-ChildItem -LiteralPath $bundleRoot -File | Where-Object { $_.Extension -in '.exe', '.dll', '.ico', '.pri', '.manifest' }) {
        & $icacls $file.FullName /grant '*S-1-15-2-1:RX' /Q | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "无法设置程序读取权限：$($file.Name)" }
    }
    foreach ($directory in Get-ChildItem -LiteralPath $bundleRoot -Directory | Where-Object { $_.Name -notin @('userdata', 'source', 'licenses') }) {
        & $icacls $directory.FullName /grant '*S-1-15-2-1:(OI)(CI)RX' /T /C /Q | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "无法设置资源读取权限：$($directory.Name)" }
    }
}

function Assert-NoOtherServer {
    foreach ($process in @(Get-Process -Name 'qingjian-server' -ErrorAction SilentlyContinue)) {
        if (-not $process.Path -or -not (Test-SamePath $process.Path (Join-Path $bundleRoot 'qingjian-server.exe'))) {
            throw '其他目录的青简服务仍在运行。请先退出原版本，或注销后重新登录，再启动本便携版。'
        }
    }
}

if ($LibraryOnly) { return }
try {
    if (-not [Environment]::Is64BitOperatingSystem -or -not [Environment]::Is64BitProcess) {
        throw '请在 64 位 Windows 中使用包内的 cmd 入口。'
    }
    Assert-Payload
    if ($Action -eq 'Status') {
        Write-Output "程序目录：$bundleRoot"
        Write-Output "便携数据：$(Join-Path $bundleRoot 'userdata')"
        $entries = @(Get-Registrations)
        if ($entries.Count -eq 0) { Write-Output '当前未发现青简 COM 注册。' } else { $entries | Format-Table -AutoSize }
        exit 0
    }
    # 提权前、提权后均检查，且不修改任何其他版本的注册。
    Assert-OwnedRegistrations @(Get-Registrations)
    if ($Action -in @('Enable', 'Disable') -and -not (Test-Administrator)) {
        $arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`" -Action $Action"
        $child = Start-Process -FilePath (Join-Path $env:SystemRoot 'System32\WindowsPowerShell\v1.0\powershell.exe') -ArgumentList $arguments -Verb RunAs -WindowStyle Hidden -Wait -PassThru
        exit $child.ExitCode
    }
    if ($Action -in @('Enable', 'Disable')) {
        $created = $false
        $lock = [Threading.Mutex]::new($false, 'Global\QingjianInstaller', [ref]$created)
        try {
            if (-not $created) { throw '另一个青简安装或启停操作正在进行，请稍后再试。' }
            $entries = @(Get-Registrations)
            Assert-OwnedRegistrations $entries
            if ($Action -eq 'Enable') {
                Assert-NoOtherServer
                Grant-ProgramReadAccess
                try {
                    Invoke-Registration 'Registry64' $false
                    Invoke-Registration 'Registry32' $false
                    $now = @(Get-Registrations)
                    Assert-OwnedRegistrations $now
                    foreach ($view in @('Registry64', 'Registry32')) {
                        if (-not ($now | Where-Object View -eq $view)) { throw "$view 注册校验失败。" }
                    }
                } catch {
                    $registrationError = $_
                    # 任一位数注册失败时撤销本包的部分注册；冲突版本不碰。
                    Assert-OwnedRegistrations @(Get-Registrations)
                    foreach ($view in @('Registry32', 'Registry64')) {
                        try { Invoke-Registration $view $true } catch { Write-Warning $_ }
                    }
                    throw $registrationError
                }
                Show-Result '已启用。请以普通权限双击 Start.cmd，再用 Win+Space 切换到青简。若切换列表未刷新，请注销后重新登录。启用期间请勿移动文件夹。'
            } else {
                if ($entries.Count -eq 0) { Show-Result '没有本包的注册，无需注销。'; exit 0 }
                Invoke-Registration 'Registry32' $true
                Invoke-Registration 'Registry64' $true
                if (@(Get-Registrations).Count -ne 0) { throw '注册仍存在，已保留目录，请检查状态后重试。' }
                foreach ($process in @(Get-Process -Name 'qingjian-server' -ErrorAction SilentlyContinue)) {
                    if (Test-SamePath $process.Path (Join-Path $bundleRoot 'qingjian-server.exe')) { Stop-Process -Id $process.Id -Force }
                }
                Show-Result '已停用。userdata 已保留。请注销或重启后再移动、删除文件夹，避免 DLL 仍被应用占用。'
            }
        } finally { $lock.Dispose() }
    } else {
        if (Test-Administrator) { throw '启动程序或设置时请不要使用管理员权限；直接双击 Start.cmd 或 Settings.cmd。' }
        New-Item -ItemType Directory -Path (Join-Path $bundleRoot 'userdata') -Force | Out-Null
        if ($Action -eq 'Start') {
            Assert-NoOtherServer
            $entries = @(Get-Registrations)
            foreach ($view in @('Registry64', 'Registry32')) {
                if (-not ($entries | Where-Object View -eq $view)) { throw '尚未完整启用，请先运行 Enable.cmd。' }
            }
            if (-not (Get-Process -Name 'qingjian-server' -ErrorAction SilentlyContinue)) {
                Start-Process -FilePath (Join-Path $bundleRoot 'qingjian-server.exe') -WorkingDirectory $bundleRoot -WindowStyle Hidden
            }
        } else {
            Start-Process -FilePath (Join-Path $bundleRoot 'qingjian-settings.exe') -WorkingDirectory $bundleRoot -WindowStyle Hidden
        }
    }
} catch {
    Write-Error -Message $_.Exception.Message -ErrorAction Continue
    Show-Result $_.Exception.Message $true
    exit 1
}
