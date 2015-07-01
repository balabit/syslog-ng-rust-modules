#ifndef RUST_FILTER_PROXY_H_INCLUDED
#define RUST_FILTER_PROXY_H_INCLUDED

#include "syslog-ng.h"

struct RustFilterProxy;

void
rust_filter_proxy_free(struct RustFilterProxy* this);

void
rust_filter_proxy_set_option(struct RustFilterProxy* this, const gchar* key, const gchar* value);

gboolean
rust_filter_proxy_eval(struct RustFilterProxy* this, LogMessage* msg);

void
rust_filter_proxy_init(struct RustFilterProxy* s, const GlobalConfig *cfg);

struct RustFilterProxy*
rust_filter_proxy_new(const gchar* filter_name);


#endif
