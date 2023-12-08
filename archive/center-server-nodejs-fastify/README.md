# 问题

- prisma还是比较拉跨
    - 只能保存utc时间，这个不能忍
    - 打印sql语句也很麻烦，不好跟http框架的logger兼容，这个似乎是orm的通病
    - 综上，决定不再用prisma了，自己写原生sql得了
