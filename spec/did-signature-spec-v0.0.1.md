
File extersion: *.dsig

```
-----BEGIN DID SIGNED MESSAGE-----
text message OR based64 message
-----BEGIN DID SIGNATURE------
Version: 0.0.1
KeyId: did:example:xxxxxxxxxxxxxxxxxxxxxx#key-1
Agent: OpenDID v0.0.0
Hash: SHA256-XXxxXXxx...XXxxXX
Type: Plain/Base64(default Plain)
Comment: comments
- line 2 ....
- line 3 ....

base64 base64 base64 base64 base64 ...
-----END DID SIGNATURE-----
```


Hash format:
```
hash-source       = "'" hash-algo "-" hash-value "'"
hash-algo         = "sha256" / "sha384" / "sha512"
hash-value        = base64-value
```

Currently supports `SHA256` ONLY!

See: https://www.w3.org/TR/CSP2/#hash_value


