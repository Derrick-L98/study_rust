使用 Tauri 打包你的应用

运行
npm install
npm run tauri dev

基础配置
src-tauri/tauri.conf.json
{
  "tauri": {
    "cli": {
      "description": "", // command description that's shown on help
      "longDescription": "", // command long description that's shown on help
      "beforeHelp": "", // content to show before the help text
      "afterHelp": "", // content to show after the help text
      "args": [], // list of arguments of the command, we'll explain it later
      "subcommands": {
        "subcommand-name": {
          // configures a subcommand that is accessible
          // with `./app subcommand-name --arg1 --arg2 --etc`
          // configuration as above, with "description", "args", etc.
        }
      }
    }
  }
}

https://tauri.app/zh-cn/v1/guides/