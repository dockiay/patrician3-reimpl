# Asistente de Patrician III (reimplementacion en Rust).
# Busca la copia del juego del usuario, la comprueba, escribe p3-juego.ini y abre el juego.
param([switch]$Configurar)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
[System.Windows.Forms.Application]::EnableVisualStyles()

$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$client = Join-Path $here 'p3-juego.exe'
$ini = Join-Path $here 'p3-juego.ini'
$exeName = 'Patrician3.exe'
$cprName = 'p2arch0_eng.cpr'
$goodHash = 'fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece'

function Test-Game([string]$folder) {
    # Devuelve: 'ok', 'otra' (otra edicion), 'sincpr', 'no'
    if ([string]::IsNullOrWhiteSpace($folder) -or -not (Test-Path -LiteralPath $folder -PathType Container)) { return 'no' }
    $exe = Join-Path $folder $exeName
    if (-not (Test-Path -LiteralPath $exe -PathType Leaf)) { return 'no' }
    if (-not (Test-Path -LiteralPath (Join-Path $folder $cprName) -PathType Leaf)) { return 'sincpr' }
    try {
        $hash = (Get-FileHash -LiteralPath $exe -Algorithm SHA256).Hash.ToLowerInvariant()
    } catch { return 'otra' }
    if ($hash -eq $goodHash) { return 'ok' }
    return 'otra'
}

function Read-Ini {
    if (-not (Test-Path -LiteralPath $ini)) { return $null }
    foreach ($line in [System.IO.File]::ReadAllLines($ini)) {
        if ($line -match '^\s*original_exe\s*=\s*"?(.+?)"?\s*$') {
            $path = $Matches[1]
            if (-not [System.IO.Path]::IsPathRooted($path)) { $path = Join-Path $here $path }
            return (Split-Path -Parent $path)
        }
    }
    return $null
}

function Write-Ini([string]$folder) {
    $text = "# Escrito por el asistente. Borra este archivo para volver a elegir la carpeta.`r`n" +
            "original_exe=" + (Join-Path $folder $exeName) + "`r`n" +
            "resources=" + (Join-Path $folder $cprName) + "`r`n"
    [System.IO.File]::WriteAllText($ini, $text, (New-Object System.Text.UTF8Encoding($false)))
}

function Find-Game {
    $candidates = New-Object System.Collections.Generic.List[string]
    $candidates.Add($here)
    # Instalaciones de GOG y del instalador original (registro de Windows).
    $keys = @(
        'HKLM:\SOFTWARE\WOW6432Node\GOG.com\Games', 'HKLM:\SOFTWARE\GOG.com\Games',
        'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall',
        'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall',
        'HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall'
    )
    foreach ($key in $keys) {
        if (-not (Test-Path $key)) { continue }
        foreach ($sub in (Get-ChildItem $key -ErrorAction SilentlyContinue)) {
            try {
                $p = Get-ItemProperty $sub.PSPath -ErrorAction Stop
                $name = '' + $p.gameName + ' ' + $p.DisplayName
                if ($name -notmatch 'Patri[cz]i') { continue }
                foreach ($value in @($p.path, $p.InstallLocation, $p.workingDir)) {
                    if ($value) { $candidates.Add([string]$value) }
                }
            } catch { }
        }
    }
    # Carpetas habituales en todos los discos.
    $names = @('GOG Games\Patrician 3', 'GOG Games\Patrician III', 'Games\Patrician 3', 'Games\Patrician III',
               'Juegos\Patrician 3', 'Juegos\Patrician III', 'Patrician 3', 'Patrician III',
               'Program Files (x86)\GOG Galaxy\Games\Patrician 3', 'Program Files (x86)\GOG Galaxy\Games\Patrician III',
               'Program Files (x86)\Steam\steamapps\common\Patrician III', 'SteamLibrary\steamapps\common\Patrician III',
               'Program Files (x86)\Ascaron Entertainment\Patrician III', 'Program Files (x86)\Patrician III')
    foreach ($drive in [System.IO.DriveInfo]::GetDrives()) {
        if ($drive.DriveType -ne 'Fixed' -or -not $drive.IsReady) { continue }
        foreach ($name in $names) { $candidates.Add((Join-Path $drive.RootDirectory.FullName $name)) }
    }
    $other = $null
    foreach ($folder in $candidates) {
        foreach ($f in @($folder, (Join-Path $folder 'Patrician III'), (Join-Path $folder 'app'))) {
            $state = Test-Game $f
            if ($state -eq 'ok') { return $f }
            if ($state -eq 'otra' -and -not $other) { $other = $f }
        }
    }
    return $other
}

