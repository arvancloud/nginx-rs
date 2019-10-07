NGX_OPTS = \
	--with-compat --with-threads --with-http_addition_module --with-http_v2_module \
	--with-http_auth_request_module --with-http_gunzip_module --with-http_gzip_static_module \
	--with-http_random_index_module --with-http_realip_module --with-http_secure_link_module \
	--with-http_slice_module --with-http_stub_status_module --with-http_sub_module \
	--with-stream --with-stream_realip_module --with-stream_ssl_preread_module \
	--with-file-aio --with-http_ssl_module --with-stream_ssl_module \
	--with-cc-opt='-g -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC' \
	--with-ld-opt='-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'

prepare-nginx:
	curl -o $(OUT_DIR)/nginx.tar.gz http://nginx.org/download/nginx-$(NGINX_VERSION).tar.gz
	mkdir -p $(OUT_DIR)/nginx
	tar -C $(OUT_DIR)/nginx -xzf $(OUT_DIR)/nginx.tar.gz --strip-components 1
	rm $(OUT_DIR)/nginx.tar.gz
	cd $(OUT_DIR)/nginx && ./configure $(NGX_OPTS)

prepare-nginx-local:
	cd $(NGINX_PATH) && auto/configure $(NGX_OPTS)

build-image:
	docker build build-utils -t nginx:builder

build:
	docker run -v ${PWD}:/nginx-rs --rm nginx:builder
