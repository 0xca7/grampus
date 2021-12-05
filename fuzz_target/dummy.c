#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "kson.h"

#define INPUT_BUFFER_SIZE 4096

int
main(int argc, char *argv[]) 
{
    int i = 0;
    int fd = -1;
    ssize_t bread = -1;

    int ret = EXIT_SUCCESS;
    uint8_t buffer[INPUT_BUFFER_SIZE] = { 0x00 };

    if(argc != 2)
    {
        printf("usage: ./dummy [filename]");
    }

    fd = open(argv[1], O_RDONLY);
    if(fd == -1)
    {
        printf("error (open): %s\n", strerror(errno));
        ret = EXIT_FAILURE;
        /* if we can't even open, just return with failure */
        return ret;
    }

    bread = read(fd, &buffer[0], INPUT_BUFFER_SIZE);
    if(bread == -1)
    {
        printf("error (read): %s\n", strerror(errno));
        /* even if we can't read, we have to close the file */
        ret = EXIT_FAILURE;
    } 
    else 
    {   /* print the result */
        printf("CHAR:\n");
        for(i = 0; i < bread; i++)
        {
            printf("%c", *(buffer+i));
        }
        printf("\nHEX:\n");
        for(i = 0; i < bread; i++)
        {
            printf("%02x ", *(buffer+i));
        }
        printf("\n");

        kson_t *ks = kson_parse((const char*)buffer);
    }

    if(close(fd) == -1)
    {
        printf("error (close): %s\n", strerror(errno));
        ret = EXIT_FAILURE;
    }

    return ret;
}
