# Configurations for needed utilities

### VM 1 *(Master)*

**BIND9**
- [/etc/bind/named.conf.options](./vm1/bind9/named.conf.options)
- [/etc/bind/db.cat.com](./vm1/bind9/db.cat.com)
- [/etc/bind/db.zone03.net](./vm1/bind9/db.zone03.net)
- [/etc/bind/db.1.168.192](./vm1/bind9/db.1.168.192)

**System**
- [/etc/resolv.conf](./vm1/resolv.conf)
- [/etc/hosts](./vm1/hosts)

**Exim4**
- [/etc/exim4/conf.d/transport/30_exim4-config_remote_smtp](./vm1/exim/30_exim4-config_remote_smtp)

**Dovecot**
- [/etc/dovecot/conf.d/10-ssl.conf](./vm1/dovecot/10-ssl.conf)

### VM 2 *(Slave)*

**BIND9**
- [/etc/bind/named.conf.options](./vm2/bind9/named.conf.options)

**System**
- [/etc/resolv.conf](./vm2/resolv.conf)
- [/etc/hosts](./vm2/hosts)

**Exim4**
- [/etc/exim4/conf.d/transport/30_exim4-config_remote_smtp](./vm2/exim/30_exim4-config_remote_smtp)

**Dovecot**
- [/etc/dovecot/conf.d/10-ssl.conf](./vm2/dovecot/10-ssl.conf)

### Both

**Exim4**
- [/etc/exim4/conf.d/main/03_exim4-config_tlsoptions](./both/exim/03_exim4-config_tlsoptions)
- [/etc/exim4/conf.d/auth/30_exim4-config_examples](./both/exim/30_exim4-config_examples)
- [/etc/exim4/conf.d/router/200_exim4-config_primary](./both/exim/200_exim4-config_primary)
- [/etc/exim4/conf.d/router/350_exim4-config_virtual_local](./both/exim/350_exim4-config_virtual_local)
- [/etc/exim4/conf.d/transport/120_exim4-config_virtual_users](./both/exim/120_exim4-config_virtual_users)
- [/etc/exim4/conf.d/main/01_exim4-config_listmacrosdefs](./both/exim/01_exim4-config_listmacrosdefs)
- [/etc/exim4/conf.d/main/02_exim4-config_options](./both/exim/02_exim4-config_options)
- [/etc/exim4/conf.d/acl/30_exim4-config_check_dkim](./both/exim/30_exim4-config_check_dkim)
- [/etc/exim4/conf.d/acl/40_exim4-config_check_data](./both/exim/40_exim4-config_check_data)

**Dovecot**
- [/etc/dovecot/conf.d/10-mail.conf](./both/dovecot/10-mail.conf)
- [/etc/dovecot/conf.d/10-auth.conf](./both/dovecot/10-auth.conf)
- [/etc/dovecot/conf.d/auth-passwdfile.conf.ext](./both/dovecot/auth-passwdfile.conf.ext)
