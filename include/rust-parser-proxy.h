#ifndef RUST_PARSER_PROXY_H_INCLUDED
#define RUST_PARSER_PROXY_H_INCLUDED

#include "syslog-ng.h"

struct RustParserProxy;

void
rust_parser_proxy_free(struct RustParserProxy* this);

void
rust_parser_proxy_set_option(struct RustParserProxy* self, const gchar* key, const gchar* value);

gboolean
rust_parser_proxy_process(struct RustParserProxy* this, LogMessage *pmsg, const gchar *input, gsize input_len);

int
rust_parser_proxy_init(struct RustParserProxy* s);

struct RustParserProxy*
rust_parser_proxy_new(const gchar* parser_name);

struct RustParserProxy*
rust_parser_proxy_clone(struct RustParserProxy *self);

#endif
