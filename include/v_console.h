#ifndef V_CONSOLE_H
#define V_CONSOLE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

void v_init();
void v_print(const char* str);
void v_print_char(char c);
void v_read_line(char* buffer, size_t max_len);

#ifdef __cplusplus
}
#endif

#endif
