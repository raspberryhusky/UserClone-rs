# UserClone-rs
*** 

## 原理
<p>复制目标帐户对应注册表项F键的值，使得目标帐户获得了相同的权限和系统上下文</p>


## 应用场景
<p>用于权限维持或者因为某种原因无法登录进目标账户，可配合影子账户使用</p>


![UserClone](/images/demo.gif "UserClone-rs")

```
//将user1的用户上下文克隆到user2中
UserClone-rs.exe user1 user2
```

## 注意
* M1，懒的编译了，需要的自己编译吧
* 懒人工具，不打码，代码直接放上边，想咋改就咋改
* 遇到无法正常运行的panic大多应该是运行权限的问题
* 仅在win11上进行了测试，别的版本应该问题不大，主要是运行权限
