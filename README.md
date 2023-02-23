# Financial Analysis

[toc]

## 项目结构

### 拓扑结构

![image-20230223094012643](README.assets/image-20230223094012643.png)

### 文件结构

```
$ tree -L 2
├── dipiper-server		爬虫服务器
├── Dockerfile			Docker 配置
├── docker_start.sh		Docker 脚本
├── financial-frontend	客户端程序
├── README.md			说明文档
├── releases			发布文件
│   ├── linux
│   └── windows
├── server				gRPC 服务端、静态文件服务器
├── simple-lstm-server	AI 服务器
```

## 运行说明

项目构建为 Docker 容器，运行其需要 Docker/Podman 软件。

由于爬虫部分使用了浏览器操作以减少被防爬拦截，所以难以塞进 Docker 中，仅提供源码和在线运行环境。

###  仅运行客户端

注意需要在菜单栏切换服务器到 `a.chiro.work`。

1. 网页版

   release 包的 `dist/` 下即为静态文件，`windows` / `linux` 下的 `dist` 的静态文件都是相同的。

   将这些静态文件挂载到静态文件服务器即可浏览网页客户端。

2. PC 客户端

   直接运行 `financial_analysis.exe`。

3. Android 客户端

   安装即用，用法和网页一致。

### 运行服务端

#### 运行 Docker 中的服务程序

由于部分内容难以塞进 Docker 里，在 Docker 里的只有这些：

<img src="README.assets/image-20230222215915429.png" alt="image-20230222215915429" style="zoom: 67%;" />



使用给定的镜像文件导入 Docker：

```shell
docker load -i financial.tar.gz
```

或者直接从 DockerHub 上拉取：

```shell
docker pull chiro2001/financial-analysis:v2
```

然后从这个镜像文件运行一个容器：

```shell
docker run -it --rm -p 51411:51411 chiro2001/financial-analysis:v2
```

访问 http://localhost:51411 即可访问静态网页客户端内容。

#### 运行 Release 包中的程序

内含：GRPC 服务器、静态文件服务器、客户端静态文件。

双击 `run_server.cmd`，然后打开 http://localhost:51411 即可访问客户端。

## 客户端使用说明

**主要功能**

1. 用户系统

   1. 在打开客户端后如果不登录，不获取用户对应 `token` 是无法与后端验证通信的
   2. 由于懒所以用户数据没有在数据库上做持久化，服务器重启后除了 `test` 其他用户数据会丢失
   3. 默认用户、密码为 `test`，打开客户端之后默认就是这个
   4. 注册是正常工作的
   5. 又因为懒得做 JWT 所以现在所有用户的 `token` 都是字符串 `"token"`，不过不影响使用就是了

2. 股票搜索

   <img src="README.assets/image-20230222225234638.png" alt="image-20230222225234638" style="zoom: 67%;" />

   1. 在搜索框中键入搜索词即可显示，不需要按回车
   2. 所有的股票简略信息被缓存到客户端
   3. 可以是股票代码、股票名称
   4. 支持正则表达式检索
   5. FIXME: 
      1. 这里有点问题，检索出的值好像是上一次搜索词的值，
      2. 可以在输入之后按一下空格以更新结果

3. 热门股票

   1. 侧边栏是随机选出的 6 支股票
   2. 对这些股票的操作和搜索结果一致

4. 查看股票 K 线

   ![image-20230222225606356](README.assets/image-20230222225606356.png)

   1. **双击**（手机上也是双击）搜索结果，打开详细信息窗口
   2. 可以切换 月/周/日 线
   3. 鼠标悬停可以看当前坐标的数据信息
   4. 底部是对应日期信息
   5. 拉动窗口可以缩放图像

5. 预测股票 K 线

   1. 鼠标拉动或者点击填写预测数量，然后点击预测
   2. 预测准确率好像不咋样
   3. 需要时间比较长，约 40s-1min
   4. 可能会预测出无效的信息

6. 股票发行信息

   ![image-20230222230243338](README.assets/image-20230222230243338.png)

   1. K 线下左侧栏目

