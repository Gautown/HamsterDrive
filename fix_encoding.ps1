# 修复文件编码的PowerShell脚本
# 将UTF-8编码的文件转换为带BOM的UTF-8编码，以便在Windows上正确显示中文

Write-Host "开始修复文件编码..."

# 获取所有.md和.rs文件
$files = Get-ChildItem -Path "." -Include *.md,*.rs -Recurse | Where-Object { $_.FullName -notlike "*target*" }

foreach ($file in $files) {
    Write-Host "正在处理文件: $($file.FullName)"
    
    # 读取文件内容
    $content = Get-Content -Path $file.FullName -Encoding UTF8
    
    # 以UTF8 with BOM格式重新写入文件
    $content | Out-File -FilePath $file.FullName -Encoding UTF8
    
    Write-Host "已修复: $($file.Name)"
}

Write-Host "编码修复完成"