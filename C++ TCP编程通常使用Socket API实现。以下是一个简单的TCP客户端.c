C++ TCP编程通常使用Socket API实现。以下是一个简单的TCP客户端与服务端的示例代码：

客户端代码：

```
#include <iostream>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>

using namespace std;

int main(int argc, char const *argv[]) {
    // 创建socket
    int sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (sock == -1) {
        cout << "Error: Create socket failed." << endl;
        return -1;
    }

    // 服务端地址信息
    struct sockaddr_in serverAddr;
    memset(&serverAddr, 0, sizeof(serverAddr));
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(8888);
    serverAddr.sin_addr.s_addr = inet_addr("127.0.0.1");

    // 连接服务端
    if (connect(sock, (struct sockaddr *)&serverAddr, sizeof(serverAddr)) == -1) {
        cout << "Error: Connect to server failed." << endl;
        close(sock);
        return -1;
    }

    // 发送数据
    char sendBuffer[1024];
    memset(sendBuffer, 0, sizeof(sendBuffer));
    strcpy(sendBuffer, "Hello server!");
    send(sock, sendBuffer, strlen(sendBuffer), 0);

    // 接收数据
    char recvBuffer[1024];
    memset(recvBuffer, 0, sizeof(recvBuffer));
    int recvLen = recv(sock, recvBuffer, sizeof(recvBuffer), 0);
    if (recvLen > 0) {
        cout << "Received message from server: " << recvBuffer << endl;
    } else {
        cout << "Error: Receive data failed." << endl;
    }

    // 关闭socket
    close(sock);

    return 0;
}
```

服务端代码：

```
#include <iostream>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>

using namespace std;

int main(int argc, char const *argv[]) {
    // 创建socket
    int listenSock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (listenSock == -1) {
        cout << "Error: Create socket failed." << endl;
        return -1;
    }

    // 绑定地址信息
    struct sockaddr_in serverAddr;
    memset(&serverAddr, 0, sizeof(serverAddr));
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(8888);
    serverAddr.sin_addr.s_addr = htonl(INADDR_ANY);

    if (bind(listenSock, (struct sockaddr *)&serverAddr, sizeof(serverAddr)) == -1) {
        cout << "Error: Bind address failed." << endl;
        close(listenSock);
        return -1;
    }

    // 监听端口
    if (listen(listenSock, SOMAXCONN) == -1) {
        cout << "Error: Listen port failed." << endl;
        close(listenSock);
        return -1;
    }

    while (true) {
        // 接受连接请求
        struct sockaddr_in clientAddr;
        socklen_t clientLen = sizeof(clientAddr);
        int clientSock = accept(listenSock, (struct sockaddr *)&clientAddr, &clientLen);
        if (clientSock == -1) {
            cout << "Error: Accept connect failed." << endl;
            continue;
        }

        // 接收数据
        char buffer[1024];
        memset(buffer, 0, sizeof(buffer));
        int recvLen = recv(clientSock, buffer, sizeof(buffer), 0);
        if (recvLen <= 0) {
            cout << "Error: Receive data failed." << endl;
            close(clientSock);
            continue;
        }

        // 处理数据
        cout << "Received message from client: " << buffer << endl;

        // 发送数据
        char sendBuffer[1024];
        memset(sendBuffer, 0, sizeof(sendBuffer));
        strcpy(sendBuffer, "Hello client!");
        send(clientSock, sendBuffer, strlen(sendBuffer), 0);

        // 关闭socket
        close(clientSock);
    }

    // 关闭socket
    close(listenSock);

    return 0;
}
```

以上代码实现了TCP客户端与服务端的基本交互，但如果数据包过大或者发送频繁，可能会出现TCP粘包问题。解决TCP粘包问题有多种方法，其中一种常用方法是在数据包前加上数据包长度，接收方先读取数据包长度再按照指定长度读取数据。修改后的代码如下：

客户端代码：

