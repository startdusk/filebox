# TODO: 待添加文档

## 遇到的问题集合

### error: linking with `cc` failed: exit status: 1
算是个官方的BUG吧
解决方式
```bash
rustup toolchain uninstall stable && rustup toolchain install stable
```

