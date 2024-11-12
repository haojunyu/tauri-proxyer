# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)






Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:

```bash
  cd tauri-app
  corepack enable
  pnpm install
  pnpm tauri android init
  pnpm tauri ios init
```

For Desktop development, run:

```bash
  pnpm tauri dev
```

For Android development, run:
```bash
  pnpm tauri android dev
```

For iOS development, run:
```bash
  pnpm tauri ios dev
```



## proxyer

pac 地址: https://raw.githubusercontent.com/thep0y/pac/main/blacklist.pac

### windows

```bash
# 查询代理
reg query "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v AutoConfigURL
# 设置代理
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v AutoConfigURL /t REG_SZ /d "<pac地址>" /f
# 关闭代理
reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v AutoConfigURL /t REG_SZ /d "" /f
```


## FAQ

1. switch Developer role

> The following build commands failed:
>        PhaseScriptExecution Build\ Rust\ Code /Users/xxx/Library/Developer/Xcode/DerivedData/ tauri-proxyer-bwulhkjmautresdzlpndheorrfpj/Build/Intermediates.noindex/tauri-proxyer.build/debug-iphonesimulator/tauri-proxyer_iOS.build/Script-FA7B3BDA684BE5FFE0916FE9.sh (in target 'tauri-proxyer_iOS' from project 'tauri-proxyer')

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```