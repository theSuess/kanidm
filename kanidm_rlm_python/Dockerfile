FROM opensuse/tumbleweed:latest

EXPOSE 1812 1813

# These all need to be on one line else the rpm cache ends
# up in the layers.
RUN zypper refresh --force && \
    zypper install -y \
    freeradius-client \
    freeradius-server \
    freeradius-server-python3 \
    freeradius-server-utils \
    hostname \
    python310 \
    python310-devel \
    python310-pip \
    timezone \
    iproute2 \
    iputils \
    openssl \
    curl && \
    zypper clean

ADD kanidm_rlm_python/mods-available/ /etc/raddb/mods-available/
COPY kanidm_rlm_python/sites-available/ /etc/raddb/sites-available/

# Set a working directory of /etc/raddb
WORKDIR /etc/raddb

# Enable the python and cache module.
RUN ln -s /etc/raddb/mods-available/python3 /etc/raddb/mods-enabled/python3
RUN ln -s /etc/raddb/sites-available/check-eap-tls /etc/raddb/sites-enabled/check-eap-tls

# disable auth via methods we don't support!
RUN rm /etc/raddb/mods-available/sql
RUN rm /etc/raddb/mods-enabled/{passwd,totp}

# Allows the radiusd user to write to the directory
RUN chown -R radiusd: /etc/raddb
RUN chmod 775 /etc/raddb/certs
RUN chmod 640 /etc/raddb/clients.conf

RUN mkdir -p /pkg/pykanidm/
COPY pykanidm/ /pkg/pykanidm/

# install the package and its dependencies
RUN python3 -m pip install --no-cache-dir --no-warn-script-location /pkg/pykanidm
# clean up after install
RUN rm -rf /pkg/*

USER radiusd
ENV LD_PRELOAD=/usr/lib64/libpython3.so
ENV KANIDM_CONFIG_FILE="/data/kanidm"

COPY kanidm_rlm_python/radius_entrypoint.py /radius_entrypoint.py
CMD [ "/usr/bin/python3", "/radius_entrypoint.py" ]
