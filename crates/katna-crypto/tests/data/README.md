Test data for `katna-crypto`.

- `smime-bob.p12`: a throwaway self-signed S/MIME certificate and its
  private key (no passphrase) for `Bob Tester <bob@example.org>`, SHA-1
  fingerprint `C32F1A750941DE846C109D32714CC0E0D157F65E`, valid until 2036.
  Made with OpenSSL for the tests only; it protects nothing. Legacy
  PKCS#12 encryption, because gpgsm cannot read OpenSSL 3's default.