function Start-Game {
    Start-Process -FilePath $client -WorkingDirectory $here
}

if (-not (Test-Path -LiteralPath $client)) {
    [void][System.Windows.Forms.MessageBox]::Show("Falta p3-juego.exe junto a este asistente.`n`nDescomprime el ZIP entero en una carpeta y vuelve a abrir Jugar.", 'Patrician III', 'OK', 'Error')
    exit 1
}

# Si ya esta configurado y sigue siendo valido, se abre el juego sin preguntar.
$saved = Read-Ini
if (-not $Configurar -and $saved -and (Test-Game $saved) -eq 'ok') {
    Start-Game
    exit 0
}

# ---------- Ventana ----------
$form = New-Object System.Windows.Forms.Form
$form.Text = 'Patrician III - Preparar el juego'
$form.ClientSize = New-Object System.Drawing.Size(620, 400)
$form.StartPosition = 'CenterScreen'
$form.FormBorderStyle = 'FixedDialog'
$form.MaximizeBox = $false
$form.Font = New-Object System.Drawing.Font('Segoe UI', 10)
$form.BackColor = [System.Drawing.Color]::FromArgb(250, 246, 236)

$title = New-Object System.Windows.Forms.Label
$title.Text = 'Patrician III'
$title.Font = New-Object System.Drawing.Font('Georgia', 22, [System.Drawing.FontStyle]::Bold)
$title.ForeColor = [System.Drawing.Color]::FromArgb(110, 70, 20)
$title.SetBounds(24, 16, 570, 44)
$form.Controls.Add($title)

$intro = New-Object System.Windows.Forms.Label
$intro.Text = "Para jugar hace falta tu propia copia de Patrician III (en ingles, version 1.1, la que vende GOG). " +
              "Este programa no trae el juego: solo necesita saber en que carpeta lo tienes instalado. No se cambia nada de esa carpeta."
$intro.SetBounds(26, 66, 570, 66)
$form.Controls.Add($intro)

$step = New-Object System.Windows.Forms.Label
$step.Text = 'Carpeta donde esta instalado el juego:'
$step.Font = New-Object System.Drawing.Font('Segoe UI', 10, [System.Drawing.FontStyle]::Bold)
$step.SetBounds(26, 140, 570, 24)
$form.Controls.Add($step)

$box = New-Object System.Windows.Forms.TextBox
$box.SetBounds(28, 168, 440, 28)
$form.Controls.Add($box)

$browse = New-Object System.Windows.Forms.Button
$browse.Text = 'Buscar...'
$browse.SetBounds(478, 166, 114, 30)
$form.Controls.Add($browse)

$status = New-Object System.Windows.Forms.Label
$status.SetBounds(28, 208, 564, 96)
$status.Font = New-Object System.Drawing.Font('Segoe UI', 10)
$form.Controls.Add($status)

$auto = New-Object System.Windows.Forms.Button
$auto.Text = 'Buscarlo por mi'
$auto.SetBounds(28, 340, 150, 38)
$form.Controls.Add($auto)

$play = New-Object System.Windows.Forms.Button
$play.Text = 'Jugar'
$play.Font = New-Object System.Drawing.Font('Segoe UI', 12, [System.Drawing.FontStyle]::Bold)
$play.SetBounds(442, 334, 150, 46)
$play.Enabled = $false
$form.Controls.Add($play)
$form.AcceptButton = $play

