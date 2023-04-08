# Walle-Q

![OneBot12](https://img.shields.io/badge/OneBot-12-black?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABwCAMAAADxPgR5AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAAxQTFRF////29vbr6+vAAAAk1hCcwAAAAR0Uk5T////AEAqqfQAAAKcSURBVHja7NrbctswDATQXfD//zlpO7FlmwAWIOnOtNaTM5JwDMa8E+PNFz7g3waJ24fviyDPgfhz8fHP39cBcBL9KoJbQUxjA2iYqHL3FAnvzhL4GtVNUcoSZe6eSHizBcK5LL7dBr2AUZlev1ARRHCljzRALIEog6H3U6bCIyqIZdAT0eBuJYaGiJaHSjmkYIZd+qSGWAQnIaz2OArVnX6vrItQvbhZJtVGB5qX9wKqCMkb9W7aexfCO/rwQRBzsDIsYx4AOz0nhAtWu7bqkEQBO0Pr+Ftjt5fFCUEbm0Sbgdu8WSgJ5NgH2iu46R/o1UcBXJsFusWF/QUaz3RwJMEgngfaGGdSxJkE/Yg4lOBryBiMwvAhZrVMUUvwqU7F05b5WLaUIN4M4hRocQQRnEedgsn7TZB3UCpRrIJwQfqvGwsg18EnI2uSVNC8t+0QmMXogvbPg/xk+Mnw/6kW/rraUlvqgmFreAA09xW5t0AFlHrQZ3CsgvZm0FbHNKyBmheBKIF2cCA8A600aHPmFtRB1XvMsJAiza7LpPog0UJwccKdzw8rdf8MyN2ePYF896LC5hTzdZqxb6VNXInaupARLDNBWgI8spq4T0Qb5H4vWfPmHo8OyB1ito+AysNNz0oglj1U955sjUN9d41LnrX2D/u7eRwxyOaOpfyevCWbTgDEoilsOnu7zsKhjRCsnD/QzhdkYLBLXjiK4f3UWmcx2M7PO21CKVTH84638NTplt6JIQH0ZwCNuiWAfvuLhdrcOYPVO9eW3A67l7hZtgaY9GZo9AFc6cryjoeFBIWeU+npnk/nLE0OxCHL1eQsc1IciehjpJv5mqCsjeopaH6r15/MrxNnVhu7tmcslay2gO2Z1QfcfX0JMACG41/u0RrI9QAAAABJRU5ErkJggg==)
![version](https://img.shields.io/github/v/tag/onebot-walle/walle-q.svg)
![license](https://img.shields.io/github/license/onebot-walle/walle-q.svg)

> Walle Mk.Q

一个 QQ 平台的 OneBot 协议实现端

A qq platform OneBot Implementation

本项目使用 [ricq](https://github.com/lz1998/ricq) 协议库与 [Walle-core](https://GitHub.com/abrahum/walle-core) LibOnebot 构建。

在线文档地址：[Walle-Mk.Q 使用手册](https://walle-q.1bot.dev)

> 本项目采用 AGPLv3 开源协议，仅出于学习目的开发，不鼓励、不支持任何除此以外的任何其他用途。

## 登录方式

- [x] 账户密码登录
- [x] 扫码登录
- [x] Token 登录

## 已支持事件 Event

### 消息事件 message

- [x] 私聊消息 message.private
- [x] 群临时消息 message.group_temp
- [x] 群消息 message.group

### 通知消息 notice

- [x] 私聊消息撤回 notice.private_message_delete
- [x] 好友增加 notice.friend_increase
- [x] 好友减少 notice.friend_decrease
- [x] 好友戳一戳 notice.friend_poke
- [x] 群成员增加 notice.group_member_increase
- [x] 群成员减少 notice.group_member_decrease
- [x] 群成员禁言 notice.group_member_ban
- [x] 群消息撤回 notice.group_message_delete
- [x] 群管理员设置 notice.group_admin_set
- [x] 群管理取消设置 notice.group_admin_unset
- [x] 群名称更新 notice.group_name_update

### 请求事件 request

- [x] 好友添加请求 request.new_friend
- [x] 新成员加群申请 request.join_group
- [x] 群邀请 request.group_invited

## 已支持消息段

### 接收与发送

- [x] text 消息
- [x] at 消息
- [x] face 消息
- [x] image 消息
- [x] reply 消息
- [x] xml 消息
- [x] voice 消息（单独使用）

### 仅接收

- [x] dice 消息
- [x] rps 消息
- [x] json 消息

### 仅发送

- [x] forward(node) 消息（单独使用）

> 消息段优先级 others > node(forward) > voice

## 已支持 API

### 元动作

- [x] 获取近期事件 get_latest_events
- [x] 获取支持的动作列表 get_supported_actions
- [x] 获取运行状态 get_status
- [x] 获取版本信息 get_version
- [x] * 关闭应用 shutdown
- [x] * 登录账号 login
- [x] * 提交登录信息 submit_login
- [x] * 登出账号 logout

### 消息动作

- [x] 发送消息 send_message
- [x] 删除消息 delete_message
- [x] 获取消息 get_message

### 单用户动作

- [x] 获取机器人自身信息 get_self_info
- [x] 获取用户信息 get_user_info
- [x] 获取好友列表 get_friend_list
- [x] 处理好友请求 set_new_friend
- [x] 删除好友 delete_friend
- [x] 获取好友请求列表 get_new_friend_requests

### 单级群组动作

- [x] 获取群信息 get_group_info
- [x] 获取群列表 get_group_list
- [x] 获取群成员信息 get_group_member_info
- [x] 获取群成员列表 get_group_member_list
- [x] 设置群名称 set_group_name
- [x] 退出群 leave_group
- [x] 踢出群成员 kick_group_member
- [x] 禁言群成员 ban_group_member
- [x] 解禁群成员 unban_group_member
- [x] 设置群管理员 set_group_admin
- [x] 取消群管理员 unset_group_admin
- [x] 处理加群请求 set_join_group
- [x] 获取加群申请 get_join_group_requests
- [x] 处理群邀请 set_group_invited
- [x] 获取群邀请 get_group_inviteds

### 文件动作

- [x] 上传文件 upload_file
- [x] 获取文件 get_file
- [x] 分片上传文件 upload_file_fragmented
- [x] 分片获取文件 get_file_fragmented

## OneBot-v11 协议支持

基本功能已支持

**v0.1.4 起不再支持v11协议，恢复支持时间未定，咕咕咕**

## 已知问题

- 群管理设置 `operator_id` 缺失
- 新成员入群 `operator_id` 缺失

## 相关项目

- [nonebot-walleq-extension](https://github.com/obwalle/nonebot-walleq-extension)：Nonebot2 OneBot-v12 协议适配器扩展