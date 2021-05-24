#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>

typedef struct {
  ngx_flag_t enable_calculation;
} ngx_http_calculator_loc_conf_t;

ngx_int_t ngx_http_calculator_handler(ngx_http_request_t *r);

static void *ngx_http_calculator_create_loc_conf(ngx_conf_t *cf);
static char *ngx_http_calculator_merge_loc_conf(ngx_conf_t *cf, void *parent,
                                                void *child);

static ngx_command_t ngx_http_calculator_commands[] = {
  {ngx_string("calculate"), NGX_HTTP_LOC_CONF | NGX_CONF_FLAG,
   ngx_conf_set_flag_slot, NGX_HTTP_LOC_CONF_OFFSET,
   offsetof(ngx_http_calculator_loc_conf_t, enable_calculation), NULL},
  ngx_null_command};

static ngx_http_module_t ngx_http_calculator_module_ctx = {
  NULL, /* preconfiguration */
  NULL, /* postconfiguration */
  NULL, /* create main configuration */
  NULL, /* init main configuration */

  NULL, /* create server configuration */
  NULL, /* merge server configuration */

  ngx_http_calculator_create_loc_conf, /* create location configuration */
  ngx_http_calculator_merge_loc_conf   /* merge location configuration */
};

ngx_module_t ngx_http_calculator = {
  NGX_MODULE_V1,
  &ngx_http_calculator_module_ctx, /* module context */
  ngx_http_calculator_commands,    /* module directives */
  NGX_HTTP_MODULE,                 /* module type */
  NULL,                            /* init master */
  NULL,                            /* init module */
  NULL,                            /* init process */
  NULL,                            /* init thread */
  NULL,                            /* exit thread */
  NULL,                            /* exit process */
  NULL,                            /* exit master */
  NGX_MODULE_V1_PADDING};

static void *ngx_http_calculator_create_loc_conf(ngx_conf_t *cf) {
  ngx_http_calculator_loc_conf_t *conf;

  conf = ngx_pcalloc(cf->pool, sizeof(ngx_http_calculator_loc_conf_t));
  if (conf == NULL) {
    return NULL;
  }

  conf->enable_calculation = NGX_CONF_UNSET;

  return conf;
}

static char *ngx_http_calculator_merge_loc_conf(ngx_conf_t *cf, void *parent,
                                                void *child) {
  ngx_http_calculator_loc_conf_t *prev = parent;
  ngx_http_calculator_loc_conf_t *conf = child;

  ngx_conf_merge_value(conf->enable_calculation, prev->enable_calculation, 0);

  if (conf->enable_calculation) {
    ngx_http_core_loc_conf_t *clcf;

    clcf = ngx_http_conf_get_module_loc_conf(cf, ngx_http_core_module);
    clcf->handler = ngx_http_calculator_handler;
  }

  return NGX_CONF_OK;
}