```
#include <iostream>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>

using namespace std;

int main(int argc, char const *argv[]) {
    // 创建socket
    int sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (sock == -1) {
        cout << "Error: Create socket failed." << endl;
        return -1;
    }

    // 服务端地址信息
    struct sockaddr_in serverAddr;
    memset(&serverAddr, 0, sizeof(serverAddr));
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(8888);
    serverAddr.sin_addr.s_addr = inet_addr("127.0.0.1");

    // 连接服务端
    if (connect(sock, (struct sockaddr *)&serverAddr, sizeof(serverAddr)) == -1) {
        cout << "Error: Connect to server failed." << endl;
        close(sock);
        return -1;
    }

    // 发送数据
    char sendBuffer[1024];
    memset(sendBuffer, 0, sizeof(sendBuffer));
    strcpy(sendBuffer, "Hello server!");
    
    int sendLen = strlen(sendBuffer);
    
    // 发送数据长度
    send(sock, &sendLen, sizeof(int), 0);
    
    // 发送数据
    send(sock, sendBuffer, strlen(sendBuffer), 0);

    // 接收数据
    char recvBuffer[1024];
    memset(recvBuffer, 0, sizeof(recvBuffer));
    
    // 先接收数据长度
    int recvLen = 0;
    
    if(recv(sock,&recvLen,sizeof(int),0) > 0){
        
        recv(sock, recvBuffer, recvLen, 0);
        
        if (recvLen > 0) {
            cout << "Received message from server: " << recvBuffer << endl;
        } else {
            cout << "Error: Receive data failed." << endl;
        }
        
    }else{
        
         cout << "Error: Receive data length failed." << endl; 
        
    }

   

    // 关闭socket
    close(sock);

    return 0;
}
```

服务端代码：

```
#include <iostream>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>

using namespace std;

int main(int argc, char const *argv[]) {
    
     // 创建socket
     int listenSock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
     if (listenSock == -1) {
         cout << "Error: Create socket failed." << endl;
         return -1;
     }
 
     // 绑定地址信息
     struct sockaddr_in serverAddr;
     memset(&serverAddr, 0, sizeof(serverAddr));
     serverAddr.sin_family = AF_INET;
     serverAddr.sin_port = htons(8888);
     serverAddr.sin_addr.s_addr = htonl(INADDR_ANY);
 
     if (bind(listenSock, (struct sockaddr *)&serverAddr, sizeof(serverAddr)) == -1) {
         cout << "Error: Bind address failed." << endl;
         close(listenSock);
         return -1;
     }
 
     // 监听端口
     if (listen(listenSock, SOMAXCONN) == -1) {
         cout << "Error: Listen port failed." << endl;
         close(listenSock);
         return -1;
     }
 
     while (true) {
         // 接受连接请求
         struct sockaddr_in clientAddr;
         socklen_t clientLen = sizeof(clientAddr);
         int clientSock = accept(listenSock, (struct sockaddr *)&clientAddr, &clientLen);
         if (clientSock == -1) {
             cout << "Error: Accept connect failed." << endl;
             continue;
         }
 
         // 接收数据长度
         int len = 0;
         recv(clientSock,&len,sizeof(int),0);
         
         // 接收数据
         char buffer[1024];
         memset(buffer, 0, sizeof(buffer));
         
         if(len > 0){
             
             recv(clientSock, buffer,len , 0);
             
             // 处理数据
             cout << "Received message from client: " << buffer << endl;

             // 发送数据长度
             int sendLen = strlen("Hello client!");
             send(clientSock,&sendLen,sizeof(int),0);
             
             // 发送数据
             char sendBuffer[1024];
             memset(sendBuffer, 0, sizeof(sendBuffer));
             strcpy(sendBuffer, "Hello client!");
             send(clientSock, sendBuffer,strlen(sendBuffer), 0);
             
         }else{
             
             cout<<"Error: Receive data length failed."<<endl;  
             
         }
         
        
         // 关闭socket
         close(clientSock);
     }
 
     // 关闭socket
     close(listenSock);
 
     return 0;

}
```

通过在发送数据时添加数据包长度，接收方先读取数据包长度再读取指定长度的数据，可以有效解决TCP粘包问题。

