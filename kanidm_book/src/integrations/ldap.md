# LDAP

While many applications can support external authentication and identity services through
Oauth2, not all services can.
Lightweight Directory Access Protocol (LDAP) has been the "lingua franca" of
authentication for many years, with almost every application in the world being
able to search and bind to LDAP. As many organisations still rely on LDAP, Kanidm
can host a read-only LDAP interface for these legacy applications.

{{#template  
    ../templates/kani-warning.md
    imagepath=../images
    title=Warning!
    text=The LDAP server in Kanidm is not a fully RFC-compliant LDAP server. This is intentional, as Kanidm wants to cover the common use cases - simple bind and search.
}}


## What is LDAP

LDAP is a protocol to read data from a directory of information. It is not
a server, but a way to communicate to a server. There are many famous LDAP
implementations such as Active Directory, 389 Directory Server, DSEE,
FreeIPA, and many others. Because it is a standard, applications can use
an LDAP client library to authenticate users to LDAP, given "one account" for
many applications - an IDM just like Kanidm!

## Data Mapping

Kanidm cannot be mapped 100% to LDAP's objects. This is because LDAP
types are simple key-values on objects which are all UTF8 strings (or subsets
thereof) based on validation (matching) rules. Kanidm internally implements complex
data types such as tagging on SSH keys, or multi-value credentials. These can not
be represented in LDAP.

Many of the structures in Kanidm do not correlate closely to LDAP. For example
Kanidm only has a GID number, where LDAP's schemas define both a UID number and a
GID number.

Entries in the database also have a specific name in LDAP, related to their path
in the directory tree. Kanidm is a flat model, so we have to emulate some tree-like
elements, and ignore others.

For this reason, when you search the LDAP interface, Kanidm will make some mapping decisions.

* The Kanidm domain name is used to generate the DN of the suffix.
* The domain\_info object becomes the suffix root.
* All other entries are direct subordinates of the domain\_info for DN purposes.
* Distinguished Names (DNs) are generated from the spn, name, or uuid attribute.
* Bind DNs can be remapped and rewritten, and may not even be a DN during bind.
* The '\*' and '+' operators can not be used in conjuction with attribute lists in searches.

These decisions were made to make the path as simple and effective as possible,
relying more on the Kanidm query and filter system than attempting to generate a tree-like
representation of data. As almost all clients can use filters for entry selection
we don't believe this is a limitation for the consuming applications.

## Security

### TLS

StartTLS is not supported due to security risks. LDAPS is the only secure method
of communicating to any LDAP server. Kanidm, when configured with certificates, will
use them for LDAPS (and will not listen on a plaintext LDAP port).

### Writes

LDAP's structure is too simplistic for writing to the complex entries that Kanidm internally
contains. As a result, writes are rejected for all users via the LDAP interface.

### Access Controls

LDAP only supports password authentication. As LDAP is used heavily in POSIX environments
the LDAP bind for any DN will use its configured posix password.

As the POSIX password is not equivalent in strength to the primary credentials of Kanidm
(which may be multi-factor authentication, MFA), the LDAP bind does not grant
rights to elevated read permissions. All binds have the permissions of "Anonymous"
even if the anonymous account is locked.

The exception is service accounts which can use api-tokens during an LDAP bind for elevated
read permissions.

## Server Configuration

To configure Kanidm to provide LDAP, add the argument to the `server.toml` configuration:

    ldapbindaddress = "127.0.0.1:3636"

You should configure TLS certificates and keys as usual - LDAP will re-use the Web
server TLS material.

## Showing LDAP Entries and Attribute Maps

By default Kanidm is limited in what attributes are generated or remapped into
LDAP entries. However, the server internally contains a map of extended attribute
mappings for application specific requests that must be satisfied.

An example is that some applications expect and require a 'CN' value, even though Kanidm does not
provide it. If the application is unable to be configured to accept "name" it may be necessary
to use Kanidm's mapping feature. Currently these are compiled into the server, so you may need to open
an issue with your requirements for attribute maps.

To show what attribute maps exists for an entry you can use the attribute search term '+'.

    # To show Kanidm attributes
    ldapsearch ... -x '(name=admin)' '*'
    # To show all attribute maps
    ldapsearch ... -x '(name=admin)' '+'

Attributes that are in the map can be requested explicitly, and this can be combined with requesting
Kanidm native attributes.

    ldapsearch ... -x '(name=admin)' cn objectClass displayname memberof

## Service Accounts

If you have [issued api tokens for a service account](../accounts_and_groups.html#using-api-tokens-with-service-accounts)
they can be used to gain extended read permissions for those service accounts.

Api tokens can also be used to gain extended search permissions with LDAP. To do this you can bind
with a dn of `dn=token` and provide the api token in the password.

> **NOTE** The `dn=token` keyword is guaranteed to not be used by any other entry, which is why it was chosen as the keyword to initiate api token binds.

```shell
ldapwhoami -H ldaps://URL -x -D "dn=token" -w "TOKEN"
ldapwhoami -H ldaps://idm.example.com -x -D "dn=token" -w "..."
# u: demo_service@idm.example.com
```

## Example

Given a default install with domain "example.com" the configured LDAP DN will be "dc=example,dc=com".

    # from server.toml
    ldapbindaddress = "[::]:3636"

This can be queried with:

    LDAPTLS_CACERT=ca.pem ldapsearch \
        -H ldaps://127.0.0.1:3636 \
        -b 'dc=example,dc=com' \
        -x '(name=test1)'

    # test1@example.com, example.com
    dn: spn=test1@example.com,dc=example,dc=com
    objectclass: account
    objectclass: memberof
    objectclass: object
    objectclass: person
    displayname: Test User
    memberof: spn=group240@example.com,dc=example,dc=com
    name: test1
    spn: test1@example.com
    entryuuid: 22a65b6c-80c8-4e1a-9b76-3f3afdff8400

It is recommended that client applications filter accounts that can login with `(class=account)`
and groups with `(class=group)`. If possible, group membership is defined in RFC2307bis or
Active Directory style. This means groups are determined from the "memberof" attribute which
contains a DN to a group.

LDAP binds can use any unique identifier of the account. The following are all valid bind DNs for
the object listed above (if it was a POSIX account, that is).

    ldapwhoami ... -x -D 'name=test1'
    ldapwhoami ... -x -D 'spn=test1@example.com'
    ldapwhoami ... -x -D 'test1@example.com'
    ldapwhoami ... -x -D 'test1'
    ldapwhoami ... -x -D '22a65b6c-80c8-4e1a-9b76-3f3afdff8400'
    ldapwhoami ... -x -D 'spn=test1@example.com,dc=example,dc=com'
    ldapwhoami ... -x -D 'name=test1,dc=example,dc=com'

Most LDAP clients are very picky about TLS, and can be very hard to debug or display errors.
For example these commands:

    ldapsearch -H ldaps://127.0.0.1:3636 -b 'dc=example,dc=com' -x '(name=test1)'
    ldapsearch -H ldap://127.0.0.1:3636 -b 'dc=example,dc=com' -x '(name=test1)'
    ldapsearch -H ldap://127.0.0.1:3389 -b 'dc=example,dc=com' -x '(name=test1)'

All give the same error:

    ldap_sasl_bind(SIMPLE): Can't contact LDAP server (-1)

This is despite the fact:

* The first command is a certificate validation error.
* The second is a missing LDAPS on a TLS port.
* The third is an incorrect port.

To diagnose errors like this, you may need to add "-d 1" to your LDAP commands or client.

