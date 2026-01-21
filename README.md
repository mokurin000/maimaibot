# maimaibot

基于 Kovi ，使用 Rust + Python 开发的，易于部署的舞萌机器人。

## 环境

rust (cargo, llvm, ...)

语音处理相关: [silk-codec](https://github.com/foyoux/silk-codec), ffmpeg

## 编译

1.53 以来，舞萌再无科技攻击或服务器 DDoS 阵亡
sdgb-utils-rs 已开源

```bash
cargo fetch --locked
cargo build --release --frozen
```

## 语音提取

> 将 silk-encoder-x64.exe 放置到 `voices/silk-encoder.exe`

请自行搜索 “HDD” 并自行搜索 “CriTools”、“vgmstream” 等，完成 acb -> wav 转换后，

```bash
# 假设 Voice_*/ 都在此处
cd voices

uv run remove-silent.py
uv run wav2pcm.py
uv run pcm2silk.py

# 若不转换，手机QQ无法播放
uv run silk-std2tencent.py
# 可选，删除 wav/pcm 文件
uv run clean_files.py
```

## 后端

- [LLOneBot](https://llonebot.com/)
- [NapCatQQ](https://napneko.github.io/)
- [Lagrange](https://github.com/LagrangeDev/Lagrange.Core)

以上均为可选的 qqnt + websocket正向 onebotv11 服务器后端

理论上 [OlivOS](https://github.com/OlivOS-Team/OlivOS) 提供了多平台后端的 onebotv11 服务器端，可使本项目适配 Telegram 等平台，但由于实际上需要修改 play-voice 识别不同平台决定发送 silk 还是 ogg 等等，本项目暂不提供可用性支持。
