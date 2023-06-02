## nako-blog 博客系统

`nako-blog` 是使用 `actix-web`, `sea-orm` 及 `tera` 的 `rust` 博客系统


### 项目介绍

*  使用 `rust` 开发的通用博客系统
*  核心使用 `actix-web`, `sea-orm` 及 `tera` 等开发
*  博客后台使用 `pear-admin` 后端模板，非前后端分离项目
*  博客模板为可设置模板


### 环境要求

 - rust >= 1.18
 - Myql
 - Redis


### 截图预览

<table>
    <tr>
        <td width="50%">
            <center>
                <img alt="登录" src="https://github.com/deatil/nako-blog/assets/24578855/876ee1aa-ca6c-44e4-ac38-2db742448c93" />
            </center>
        </td>
        <td width="50%">
            <center>
                <img alt="控制台" src="https://github.com/deatil/nako-blog/assets/24578855/5619400f-d223-490e-8fce-50d48289885f" />
            </center>
        </td>
    </tr>
    <tr>
        <td width="50%">
            <center>
                <img alt="文章管理" src="https://github.com/deatil/nako-blog/assets/24578855/cecc15bb-c318-41ae-96aa-a2edac0ef2a7" />
            </center>
        </td>
        <td width="50%">
            <center>
                <img alt="附件管理" src="https://github.com/deatil/nako-blog/assets/24578855/5d5d4ac9-1d39-4afa-9dbd-74ba6579e633" />
            </center>
        </td>
    </tr>
</table>

更多截图
[nako-blog 截图](https://github.com/deatil/nako-blog/issues/2)


### 安装及开发步骤

1. 首先克隆项目到本地

```
git clone https://github.com/deatil/nako-blog.git
```

2. 然后配置数据库等信息

```
/conf.ini
```

3. 最后导入 sql 数据到数据库

```
/docs/nako_blog.sql
```

4. 运行测试

```rust
cargo run
```

6. 后台登录账号及密码：`admin` / `123456`, 后台登录地址: `/admin/index`


### 特别鸣谢

感谢以下的项目,排名不分先后

 - actix-web

 - sea-orm
 
 - dotenvy

 - tera


### 开源协议

*  `nako-blog` 遵循 `Apache2` 开源协议发布，在保留本系统版权的情况下提供个人及商业免费使用。


### 版权

*  该系统所属版权归 deatil(https://github.com/deatil) 所有。