$green = [System.Drawing.Color]::FromArgb(30, 110, 40)
$red = [System.Drawing.Color]::FromArgb(170, 30, 30)
$amber = [System.Drawing.Color]::FromArgb(150, 95, 0)

function Update-Status {
    $folder = $box.Text.Trim().Trim('"')
    if ($folder -eq '') {
        $status.ForeColor = [System.Drawing.Color]::Black
        $status.Text = "Pulsa «Buscar...» y elige la carpeta del juego, o «Buscarlo por mi» para que lo intente encontrar el asistente.`nEs la carpeta que contiene el archivo Patrician3.exe."
        $play.Enabled = $false
        return
    }
    switch (Test-Game $folder) {
        'ok' {
            $status.ForeColor = $green
            $status.Text = "Correcto: es Patrician III en ingles, version 1.1.`nYa puedes pulsar «Jugar»."
            $play.Enabled = $true
        }
        'otra' {
            $status.ForeColor = $red
            $status.Text = "Aqui hay un Patrician III, pero no es la edicion en ingles 1.1 (por ejemplo, es la espanola u otra version), y con esa el juego no arranca.`nElige la carpeta donde tengas la version inglesa de GOG."
            $play.Enabled = $false
        }
        'sincpr' {
            $status.ForeColor = $red
            $status.Text = "En esta carpeta esta Patrician3.exe pero falta el archivo p2arch0_eng.cpr.`nHace falta la edicion en ingles del juego, instalada completa."
            $play.Enabled = $false
        }
        default {
            $status.ForeColor = $red
            $status.Text = "En esta carpeta no esta el juego (no hay ningun Patrician3.exe).`nElige la carpeta donde lo instalaste; suele llamarse «Patrician 3» o «Patrician III»."
            $play.Enabled = $false
        }
    }
}

$box.Add_TextChanged({ Update-Status })

$browse.Add_Click({
    $dialog = New-Object System.Windows.Forms.FolderBrowserDialog
    $dialog.Description = 'Elige la carpeta donde tienes instalado Patrician III (la que contiene Patrician3.exe)'
    $dialog.ShowNewFolderButton = $false
    $current = $box.Text.Trim()
    if ($current -ne '' -and (Test-Path -LiteralPath $current)) { $dialog.SelectedPath = $current }
    if ($dialog.ShowDialog($form) -eq 'OK') {
        $chosen = $dialog.SelectedPath
        # Si eligieron la carpeta de encima, se entra en la subcarpeta que tenga el juego.
        if ((Test-Game $chosen) -eq 'no') {
            foreach ($sub in (Get-ChildItem -LiteralPath $chosen -Directory -ErrorAction SilentlyContinue)) {
                if ((Test-Game $sub.FullName) -ne 'no') { $chosen = $sub.FullName; break }
            }
        }
        $box.Text = $chosen
    }
})

$auto.Add_Click({
    $form.Cursor = 'WaitCursor'
    $status.ForeColor = [System.Drawing.Color]::Black
    $status.Text = 'Buscando el juego en este equipo...'
    $form.Refresh()
    $found = Find-Game
    $form.Cursor = 'Default'
    if ($found) { $box.Text = $found }
    else {
        $status.ForeColor = $red
        $status.Text = "No lo he encontrado en los sitios habituales.`nPulsa «Buscar...» y elige tu la carpeta donde lo instalaste."
    }
})

$play.Add_Click({
    try {
        Write-Ini ($box.Text.Trim().Trim('"'))
        Start-Game
        $form.Close()
    } catch {
        [void][System.Windows.Forms.MessageBox]::Show("No se ha podido abrir el juego:`n$($_.Exception.Message)", 'Patrician III', 'OK', 'Error')
    }
})

$form.Add_Shown({
    if ($saved) { $box.Text = $saved }
    else {
        $found = Find-Game
        if ($found) { $box.Text = $found }
    }
    Update-Status
})

[void]$form.ShowDialog()
