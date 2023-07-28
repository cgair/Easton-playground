#include <sys/socket.h>
#include <string.h>
#include <arpa/inet.h>
#include <unistd.h>

#include <stdio.h>
#include <stdlib.h>

// #define error_handling(msg) \
//     do { perror(msg); exit(EXIT_FAILURE); } while (0)

void error_handling(char* message) {
    fputs(message, stderr);
    fputc('\n', stderr);
    exit(EXIT_FAILURE);
}

int main(int argc, char* argv[])
{
    int serv_sock, clnt_sock;
    socklen_t clnt_addr_size;

    struct sockaddr_in serv_addr, clnt_addr;

    char message[] = "Hello world!";

    if (argc != 2) {
        printf("Usage %s <port>\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    serv_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (serv_sock == -1) 
        error_handling("socket() error.");
    
    memset(&serv_addr, 0, sizeof(serv_addr)); /* Clear structure */
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(atoi(argv[1]));
    serv_addr.sin_addr.s_addr = INADDR_ANY; /* Bind to any available network interface */

    if (bind(serv_sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) == -1)
        error_handling("bind() error");

    if (listen(serv_sock, 5) == -1)
        error_handling("listen() error");

    clnt_addr_size = sizeof(clnt_addr);

    clnt_sock = accept(serv_sock, (struct sockaddr *)&clnt_addr, &clnt_addr_size);
    if (clnt_sock == -1)
        error_handling("accept() error");

    write(clnt_sock, message, sizeof(message));

    close(serv_sock);

    return 0;
}