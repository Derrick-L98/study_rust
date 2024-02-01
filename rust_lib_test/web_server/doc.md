1、建立TcpListener
2、读取请求内容
3、编写响应
    1）返回一个响应
    2）返回一个真正的网页
    3）有条件的返回网页

http简单介绍
    1）http请求报文包含三个部分内容：请求行、请求头、请求体
    Method Request-URI HTTP-Version CLRF //请求行：请求方式、协议版本等
    headers CRLF    //请求头：包含若干个属性，格式为“属性名：属性值”服务端获取客户端的信息
    message-body    //请求体：客户端真正要传输服务的内容
    2）http响应报文也有三部分：响应行、响应头、响应体
    响应行：报文协议以及版本、状态码及状态描述
    headers CRLF响应头：有多个属性组成
    message-body响应体：真正响应的内容
1、HTTP、TCP
实现一个服务端，客户端通过浏览器进行请求连接的过程

库：
Struct std::net::TcpListener

查文档：
http://doc.rust-lang.org/std/net/