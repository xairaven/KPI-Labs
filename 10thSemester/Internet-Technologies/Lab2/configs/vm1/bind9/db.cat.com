;
; BIND data file for local loopback interface
;
$TTL	604800
@	IN	SOA	ns1.cat.com. admin.cat.com. (
			8	; Serial
			604800	; Refresh (Slave - Refresh DB after ...)
			86400	; Retry (Slave - Retry query after x if not succeed)
			2419200	; Expire (Slave consider information non-valid)
			604800	; Negative Cache TTL	

);

@	IN	NS	ns1.cat.com.
@	IN	NS	ns2.cat.com.

@	IN	MX	10 mail.cat.com.
@	IN	TXT	"v=spf1 a mx -all"
mail	IN	A	192.168.1.2

ns1	IN	A	192.168.1.2
ns2	IN	A	192.168.1.3

tiger	IN	A	192.168.1.11
lion	IN	A	192.168.1.12
lynx	IN	A	192.168.1.13
leopard	IN	A	192.168.1.14
jaguar	IN	A	192.168.1.15

www 	IN	CNAME	tiger
ftp	IN	CNAME	lion
ssh	IN	CNAME	lynx
pop3	IN	CNAME	leopard
imap	IN	CNAME	jaguar

dkim._domainkey	IN	TXT	"v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC871RfQaDUgbmOuJ1lOgvHiUXVIXYCvl9qyx8vuKCbK+G9vJ8PCY13P33+ldaMEdf8h3CiSmfVlolgPbsxJtca9GcvgAs8LjVN7kH6//PuAPYdT1cWiyiXS186nL4sVW7ndwTTuDjSiXfRCB6Pdu2xl+ZmgBl4hXso8yVjI96ZbwIDAQAB"
