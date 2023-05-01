#ifndef PANIC_H
#define PANIC_H

#include <stddef.h>
struct pan;

extern "C" {
void __panic__(const void *context, void (*cb)(const void *, pan *));
void __panic_push__(pan *, const char *, size_t);
void panic(const char *message);
}

#endif // PANIC_H