7. 股票财务数据

   1. K 线下右侧栏目
   2. 鼠标拉动或者点击选择数据年份
   3. 然后点击按钮获取数据

8. 三年营业收入数据分析

   1. K 线下中间栏目
   2. 打开股票详细信息窗口后自动加载并计算
   3. 计算过程中获取的数据、计算结果会存入数据库 `dipiper.financial_analysis` 和 `dipiper.financial_data`

9. 其他功能

   1. 切换服务器
      1. 在菜单栏可以切换服务器
      2. `a.chiro.work` 一般都开启
      3. `localhost` 指的是本机地址，如果服务端运行在本机就选择这个
      4. 演示的时候请切换到 `a.chiro.work`
   2. 调试模式
      1. 菜单栏点击调试模式可以打开调试侧边栏
      2. 可以看 CPU 使用、帧率之类的
   3. 退出登录
      1. 点击后可以退出登录再重新登录
      2. 清除本地 `token` 信息
   4. 重新连接后端
      1. 调试用
   5. 主题切换器
      1. 左上角
      2. 点击可以切换白色 / 黑色主题

## 项目亮点

1. 功能丰富
   1. 提供多种功能 +上面列出的
2. 界面美观
   1. 提供了操作性强的界面
   2. 在页面内提供窗口功能，便于对不同的数据进行比较

3. 多端互联
   1. 使用 Rust 构建了大部分业务逻辑
   2. 同时使用 Rust 构建了客户端前端
   2. 实现了 PC 客户端、Android 软件端、网页端的完全相同的操作逻辑
   2. 同时 PC 端并不是使用 Web 技术实现的，没有 Electron 的巨大体积
4. 使用 Rust 新兴语言
   1. 使用编译型语言 Rust，运行速度快
   2. 客户端、服务端体积较小
   3. 利于项目管理
5. 将多种语言、架构、程序类型合并
   1. 加强了系统的综合能力

6. 使用 Docker 容器进行服务器布置
   1. 减少因服务器环境不同造成的后端不兼容
   2. 便于 Deploy 到不同的系统、设备上
   3. 减小服务端体积

7. 使用 gRPC、JRPC 进行数据沟通
   1. 建立了统一的 API 格式，统一了前后端开发的流程
   2. gRPC 基于 HTTP/2，能够同时支持长短连接，以及支持二进制数据直接传输，有助于提高反应速度、减小流量消耗
   3. JRPC 提供更通用的 RPC 方案，便于多种不同语言之间的通信

8. 融合的 HTTP 路由
   1. 在同一个端口对 HTTP/1 和 HTTP/2 请求进行处理
   2. 使用融合的 HTTP 路由保证两种请求都能正常执行
   3. 减小了端口开支，便于发布和维护

## 测试过程

在项目中使用了许多测试方法，如回归测试、单元测试等。例如：

1. 在 `server/src/client.rs` 中进行了单元测试：

```rust
use anyhow::Result;
use rpc::api::LoginRegisterRequest;
use rpc::API_PORT;
use tracing::info;

// 对 gRPC 两个 Service 的基本连接测试
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = format!("http://0.0.0.0:{}", API_PORT);

    let mut client = rpc::api::api_rpc_client::ApiRpcClient::connect(addr.clone()).await?;
    info!("got client: {:?}", client);
    let r = client.login(LoginRegisterRequest::default()).await?;
    info!("login resp: {:?}", r);

    let mut client = rpc::api::register_client::RegisterClient::connect(addr.clone()).await?;
    info!("got register client: {:?}", client);
    let r = client.register(LoginRegisterRequest::default()).await?;
    info!("register resp: {:?}", r);

    Ok(())
}
```

2. 在许多 Rust 文件的注释中存在的测试。

3. 在 Postman 中进行的 API 回归测试。

   ![image-20230223110950620](README.assets/image-20230223110950620.png)

4. 在 Github Action 上进行的持续性集成测试。

   ![image-20230223111148313](README.assets/image-20230223111148313.png)
