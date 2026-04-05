$repo = "C:\Users\Admin\Documents\SSBUMods"

Set-Location $repo

git add .

git diff --cached --quiet
if ($LASTEXITCODE -ne 0) {
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    git commit -m "Autosave $timestamp"
    git push origin main
}