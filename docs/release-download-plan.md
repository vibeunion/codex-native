# Release、CTFile 与下载量计划

## 目标

面向 Codex Desktop 用户，保留 GitHub Release 作为国际主发布渠道，并用 CTFile
提供中国大陆更稳定的二进制下载入口。下载收益用于：

- macOS Developer ID 代码签名证书；
- Apple notarization 相关费用；
- Apple Developer Program / App Store 上架费用；
- 必要的构建、存储和流量支出。

30,000～40,000 次/年约等于：

| 口径 | 目标 |
| --- | ---: |
| 年下载 | 30,000～40,000 |
| 月下载 | 2,500～3,333 |
| 日均下载 | 82～110 |

运营目标建议按 3,500 次/月规划，保留约 5%～15% 的波动空间，不把下载次数
承诺为固定收入。

## 发布链路

1. 推送 `vX.Y.Z` tag。
2. macOS、Windows 测试、fmt、clippy 和 workspace tests 通过。
3. macOS 归档完成签名、notarization 和 stapling 校验。
4. 准备二进制与 `SHA256SUMS`，但暂不创建 GitHub Release。
5. `scripts/publish-ctfile.ts` 在 CTFile 创建 `codex-native/<version>` 目录，
   使用 MD5/文件大小申请上传地址，流式上传后回读目录。
6. 只有拿到 CTFile API 返回的 canonical HTTPS 分享地址、且远端大小匹配时，
   才生成 `ctfile-links.md`。
7. 创建 GitHub Release，将 CTFile 表格放入 Release notes，并把 `ctfile-links.md`
   作为 Release 资产保存。

CTFile 上传失败会使正式 Release job 失败。这样可以重跑同一个 tag，脚本会复用
同名且大小一致的远端文件，不会因为网络重试制造重复版本。

## GitHub 配置

必需 Secrets：

- `CTFILE_SESSION`
- `CTFILE_FOLDER_ID`

可选 Repository variable：

- `CTFILE_PUBLIC_ROOT_URL`：真实公开目录地址，例如
  `https://url88.ctfile.com/dir/d123456`。不设置时由文件夹 ID 生成兼容地址。

凭据只存在于 GitHub Actions Secrets。日志只输出文件名、大小和有限错误类型，
不会输出 session、密码或带签名参数的上传 URL。

## 统计口径

每月记录以下数据，保存在发布运营表或私有报表中：

- CTFile 总下载次数、按版本和文件名拆分；
- GitHub Release 下载次数，作为对照而不是简单相加；
- 失败下载、重复下载和异常峰值；
- 二进制平均大小、总流量和每月实际收益；
- 代码签名、notarization、Apple 开发者账户及构建基础设施支出。

下载次数不等于独立用户数。对外只公布总下载量和用途，内部再结合版本、平台、
日期和流量判断真实覆盖。

## 流量与费用预算

用以下公式做月度预算：

`月流量 = 月下载次数 × 平均二进制大小 × (1 + 重试/重复下载系数)`

先按 3,500 次/月和 10% 重复下载系数测算，再用 CTFile 月度报表校正。若连续两个月
超过目标流量或收益不足以覆盖签名/Apple 固定成本，应优先：

1. 保留 GitHub Release 的校验文件；
2. 在 README 和 Release 中引导用户按平台下载单个文件；
3. 避免把多个平台打包成不必要的大归档；
4. 再评估是否需要备用 WebDAV 或其他国内镜像。

## 上线验收

首次真实发布前需要人工完成：

- 创建 CTFile 公开根目录并确认匿名浏览/下载权限；
- 将真实 `CTFILE_SESSION`、`CTFILE_FOLDER_ID` 写入 GitHub Secrets；
- 设置并验证 `CTFILE_PUBLIC_ROOT_URL`；
- 使用一个测试 tag 验证 Release、CTFile 文件、canonical 链接和 SHA-256；
- 确认 macOS 签名、notarization、Apple 开发者账户均可用。

在这些外部账号和真实凭据完成前，本地代码和 CI 配置可以验证，但不能声称已经
完成真实 CTFile 上传、收益统计或 App Store 上架。
