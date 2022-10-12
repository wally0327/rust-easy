# 异步I/O

Tokio 库中的IO操作和std库使用方式一样，只不过采用的是异步的模式

## AsyncRead 和 AsyncWrite

这是两个提供给 byte streams 中数据读取的接口。所有的相关方法必须使用 async 和 .await 进行调用， 
此外 `AsyncRead` 和 `AsyncWrite` 不直接使用，而是使用 `AsyncReadExt` 和 `AsyncWriteExt`.

当 read 函数返回 OK(0) 时，表明相关数据读取完毕。

* `AsyncReadExt::read` provides an async method for reading data into a buffer, returning the number of bytes read.
* `AsyncReadExt::read_to_end` reads all bytes from the stream until EOF.

## tokio::select!

`select` 支持在多个异步计算任务中任意一个任务的结束和返回